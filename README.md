# S3mgr 🚀

A powerful command-line tool for interacting with S3-compatible storage services, written in Rust.

### ✨ Features

- 📂 List files in S3 buckets
- 📄 Cat files
- 📁 Create directories
- ✂️ Move and copy files
- 🗑️ Remove files and directories (with recursive option)
- ⬆️ Upload files and directories
- ⬇️ Download files
- 🔧 Configurable chunk size for uploads/downloads
- 🌐 Support for custom S3-compatible endpoints
- 🔐 Secure credential management

### 📦 Installation

```bash
cargo install --git https://github.com/D4n13l3k00/s3mgr.git
```


### ⚙️ Configuration

Before using S3mgr, you need to configure your S3 credentials:

```bash
s3mgr config -a YOUR_ACCESS_KEY -s YOUR_SECRET_KEY -r YOUR_REGION -b YOUR_BUCKET
```

For custom S3-compatible services (like MinIO, Selectel, etc.), you can specify an endpoint:

```bash
s3mgr config -e https://your-endpoint.com # ...other options
```

### 📖 Usage

#### 📋 List files
<details>
<summary>Click to expand</summary>

```bash
s3mgr ls [path]
```
</details>

#### 📄 Cat file
<details>
<summary>Click to expand</summary>

```bash
s3mgr cat <path>
```
</details>

#### 📁 Create directory
<details>
<summary>Click to expand</summary>

```bash
s3mgr md <path>
```
</details>

#### ✂️ Move file
<details>
<summary>Click to expand</summary>

```bash
s3mgr mv <source> <destination>
```
</details>

#### 📋 Copy file
<details>
<summary>Click to expand</summary>

```bash
s3mgr cp <source> <destination>
```
</details>

#### 🗑️ Remove file/directory
<details>
<summary>Click to expand</summary>

```bash
s3mgr rm <path> [-r]  # -r for recursive removal
```
</details>

#### ⬆️ Upload file/directory
<details>
<summary>Click to expand</summary>

```bash
s3mgr up <path> [-d destination] [-r] [-c chunk-size]
```
</details>

#### ⬇️ Download file/directory
<details>
<summary>Click to expand</summary>

```bash
s3mgr dl source <path> [-r] [-d destination] [-c chunk-size] 
```
</details>

### 💡 Examples

Upload a file with custom chunk size:
```bash
s3mgr up large_file.dat -c 10M
```

Download a directory recursively:
```bash
s3mgr dl my-folder/ -d ./downloads -r
```

## 📜 License

This project is licensed under the AGPL-3.0-or-later License. See the [LICENSE](LICENSE) file for details.

## 📝 Author

This project is developed by [D4n13l3k00](https://github.com/D4n13l3k00).


