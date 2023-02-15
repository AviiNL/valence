use std::{
    io::{ErrorKind, Read, Write},
    path::PathBuf,
    sync::RwLock,
};

use bytes::BytesMut;

use time::OffsetDateTime;
use valence_protocol::{Decode, DecodePacket, PacketDecoder};

use crate::packet_widget::PacketDirection;

pub struct PacketBuilder {
    pub direction: PacketDirection,
    pub data: Vec<u8>,
}

// impl PacketBuilder {
//     pub fn new(direction: PacketDirection, data: Vec<u8>) -> Self {
//         Self { direction, data }
//     }

//     pub fn to_packet<P>(&self) -> anyhow::Result<Packet>
//     where
//         P: for<'a> DecodePacket<'a>,
//     {
//         let first_byte = self.data[0];

//         let mut dec = PacketDecoder::new();
//         dec.queue_bytes(BytesMut::from(&self.data[..]));

//         let pkt: P = dec.try_next_packet()?.unwrap();

//         let mut packet = String::new();
//         let mut formatted_packet = String::new();

//         std::fmt::Write::write_fmt(&mut packet, format_args!("{pkt:?}")).unwrap(); // unwrapping because we already know it works
//         std::fmt::Write::write_fmt(&mut formatted_packet, format_args!("{pkt:#?}")).unwrap(); // unwrapping because we already know it works

//         let packet_name = packet
//             .split_once(|ch: char| !ch.is_ascii_alphanumeric())
//             .map(|(fst, _)| fst)
//             .unwrap_or(&packet);

//         let time = match OffsetDateTime::now_local() {
//             Ok(time) => time,
//             Err(_) => {
//                 eprintln!("Unable to get local time, using UTC"); // this might get a bit spammy..
//                 OffsetDateTime::now_utc()
//             }
//         };

//         Ok(Packet {
//             id: 0,
//             direction: self.direction,
//             selected: false,
//             packet_type: first_byte,
//             packet_name: packet_name.to_owned(),
//             packet: Vec::new(),
//             packet_raw: Vec::new(),
//             created_at: time,
//         })
//     }
// }

#[derive(Clone)]
pub struct Packet {
    pub(crate) id: usize,
    pub(crate) direction: PacketDirection,
    pub(crate) selected: bool,
    pub(crate) packet: Vec<u8>,
    pub(crate) created_at: OffsetDateTime,
}

impl Packet {
    pub(crate) fn selected(&mut self, value: bool) {
        self.selected = value;
    }

    pub fn get_packet_type(&self) -> u8 {
        self.packet[0]
    }

    pub fn get_packet_name<P>(&self) -> String
    where
        P: for<'a> DecodePacket<'a>,
    {
        let mut dec = PacketDecoder::new();
        dec.queue_slice(&self.packet);

        let pkt: P = dec.try_next_packet().unwrap().unwrap();

        let mut packet = String::new();
        std::fmt::Write::write_fmt(&mut packet, format_args!("{pkt:?}")).unwrap();

        let packet_name = packet
            .split_once(|ch: char| !ch.is_ascii_alphanumeric())
            .map(|(fst, _)| fst)
            .unwrap_or(&packet);

        packet_name.to_owned()
    }
}

pub struct Context {
    pub selected_packet: RwLock<Option<usize>>,
    pub(crate) packets: RwLock<Vec<Packet>>,
    pub(crate) packet_count: RwLock<usize>,
    pub filter: RwLock<String>,
    context: Option<egui::Context>,
}

impl Context {
    pub fn new(ctx: Option<egui::Context>) -> Self {
        Self {
            selected_packet: RwLock::new(None),
            packets: RwLock::new(Vec::new()),
            filter: RwLock::new("".into()),
            context: ctx,
            packet_count: RwLock::new(0),
        }
    }

    pub fn clear(&self) {
        *self.selected_packet.write().expect("Poisoned RwLock") = None;
        self.packets.write().expect("Poisoned RwLock").clear();
        if let Some(ctx) = &self.context {
            ctx.request_repaint();
        }
    }

    pub fn add(&self, mut packet: Packet) {
        packet.id = self.packets.read().expect("Poisened RwLock").len();
        self.packets.write().expect("Poisoned RwLock").push(packet);
        if let Some(ctx) = &self.context {
            ctx.request_repaint();
        }
    }

    pub fn set_selected_packet(&self, idx: usize) {
        *self.selected_packet.write().expect("Poisoned RwLock") = Some(idx);
    }

    pub fn set_filter(&self, filter: String) {
        *self.filter.write().expect("Posisoned RwLock") = filter;
        *self.selected_packet.write().expect("Poisoned RwLock") = None;
    }

    pub fn save(&self, path: PathBuf) -> Result<(), std::io::Error> {
        // let packets = self
        //     .packets
        //     .read()
        //     .expect("Poisoned RwLock")
        //     .iter()
        //     .filter(|packet| packet.packet_name != "ChunkDataAndUpdateLight") // temporarily blacklisting this packet because HUGE
        //     .map(|packet| packet.get_packet_str())
        //     .collect::<Vec<String>>()
        //     .join("\n");

        // std::fs::write(path, packets)?;

        Ok(())
    }
}
