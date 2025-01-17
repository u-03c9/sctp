use super::{chunk_header::*, chunk_type::*, *};

use bytes::{Bytes, BytesMut};
use std::fmt;

/// CookieEcho represents an SCTP Chunk of type CookieEcho
///
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |   Type = 10   |Chunk  Flags   |         Length                |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                     Cookie                                    |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
#[derive(Default, Debug, Clone)]
pub(crate) struct ChunkCookieEcho {
    pub(crate) cookie: Bytes,
}

/// makes ChunkCookieEcho printable
impl fmt::Display for ChunkCookieEcho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.header())
    }
}

impl Chunk for ChunkCookieEcho {
    fn header(&self) -> ChunkHeader {
        ChunkHeader {
            typ: CT_COOKIE_ECHO,
            flags: 0,
            value_length: self.value_length() as u16,
        }
    }

    fn unmarshal(raw: &Bytes) -> Result<Self, Error> {
        let header = ChunkHeader::unmarshal(raw)?;

        if header.typ != CT_COOKIE_ECHO {
            return Err(Error::ErrChunkTypeNotCookieEcho);
        }

        let cookie = raw.slice(CHUNK_HEADER_SIZE..CHUNK_HEADER_SIZE + header.value_length());
        Ok(ChunkCookieEcho { cookie })
    }

    fn marshal_to(&self, buf: &mut BytesMut) -> Result<usize, Error> {
        self.header().marshal_to(buf)?;
        buf.extend(self.cookie.clone());
        Ok(buf.len())
    }

    fn check(&self) -> Result<(), Error> {
        Ok(())
    }

    fn value_length(&self) -> usize {
        self.cookie.len()
    }

    fn as_any(&self) -> &(dyn Any + Send + Sync) {
        self
    }
}
