//! Hardened stdio transport — Content-Length framing, 8 MiB limit, JSON-RPC 2.0 validation.

use std::io::{BufRead, BufReader, Read, Write};

/// Maximum content length: 8 MiB.
pub const MAX_CONTENT_LENGTH_BYTES: usize = 8 * 1024 * 1024;

/// Transport errors.
#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    /// Missing Content-Length header.
    #[error("Missing Content-Length header")]
    MissingContentLength,

    /// Message exceeds maximum size.
    #[error("Message too large: {0} bytes (max {1})")]
    MessageTooLarge(usize, usize),

    /// Invalid UTF-8 in message.
    #[error("Invalid UTF-8 in message")]
    InvalidUtf8,

    /// Invalid JSON-RPC version.
    #[error("Invalid JSON-RPC version: expected \"2.0\", got {0:?}")]
    InvalidJsonRpcVersion(String),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON parse error.
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Hardened stdio transport.
pub struct StdioTransport<R: Read, W: Write> {
    reader: BufReader<R>,
    writer: W,
}

impl<R: Read, W: Write> StdioTransport<R, W> {
    /// Create a new transport.
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: BufReader::new(reader),
            writer,
        }
    }

    /// Read a single message using Content-Length framing.
    pub fn read_message(&mut self) -> Result<String, TransportError> {
        let mut content_length: Option<usize> = None;

        // Read headers until blank line
        loop {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line)?;
            if bytes_read == 0 {
                return Err(TransportError::Io(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "EOF while reading headers",
                )));
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                break;
            }

            // Case-insensitive header parsing: accept "content-length:" in any case
            let lower = trimmed.to_lowercase();
            if let Some(value) = lower.strip_prefix("content-length:") {
                content_length = Some(
                    value
                        .trim()
                        .parse::<usize>()
                        .map_err(|_| TransportError::MissingContentLength)?,
                );
            }
        }

        let length = content_length.ok_or(TransportError::MissingContentLength)?;

        // Validate size BEFORE reading body
        if length > MAX_CONTENT_LENGTH_BYTES {
            return Err(TransportError::MessageTooLarge(
                length,
                MAX_CONTENT_LENGTH_BYTES,
            ));
        }

        // Read body
        let mut body = vec![0u8; length];
        self.reader.read_exact(&mut body)?;

        String::from_utf8(body).map_err(|_| TransportError::InvalidUtf8)
    }

    /// Write a message with Content-Length header.
    pub fn write_message(&mut self, content: &str) -> Result<(), TransportError> {
        let header = format!("Content-Length: {}\r\n\r\n", content.len());
        self.writer.write_all(header.as_bytes())?;
        self.writer.write_all(content.as_bytes())?;
        self.writer.flush()?;
        Ok(())
    }
}

/// Validate that a JSON value is a valid JSON-RPC 2.0 request.
pub fn validate_jsonrpc(request: &serde_json::Value) -> Result<(), TransportError> {
    match request.get("jsonrpc").and_then(|v| v.as_str()) {
        Some("2.0") => Ok(()),
        Some(other) => Err(TransportError::InvalidJsonRpcVersion(other.to_string())),
        None => Err(TransportError::InvalidJsonRpcVersion("missing".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write_message() {
        let input = "Content-Length: 13\r\n\r\n{\"test\":true}";
        let mut output = Vec::new();

        let mut transport =
            StdioTransport::new(std::io::Cursor::new(input.as_bytes().to_vec()), &mut output);
        let msg = transport.read_message().unwrap();
        assert_eq!(msg, "{\"test\":true}");
    }

    #[test]
    fn test_reject_oversized() {
        let header = format!("Content-Length: {}\r\n\r\n", MAX_CONTENT_LENGTH_BYTES + 1);
        let mut output = Vec::new();

        let mut transport = StdioTransport::new(
            std::io::Cursor::new(header.as_bytes().to_vec()),
            &mut output,
        );
        let result = transport.read_message();
        assert!(matches!(result, Err(TransportError::MessageTooLarge(_, _))));
    }

    #[test]
    fn test_validate_jsonrpc() {
        let valid: serde_json::Value =
            serde_json::from_str(r#"{"jsonrpc":"2.0","method":"test"}"#).unwrap();
        assert!(validate_jsonrpc(&valid).is_ok());

        let invalid: serde_json::Value =
            serde_json::from_str(r#"{"jsonrpc":"1.0","method":"test"}"#).unwrap();
        assert!(validate_jsonrpc(&invalid).is_err());

        let missing: serde_json::Value = serde_json::from_str(r#"{"method":"test"}"#).unwrap();
        assert!(validate_jsonrpc(&missing).is_err());
    }
}
