#[cfg(test)]
mod chunk_test;

pub(crate) mod chunk_abort;
pub(crate) mod chunk_cookie_ack;
pub(crate) mod chunk_cookie_echo;
pub(crate) mod chunk_error;
pub(crate) mod chunk_forward_tsn;
pub(crate) mod chunk_header;
pub(crate) mod chunk_heartbeat;
pub(crate) mod chunk_heartbeat_ack;
pub(crate) mod chunk_init;
pub mod chunk_payload_data;
pub(crate) mod chunk_reconfig;
pub(crate) mod chunk_selective_ack;
pub(crate) mod chunk_shutdown;
pub(crate) mod chunk_shutdown_ack;
pub(crate) mod chunk_shutdown_complete;
pub(crate) mod chunk_type;

use crate::error::Error;
use chunk_header::*;

use bytes::{Bytes, BytesMut};
use std::marker::Sized;
use std::{any::Any, fmt};

pub(crate) trait Chunk: fmt::Display + fmt::Debug {
    fn header(&self) -> ChunkHeader;
    fn unmarshal(raw: &Bytes) -> Result<Self, Error>
    where
        Self: Sized;
    fn marshal_to(&self, buf: &mut BytesMut) -> Result<usize, Error>;
    fn check(&self) -> Result<(), Error>;
    fn value_length(&self) -> usize;
    fn as_any(&self) -> &(dyn Any + Send + Sync);

    fn marshal(&self) -> Result<Bytes, Error> {
        let capacity = CHUNK_HEADER_SIZE + self.value_length();
        let mut buf = BytesMut::with_capacity(capacity);
        self.marshal_to(&mut buf)?;
        Ok(buf.freeze())
    }
}
