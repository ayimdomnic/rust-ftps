pub enum FtpCommand {
    User(String),
    Pass(String),

    // Check if there are other needed commands
}

impl FtpCommand {
    pub fn to_string(&self) -> String {
        match self {
            FtpCommand::User(username) => format!("USER {}\r\n", username),
            FtpCommand::Pass(password) => format!("PASS {}\r\n", password)

            //handle any additional commands
        }
    }
}