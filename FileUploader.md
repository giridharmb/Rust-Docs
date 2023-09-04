## File Uploader : Server and Client

#### Server

`Cargo.toml`

```toml
[package]
name = "f-upload-server"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
actix-multipart = "0.6.1"
actix-rt = "2.9.0"
actix-web = "4.4.0"
bytes = "1.4.0"
env_logger = "0.10.0"
futures = "0.3.28"
log = "0.4.20"
sanitize-filename = "0.5.0"
serde = { version = "1.0.188", features = ["derive"] }
serde_json = "1.0.105"
tokio = { version = "1.32.0", features = ["full"] }
warp = "0.3.5"
```

`main.rs`

```rust
// server.rs

use std::convert::Infallible;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use warp::{Filter, reply};
use bytes::Bytes;
use warp::http::HeaderMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;
use env_logger;
use env_logger::Env;
use log::{info, warn, debug};
use sanitize_filename;

#[derive(Serialize)]
struct Response {
    message: String,
}
async fn save_file(headers: HeaderMap, bytes: Bytes) -> Result<impl warp::Reply, Infallible> {
    let epoch_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let epoch_time_str = epoch_time.to_string();
    let default_filename = format!("file_{}",epoch_time_str);

    if let Some(filename) = headers.get("X-Filename") {
        let filename_str = filename.to_str().unwrap_or(default_filename.as_str());
        let sanitized_file = sanitize_filename::sanitize(filename_str);
        let complete_file_path = format!("./uploads/{}", sanitized_file);
        let mut file = File::create(complete_file_path).await.unwrap();
        file.write_all(&bytes).await.unwrap();
        Ok(format!("File ({}) uploaded successfully", filename_str))
    } else {
        Ok("Missing X-Filename header".to_string())
    }
}


#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    std::fs::create_dir_all("./uploads").unwrap();
    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::header::headers_cloned())
        .and(warp::body::bytes())
        .and_then(save_file);

    println!("Server started at http://127.0.0.1:3030/upload");
    warp::serve(upload_route).run(([0, 0, 0, 0], 3030)).await;
}
```

Build

```bash
cargo build --release
```

Start The Server

```bash
target/release/f-upload-server
```

#### Client (Which Will Upload The File)

`Cargo.toml`

```toml
[package]
name = "f-upload-client"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
actix-multipart = "0.6.1"
actix-rt = "2.9.0"
actix-web = "4.4.0"
clap = { version = "4.4.2", features = [ "derive" ] }
env_logger = "0.10.0"
futures = "0.3.28"
log = "0.4.20"
reqwest = { version = "0.11.20", features = ["json"] }
sanitize-filename = "0.5.0"
tokio = { version = "1.32.0", features = ["full"] }
tokio-util = { version = "0.7.8", features = ["codec"] }
```

`main.rs`

```rust
// client.rs

use std::fs::File;
use std::io::Read;
use tokio;
use clap::Parser;
use clap::Command;
use env_logger;
use env_logger::Env;
use log::{info, warn, debug};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// name of the file to upload
    #[arg(short, long)]
    file_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let args = Args::parse();

    println!("File to upload : {}", args.file_name);

    let file_name = args.file_name.as_str();
    let mut file = File::open(file_name)?;
    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents)?;

    let client = reqwest::Client::new();
    let request_url = "http://127.0.0.1:3030/upload";

    let response = client
        .post(request_url)
        .body(file_contents)
        .header("X-Filename", file_name)
        .send()
        .await?;

    if response.status().is_success() {
        println!("File uploaded successfully");
    } else {
        println!("File upload failed: {:?}", response.status());
    }

    Ok(())
}
```

Build

```bash
cargo build --release
```

Generate A Random Binary File

```bash
dd if=/dev/urandom of=random_file.bin bs=1204 count=100000
```

Upload The File To The Server

```bash
target/release/f-upload-client -f random_file.bin
```

