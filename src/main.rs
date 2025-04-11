use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
mod cmds;
mod config;
mod s3;
mod utils;
use config::Config;
use s3::S3Client;
use utils::size::parse_human_size;

#[derive(Parser)]
#[command(
    author = "D4n13l3k00",
    version,
    about = "A command-line tool for interacting with S3-compatible storage",
    long_about = "S3mgr is a command-line tool that provides a simple interface for interacting with S3-compatible storage services. It supports basic operations like listing, moving, copying, and removing files, as well as uploading and downloading content."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List files in the S3 bucket
    Ls {
        /// Optional path prefix to filter files
        path: Option<PathBuf>,
    },
    /// Create a new directory
    Md {
        /// Path of directory to create
        path: PathBuf,
    },
    /// Move a file from source to destination
    Mv {
        /// Source path in S3
        source: PathBuf,
        /// Destination path in S3
        destination: PathBuf,
    },
    /// Copy a file from source to destination
    Cp {
        /// Source path in S3
        source: PathBuf,
        /// Destination path in S3
        destination: PathBuf,
    },
    /// Display the contents of a file
    Cat {
        /// Path of the file to display
        path: PathBuf,
    },
    /// Remove a file or directory
    Rm {
        /// Path to remove
        path: PathBuf,
        /// Remove recursively if path is a directory
        #[arg(short, long)]
        recursive: bool,
    },
    /// Upload a local file to S3
    Up {
        /// Local file path to upload
        path: PathBuf,
        /// Destination path in S3 (optional)
        #[arg(short, long)]
        destination: Option<String>,
        /// Upload directories recursively
        #[arg(short, long)]
        recursive: bool,
        /// Chunk size for uploading files (e.g., 5M, 1G, 512K, default: 5MB)
        #[arg(short = 'c', long = "chunk-size", value_parser = parse_human_size)]
        chunk_size: Option<usize>,
    },
    /// Download a file from S3
    Dl {
        /// Source path in S3
        source: String,
        /// Local destination path (optional, defaults to current directory)
        #[arg(default_value = ".")]
        destination: PathBuf,
        /// Download directories recursively
        #[arg(short, long)]
        recursive: bool,
        /// Chunk size for downloading files (e.g., 5M, 1G, 512K, default: 5MB)
        #[arg(short = 'c', long = "chunk-size", value_parser = parse_human_size)]
        chunk_size: Option<usize>,
    },
    /// Configure S3 credentials and settings
    Config {
        /// AWS access key ID
        #[arg(short = 'a', long)]
        access_key: Option<String>,
        /// AWS secret access key
        #[arg(short = 's', long)]
        secret_key: Option<String>,
        /// AWS region (e.g., us-east-1)
        #[arg(short = 'r', long)]
        region: Option<String>,
        /// S3 bucket name
        #[arg(short = 'b', long)]
        bucket: Option<String>,
        /// Custom S3 endpoint URL (for non-AWS S3-compatible services)
        #[arg(short = 'e', long)]
        endpoint: Option<String>,
        /// Default chunk size for uploading files (e.g., 5M, 1G, 512K)
        #[arg(long = "upload-chunk-size", value_parser = parse_human_size)]
        upload_chunk_size: Option<usize>,
        /// Default chunk size for downloading files (e.g., 5M, 1G, 512K)
        #[arg(long = "download-chunk-size", value_parser = parse_human_size)]
        download_chunk_size: Option<usize>,
        /// View current configuration
        #[arg(short = 'v', long = "view")]
        view: bool,
        /// Show all sensitive information including secret keys
        #[arg(long = "all")]
        show_all: bool,
        /// Reset configuration to default values
        #[arg(long = "reset")]
        reset: bool,
    },
}

async fn handle_s3_command(command: &Commands, s3_client: &S3Client) -> Result<()> {
    match command {
        Commands::Ls { path } => cmds::ls::execute(path.clone(), s3_client).await,
        Commands::Md { path } => cmds::md::execute(path.clone(), s3_client).await,
        Commands::Mv {
            source,
            destination,
        } => cmds::mv::execute(source.clone(), destination.clone(), s3_client).await,
        Commands::Cp {
            source,
            destination,
        } => cmds::cp::execute(source.clone(), destination.clone(), s3_client).await,
        Commands::Cat { path } => cmds::cat::execute(path.clone(), s3_client).await,
        Commands::Rm { path, recursive } => {
            cmds::rm::execute(path.clone(), *recursive, s3_client).await
        }
        Commands::Up {
            path,
            destination,
            recursive,
            chunk_size,
        } => {
            cmds::up::execute(
                path.clone(),
                destination.clone(),
                *recursive,
                *chunk_size,
                s3_client,
            )
            .await
        }
        Commands::Dl {
            source,
            destination,
            recursive,
            chunk_size,
        } => {
            cmds::dl::execute(
                source.clone(),
                destination.clone(),
                *recursive,
                *chunk_size,
                s3_client,
            )
            .await
        }
        Commands::Config { .. } => unreachable!(),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Config {
            access_key,
            secret_key,
            region,
            bucket,
            endpoint,
            upload_chunk_size,
            download_chunk_size,
            view,
            show_all,
            reset,
        } => {
            cmds::config::execute(
                access_key.clone(),
                secret_key.clone(),
                region.clone(),
                bucket.clone(),
                endpoint.clone(),
                *upload_chunk_size,
                *download_chunk_size,
                *view,
                *show_all,
                *reset,
            )?;
            return Ok(());
        }
        _ => {
            let config = Config::load()?;
            let s3_client = S3Client::new(&config.s3)?;
            handle_s3_command(&cli.command, &s3_client).await?;
        }
    }

    Ok(())
}
