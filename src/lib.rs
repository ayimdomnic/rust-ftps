use async_native_tls::TlsConnector;
use async_std::{
    io::{ReadExt, WriteExt},
    net::TcpStream,
};

mod command;
mod response;

use command::FtpCommand;
use response::FtpResponse;

/// A client for interacting with an FTPS (FTP Secure) server.
///
/// This struct represents a client that connects to an FTPS server and can perform login and command
/// operations over a secure TLS connection.
pub struct FtpsClient {
    strean: async_native_tls::TlsStream<TcpStream>,
}

impl FtpsClient {
    /// Logs into the FTPS server with the given username and password.
    ///
    /// # Arguments
    ///
    /// * `user` - The username for authentication.
    /// * `pass` - The password for authentication.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or an error if the login process fails.
    ///
    /// # Errors
    ///
    /// This function returns errors if the command cannot be sent or if the response cannot be read.
    pub async fn login(
        &mut self,
        user: &str,
        pass: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.send_command(FtpCommand::User(user.to_string()))
            .await?;
        self.read_response().await?;
        self.send_command(FtpCommand::Pass(pass.to_string()))
            .await?;
        self.read_response().await?;

        Ok(())
    }

    /// Sends an FTP command to the FTPS server.
    ///
    /// # Arguments
    ///
    /// * `command` - The `FtpCommand` to be sent to the server.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or an error if the command cannot be sent.
    ///
    /// # Errors
    ///
    /// This function returns errors if the command cannot be written to the stream or if flushing fails.
    pub async fn send_command(
        &mut self,
        command: FtpCommand,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.strean
            .write_all(command.to_string().as_bytes())
            .await?;
        self.strean.flush().await?;
        Ok(())
    }

    /// Reads a response from the FTPS server.
    ///
    /// # Returns
    ///
    /// Returns a `FtpResponse` if successful or an error if the response cannot be read.
    ///
    /// # Errors
    ///
    /// This function returns errors if the response cannot be read from the stream or if conversion
    /// from bytes to a string fails.
    async fn read_response(&mut self) -> Result<FtpResponse, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        self.strean.read_to_end(&mut buffer).await?;
        let response = String::from_utf8(buffer)?;
        FtpResponse::from_string(response)
    }

    /// Connects to the FTPS server and establishes a TLS-secured connection.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address of the FTPS server to connect to.
    ///
    /// # Returns
    ///
    /// Returns an instance of `FtpsClient` if successful or an error if the connection fails.
    ///
    /// # Errors
    ///
    /// This function returns errors if the TCP connection or TLS handshake fails.
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(addr).await?;
        let connector = TlsConnector::new();
        let tls_stream = connector.connect(addr, stream).await?;

        Ok(FtpsClient { strean: tls_stream })
    }
}
