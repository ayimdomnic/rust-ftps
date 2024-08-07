/// Represents FTP commands used for communication with an FTP server.
///
/// This enum includes variants for common FTP commands such as `USER` and `PASS`, which are
/// used for user authentication. Additional FTP commands can be added as needed.
pub enum FtpCommand {
    /// The `USER` command to provide a username for authentication.
    ///
    /// # Variants
    ///
    /// * `User(String)` - Contains the username to be sent to the FTP server.
    User(String),

    /// The `PASS` command to provide a password for authentication.
    ///
    /// # Variants
    ///
    /// * `Pass(String)` - Contains the password to be sent to the FTP server.
    Pass(String),
}

impl FtpCommand {
    /// Converts an `FtpCommand` to its string representation for sending to the FTP server.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `FtpCommand` instance to be converted to a string.
    ///
    /// # Returns
    ///
    /// Returns a `String` representation of the FTP command formatted for transmission.
    ///
    /// # Examples
    ///
    /// ```
    /// let command = FtpCommand::User("example_user".to_string());
    /// assert_eq!(command.to_string(), "USER example_user\r\n");
    ///
    /// let command = FtpCommand::Pass("example_pass".to_string());
    /// assert_eq!(command.to_string(), "PASS example_pass\r\n");
    /// ```
    pub fn to_string(&self) -> String {
        match self {
            FtpCommand::User(username) => format!("USER {}\r\n", username),
            FtpCommand::Pass(password) => format!("PASS {}\r\n", password),
        }
    }
}
