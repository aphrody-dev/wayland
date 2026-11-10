use byteorder::{ByteOrder, LittleEndian};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum WireError {
    #[error("Buffer too short: needed {needed}, available {available}")]
    BufferTooShort { needed: usize, available: usize },

    #[error("Invalid message size: {0} (must be >= 8 and 4-byte aligned)")]
    InvalidMessageSize(u16),

    #[error("Invalid string: null terminator missing or invalid utf8")]
    InvalidString,

    #[error("Unexpected end of stream")]
    UnexpectedEof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WaylandArgument {
    Int(i32),
    Uint(u32),
    Fixed(f64),
    String(String),
    Object(u32),
    NewId(u32),
    Array(Vec<u8>),
    Fd(i32),
}

impl WaylandArgument {
    pub fn as_uint(&self) -> Option<u32> {
        match self {
            WaylandArgument::Uint(v) => Some(*v),
            WaylandArgument::NewId(v) => Some(*v),
            WaylandArgument::Object(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            WaylandArgument::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_fixed(&self) -> Option<f64> {
        match self {
            WaylandArgument::Fixed(v) => Some(*v),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WaylandMessage {
    pub sender_id: u32,
    pub opcode: u16,
    pub args: Vec<WaylandArgument>,
}

impl WaylandMessage {
    pub fn new(sender_id: u32, opcode: u16, args: Vec<WaylandArgument>) -> Self {
        Self {
            sender_id,
            opcode,
            args,
        }
    }

    pub fn encode(&self, buf: &mut Vec<u8>) {
        let start_offset = buf.len();
        // Reserve 8 bytes for header: sender_id (4), opcode (2), size (2)
        buf.extend_from_slice(&[0u8; 8]);
        LittleEndian::write_u32(&mut buf[start_offset..start_offset + 4], self.sender_id);
        LittleEndian::write_u16(&mut buf[start_offset + 4..start_offset + 6], self.opcode);

        for arg in &self.args {
            match arg {
                WaylandArgument::Int(v) => {
                    let mut b = [0u8; 4];
                    LittleEndian::write_i32(&mut b, *v);
                    buf.extend_from_slice(&b);
                }
                WaylandArgument::Uint(v) | WaylandArgument::Object(v) | WaylandArgument::NewId(v) => {
                    let mut b = [0u8; 4];
                    LittleEndian::write_u32(&mut b, *v);
                    buf.extend_from_slice(&b);
                }
                WaylandArgument::Fixed(f) => {
                    let fixed_raw = (*f * 256.0) as i32;
                    let mut b = [0u8; 4];
                    LittleEndian::write_i32(&mut b, fixed_raw);
                    buf.extend_from_slice(&b);
                }
                WaylandArgument::String(s) => {
                    let bytes = s.as_bytes();
                    let len = (bytes.len() + 1) as u32; // including null terminator
                    let mut b = [0u8; 4];
                    LittleEndian::write_u32(&mut b, len);
                    buf.extend_from_slice(&b);
                    buf.extend_from_slice(bytes);
                    buf.push(0); // null terminator
                    let pad = (4 - ((bytes.len() + 1) % 4)) % 4;
                    buf.extend_from_slice(&vec![0u8; pad]);
                }
                WaylandArgument::Array(arr) => {
                    let len = arr.len() as u32;
                    let mut b = [0u8; 4];
                    LittleEndian::write_u32(&mut b, len);
                    buf.extend_from_slice(&b);
                    buf.extend_from_slice(arr);
                    let pad = (4 - (arr.len() % 4)) % 4;
                    buf.extend_from_slice(&vec![0u8; pad]);
                }
                WaylandArgument::Fd(_) => {
                    // Fds are sent via ancillary data in unix / hyperv, no payload in wire body
                }
            }
        }

        let total_size = (buf.len() - start_offset) as u16;
        LittleEndian::write_u16(&mut buf[start_offset + 6..start_offset + 8], total_size);
    }

    pub fn decode_raw(buf: &[u8]) -> Result<(u32, u16, usize, &[u8]), WireError> {
        if buf.len() < 8 {
            return Err(WireError::BufferTooShort {
                needed: 8,
                available: buf.len(),
            });
        }
        let sender_id = LittleEndian::read_u32(&buf[0..4]);
        let opcode = LittleEndian::read_u16(&buf[4..6]);
        let size = LittleEndian::read_u16(&buf[6..8]);

        if size < 8 || (size % 4) != 0 {
            return Err(WireError::InvalidMessageSize(size));
        }

        if buf.len() < size as usize {
            return Err(WireError::BufferTooShort {
                needed: size as usize,
                available: buf.len(),
            });
        }

        let payload = &buf[8..size as usize];
        Ok((sender_id, opcode, size as usize, payload))
    }
}

impl fmt::Display for WaylandMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "wl_msg[sender={}, op={}, args={:?}]",
            self.sender_id, self.opcode, self.args
        )
    }
}
