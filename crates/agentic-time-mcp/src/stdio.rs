//! Hardened stdio transport — newline-delimited JSON-RPC 2.0 (MCP standard).

use std::io::{BufRead, BufReader, Read, Write};

/// Maximum message size: 8 MiB.
pub const MAX_MESSAGE_BYTES: usize = 8 * 1024 * 1024;

/// Transport errors.
#[derive(Debug, thiserror::Error)]
pub enum TransportError {
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

/// Hardened stdio transport using newline-delimited JSON.
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

    /// Read a single newline-delimited JSON message.
    pub fn read_message(&mut self) -> Result<String, TransportError> {
        let mut line = String::new();
        let bytes_read = self.reader.read_line(&mut line)?;
        if bytes_read == 0 {
            return Err(TransportError::Io(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "EOF on stdin",
            )));
        }

        let trimmed = line.trim().to_string();
        if trimmed.is_empty() {
            // Skip blank lines and read the next one
            return self.read_message();
        }

        if trimmed.len() > MAX_MESSAGE_BYTES {
            return Err(TransportError::MessageTooLarge(
                trimmed.len(),
                MAX_MESSAGE_BYTES,
            ));
        }

        Ok(trimmed)
    }

    /// Write a JSON message followed by a newline.
    pub fn write_message(&mut self, content: &str) -> Result<(), TransportError> {
        self.writer.write_all(content.as_bytes())?;
        self.writer.write_all(b"\n")?;
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
        let input = "{\"test\":true}\n";
        let mut output = Vec::new();

        let mut transport =
            StdioTransport::new(std::io::Cursor::new(input.as_bytes().to_vec()), &mut output);
        let msg = transport.read_message().unwrap();
        assert_eq!(msg, "{\"test\":true}");
    }

    #[test]
    fn test_skip_blank_lines() {
        let input = "\n\n{\"test\":true}\n";
        let mut output = Vec::new();

        let mut transport =
            StdioTransport::new(std::io::Cursor::new(input.as_bytes().to_vec()), &mut output);
        let msg = transport.read_message().unwrap();
        assert_eq!(msg, "{\"test\":true}");
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
