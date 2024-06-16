use anyhow::Result;
use bytes::{Buf, BytesMut};
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder, Framed};
use tracing::info;

use crate::{Backend, Command, CommandExecutor, RespDecode, RespEncode, RespError, RespFrame};

pub async fn stream_handler(stream: TcpStream, backend: Backend) -> Result<()> {
    let mut framed = Framed::new(stream, RespFrameCodec);
    loop {
        match framed.next().await {
            Some(Ok(frame)) => {
                let request = RedisRequest {
                    frame,
                    backend: backend.clone(),
                };
                let response = request_handler(request).await?;
                framed.send(response.frame).await?;
            }
            Some(Err(e)) => return Err(e),
            None => return Ok(()),
        }
    }
}

struct RedisRequest {
    frame: RespFrame,
    backend: Backend,
}

#[derive(Debug)]
struct RedisResponse {
    frame: RespFrame,
}

#[derive(Debug)]
struct RespFrameCodec;

async fn request_handler(request: RedisRequest) -> Result<RedisResponse> {
    let (frame, backend) = (request.frame, request.backend);
    let cmd = Command::try_from(frame)?;
    info!("Executing command: {cmd:?}");
    let frame = cmd.execute(&backend);
    Ok(RedisResponse { frame })
}

impl Encoder<RespFrame> for RespFrameCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: RespFrame, dst: &mut BytesMut) -> Result<()> {
        dst.extend_from_slice(item.encode().as_slice());
        Ok(())
    }
}

impl Decoder for RespFrameCodec {
    type Item = RespFrame;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        match RespFrame::decode(src.as_ref()) {
            Ok(frame) => {
                src.advance(frame.byte_size());
                Ok(Some(frame))
            }
            Err(RespError::NotComplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
