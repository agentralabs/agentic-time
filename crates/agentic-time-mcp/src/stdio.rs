//! Hardened stdio transport — Content-Length framed JSON-RPC 2.0 (MCP standard).

use std::io::{BufRead, BufReader, Read, Write};

/// Maximum frame size: 8 MiB.
pub const MAX_CONTENT_LENGTH_BYTES: usize = 8 * 1024 * 1024;
/// JSON-RPC stdio framing header marker.
pub const CONTENT_LENGTH_HEADER: &str = "content-length:";
/// Backward-compatible alias for existing transport checks.
pub const MAX_MESSAGE_BYTES: usize = MAX_CONTENT_LENGTH_BYTES;

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

/// Hardened stdio transport using Content-Length framing.
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

    /// Read a single Content-Length framed JSON message.
    pub fn read_message(&mut self) -> Result<String, TransportError> {
        let mut content_length: Option<usize> = None;

        // Parse headers until the terminating empty line.
        loop {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line)?;
            if bytes_read == 0 {
                return Err(TransportError::Io(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "EOF while reading headers",
                )));
            }

            let trimmed = line.trim_end_matches(['\r', '\n']);
            if trimmed.is_empty() {
                break;
            }

            if let Some((name, value)) = trimmed.split_once(':') {
                let header_name = CONTENT_LENGTH_HEADER.trim_end_matches(':');
                if name.trim().eq_ignore_ascii_case(header_name) {
                    let parsed = value.trim().parse::<usize>().map_err(|_| {
                        TransportError::Io(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "invalid Content-Length value",
                        ))
                    })?;
                    content_length = Some(parsed);
                }
            }
        }

        let len = content_length.ok_or_else(|| {
            TransportError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "missing Content-Length header",
            ))
        })?;

        if len > MAX_MESSAGE_BYTES {
            return Err(TransportError::MessageTooLarge(len, MAX_MESSAGE_BYTES));
        }

        let mut body = vec![0u8; len];
        self.reader.read_exact(&mut body)?;
        let message = String::from_utf8(body).map_err(|_| TransportError::InvalidUtf8)?;
        Ok(message)
    }

    /// Write a JSON message with MCP Content-Length framing.
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
        let input = b"Content-Length: 13\r\n\r\n{\"test\":true}";
        let mut output = Vec::new();

        let mut transport = StdioTransport::new(std::io::Cursor::new(input.to_vec()), &mut output);
        let msg = transport.read_message().unwrap();
        assert_eq!(msg, "{\"test\":true}");

        transport.write_message("hello").unwrap();
        let written = String::from_utf8(output).unwrap();
        assert_eq!(written, "Content-Length: 5\r\n\r\nhello");
    }

    #[test]
    fn test_case_insensitive_content_length() {
        let input = b"content-length: 4\r\n\r\ntest";
        let mut output = Vec::new();
        let mut transport = StdioTransport::new(std::io::Cursor::new(input.to_vec()), &mut output);
        let msg = transport.read_message().unwrap();
        assert_eq!(msg, "test");
    }

    #[test]
    fn test_missing_content_length_fails() {
        let input = b"No-Header: value\r\n\r\n{}";
        let mut output = Vec::new();
        let mut transport = StdioTransport::new(std::io::Cursor::new(input.to_vec()), &mut output);
        assert!(transport.read_message().is_err());
    }

    #[test]
    fn test_validate_jsonrpc() {
        let valid: serde_json::Value =
            serde_json::from_str(r#"{"jsonrpc":"2.0","method":"test"}"#).unwrap_or_default();
        assert!(validate_jsonrpc(&valid).is_ok());

        let invalid: serde_json::Value =
            serde_json::from_str(r#"{"jsonrpc":"1.0","method":"test"}"#).unwrap_or_default();
        assert!(validate_jsonrpc(&invalid).is_err());

        let missing: serde_json::Value =
            serde_json::from_str(r#"{"method":"test"}"#).unwrap_or_default();
        assert!(validate_jsonrpc(&missing).is_err());
    }
}
