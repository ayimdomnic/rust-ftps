use async_native_tls::TlsConnector;
use async_std::{
    io::{ReadExt, WriteExt},
    net::TcpStream,
};

mod command;
mod response;

use command::FtpCommand;
use response::FtpResponse;

pub struct FtpsClient {
    strean: async_native_tls::TlsStream<TcpStream>,
}

impl FtpsClient {
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

    pub async fn send_command(
        &mut self,
        command: FtpCommand,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.stream
            .write_all(command.to_string().as_bytes())
            .await?;
        self.strean.flush().await?;
        Ok(())
    }

    async fn read_response(&mut self) -> Result<FtpResponse, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        self.strean.read_to_end(&mut buffer).await?;
        let response = String::from_utf8(buffer)?;
        FtpResponse::from_string(response)
    }
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(addrs).await?;
        let connector = TlsConnector::new();
        let tls_stream = connector.connect(addr, stream).await?;

        Ok(FtpsClient { strean: tls_stream })
    }
}
