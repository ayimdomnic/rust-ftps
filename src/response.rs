/// Represents a response from an FTP server.
///
/// This struct contains an FTP response code and message, which are used to interpret the
/// server's replies to FTP commands.
pub struct FtpResponse {
    /// The response code from the FTP server.
    ///
    /// This is a numerical code indicating the status of the response. Common FTP codes include
    /// `220` for a successful connection or `530` for authentication failure.
    pub code: u32,

    /// The response message from the FTP server.
    ///
    /// This is a textual description accompanying the response code, providing additional context
    /// or information about the response.
    pub message: String,
}

impl FtpResponse {
    /// Creates a new `FtpResponse` with the given code and message.
    ///
    /// # Arguments
    ///
    /// * `code` - The numerical response code from the FTP server.
    /// * `message` - The textual message accompanying the response code.
    ///
    /// # Returns
    ///
    /// Returns a new `FtpResponse` instance with the specified code and message.
    ///
    /// # Examples
    ///
    /// ```
    /// let response = FtpResponse::new(220, "Service ready for new user".to_string());
    /// assert_eq!(response.code, 220);
    /// assert_eq!(response.message, "Service ready for new user".to_string());
    /// ```
    pub fn new(code: u32, message: String) -> Self {
        FtpResponse { code, message }
    }

    /// Parses a response string into an `FtpResponse`.
    ///
    /// # Arguments
    ///
    /// * `response` - A string containing the full FTP response.
    ///
    /// # Returns
    ///
    /// Returns an `FtpResponse` instance if parsing is successful. If parsing fails (e.g., due to
    /// an invalid response format or incorrect code), returns an error.
    ///
    /// # Errors
    ///
    /// This function returns an error if the response code cannot be parsed as a `u32` or if the
    /// string does not split correctly into code and message parts.
    ///
    /// # Examples
    ///
    /// ```
    /// let response_str = "220 Service ready for new user".to_string();
    /// let response = FtpResponse::from_string(response_str).unwrap();
    /// assert_eq!(response.code, 220);
    /// assert_eq!(response.message, "Service ready for new user".to_string());
    /// ```
    pub fn from_string(response: String) -> Result<Self, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = response.splitn(2, ' ').collect();
        if parts.len() < 2 {
            return Err("Invalid response format".into());
        }

        let code = parts[0].parse()?;
        let message = parts[1].to_string();

        Ok(FtpResponse { code, message })
    }
}
