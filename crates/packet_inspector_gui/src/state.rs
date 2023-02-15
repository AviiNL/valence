use std::io::ErrorKind;
use std::sync::Arc;

use time::OffsetDateTime;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use valence_protocol::{DecodePacket, EncodePacket, PacketDecoder, PacketEncoder};

use crate::context::{Context, Packet, PacketBuilder};
use crate::packet_widget::PacketDirection;

pub struct State {
    pub direction: PacketDirection,
    pub context: Arc<Context>,
    pub enc: PacketEncoder,
    pub dec: PacketDecoder,
    pub read: OwnedReadHalf,
    pub write: OwnedWriteHalf,
}

impl State {
    pub async fn rw_packet<'a, P>(&'a mut self) -> anyhow::Result<P>
    where
        P: DecodePacket<'a> + EncodePacket,
    {
        while !self.dec.has_next_packet()? {
            self.dec.reserve(4096);
            let mut buf = self.dec.take_capacity();

            if self.read.read_buf(&mut buf).await? == 0 {
                return Err(std::io::Error::from(ErrorKind::UnexpectedEof).into());
            }

            self.dec.queue_bytes(buf);
        }

        let pkt: P = self.dec.try_next_packet()?.unwrap();

        self.enc.append_packet(&pkt)?;

        let bytes = self.enc.take(); // this is already a re-encoded packet..
        self.write.write_all(&bytes).await?;

        let direction = self.direction.clone();
        let context = self.context.clone();

        let time = match OffsetDateTime::now_local() {
            Ok(time) => time,
            Err(_) => {
                eprintln!("Unable to get local time, using UTC"); // this might get a bit spammy..
                OffsetDateTime::now_utc()
            }
        };

        // tokio::spawn(async move {
        // let pb = PacketBuilder::new(direction, bytes.to_vec()); // stick the bytes in the packetbuilder
        // let packet = pb.to_packet::<P>().unwrap(); // have the packetbuilder poop out a Packet {}
        context.add(Packet {
            id: 0,
            direction,
            selected: false,
            packet: bytes.to_vec(),
            created_at: time,
        });

        Ok(pkt)
    }
}
