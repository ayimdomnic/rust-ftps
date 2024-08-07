# Rust FTPS

A Rust library for interacting with FTPS servers, providing a secure and efficient way to handle file transfers over FTP with SSL/TLS support.

## Features

- Secure file transfer using FTPS (FTP over SSL/TLS)
- Simple and intuitive API for common FTPS operations
- Support for both explicit and implicit FTPS modes
- Comprehensive error handling and logging

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
rust-ftps = "0.1"
```

Replace `"0.1"` with the latest version number.

## Usage

Here's a basic example of how to use `rust-ftps`:

```rust
use rust_ftps::FTPSClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FTPSClient::new("ftp.example.com", 21)?;
    client.login("username", "password")?;
    
    client.change_directory("/path/to/directory")?;
    let files = client.list_files()?;
    
    for file in files {
        println!("Found file: {}", file);
    }
    
    client.quit()?;
    Ok(())
}
```

## Documentation

For more detailed usage and API documentation, please refer to the [docs.rs](https://docs.rs/rust-ftps) page.

## Contributing

Contributions are welcome! Please check out our [contributing guidelines](CONTRIBUTING.md) for details on how to get started.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For any questions or support, please reach out to [ayimdomnic@outlook.com](mailto:ayimdomnic@outlook.com).
