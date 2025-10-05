use anyhow::{
    Result,
    bail
};
use byteorder::{
    ByteOrder,
    BigEndian
};
use log::{
    debug,
    info,
    warn
};

// HEADER_SIZE is now 65 bytes (57 + 8 for payload_len)
pub const HEADER_SIZE: usize = 65;

const MAGIC: &[u8; 4] = b"P2WV";

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub version: u8,
    pub nonce: [u8; 12],
    pub auth_tag: [u8; 16],
    pub salt: [u8; 16],
    pub width: u32,
    pub height: u32,
    pub payload_len: u64, // length of the f data
}

impl Header {
    /// New header
    pub fn new(version: u8, nonce: [u8; 12], auth_tag: [u8; 16], salt: [u8; 16], width: u32, height: u32, payload_len: u64) -> Self {
        debug!("New Header (version = {})", version);
        Self { version, nonce, auth_tag, salt, width, height, payload_len }
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> [u8; HEADER_SIZE] {
        let mut buf = [0u8; HEADER_SIZE];
        buf[..4].copy_from_slice(MAGIC);
        buf[4] = self.version;
        buf[5..17].copy_from_slice(&self.nonce);
        buf[17..33].copy_from_slice(&self.auth_tag);
        buf[33..49].copy_from_slice(&self.salt);
        BigEndian::write_u32(&mut buf[49..53], self.width);
        BigEndian::write_u32(&mut buf[53..57], self.height);
        BigEndian::write_u64(&mut buf[57..65], self.payload_len);
        debug!("Serialized Header to {} bytes", HEADER_SIZE);
        buf
    }

    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        if buf.len() < HEADER_SIZE {
            bail!("Buf too small for Header (got {}, expected {})", buf.len(), HEADER_SIZE);
        }

        if &buf[..4] != MAGIC {
            warn!("Invalid magic bytes");
            bail!("Invalid header magic");
        }

        let version = buf[4];

        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&buf[5..17]);

        let mut auth_tag = [0u8; 16];
        auth_tag.copy_from_slice(&buf[17..33]);

        let mut salt = [0u8; 16];
        salt.copy_from_slice(&buf[33..49]);

        let width = BigEndian::read_u32(&buf[49..53]);
        let height = BigEndian::read_u32(&buf[53..57]);
        let payload_len = BigEndian::read_u64(&buf[57..65]);

        info!("Parsed Header (version = {}, payload_len = {})", version, payload_len);

        Ok(Self { version, nonce, auth_tag, salt, width, height, payload_len })
    }
}