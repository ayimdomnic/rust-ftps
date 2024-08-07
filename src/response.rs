pub struct FtpResponse {
    pub code: u32,
    pub message: String
}

impl FtpResponse {
    pub fn new(code: u32, message: String) -> Self {
        FtpResponse {
            code, 
            message
        }
    }

    pub fn from_string(response: string) -> Result<Self, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = response.splitn(2, ' ').collect();
        let code = parts[0].parse()?;
        let message = parts[1].to_string();

        Ok(FtpResponse { code, message })
    }
}