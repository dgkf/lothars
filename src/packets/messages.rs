use std::io::{Read,Error};
use std::fmt::{Display,Formatter};

use bitvec::prelude::{BitVec,Lsb0};
use protobuf::{CodedInputStream,Message,ProtobufResult,ProtobufEnum};
use snap::raw::Decoder;

use crate::utils::*;
use super::commands::*;
use crate::protobufs::demo::*;
use crate::protobufs::netmessages::*;
use crate::protobufs::networkbasetypes::*;

#[derive(Debug)]
pub struct CDemoMessage<T> {
    pub kind: EDemoCommands,
    pub msg: DemoMessage<T>,
    pub compressed: bool,
    pub tick: u32,
    pub size: usize,
}

impl<T: Message + ?Sized> Display for CDemoMessage<Box<T>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let tick = if self.tick == std::u32::MAX { -1 } else { self.tick as i64 };
        let min = tick / (30 * 60);
        let sec = tick % (30 * 60) / 30;
        let tick_rem = tick % 30;

        write!(f, "{:?} ({:.2}B @{:0.}:{:02.}{:+}t)", self.kind, self.size, min, sec, tick_rem)
    }
}

pub fn read_next_message_from<'a>(cis: &'a mut CodedInputStream) -> ProtobufResult<CDemoMessage<Box<dyn Message>>> {
    const IS_COMPRESSED: EDemoCommands = EDemoCommands::DEM_IsCompressed;

    // retrieve cmd enum
    let raw_cmd = cis.read_raw_varint32()? as i32;
    let compressed: bool = !(raw_cmd & (IS_COMPRESSED as i32) == 0);
    let raw_cmd: i32 = raw_cmd & !(IS_COMPRESSED as i32);

    let tick = cis.read_raw_varint32()? as u32;
    let size = cis.read_raw_varint32()? as usize;
    let kind = match EDemoCommands::from_i32(raw_cmd) {
        Some(cmd) => cmd,
        None      => EDemoCommands::DEM_Error,
    };

    let mut msg = new_message_from_command(DemoCommand::EDemo(kind));
    let data = read_message_data(cis, size, compressed);

    // parse message from byte stream
    match msg {
        DemoMessage::Standalone(ref mut m) => m.merge_from_bytes(&data),
        DemoMessage::Packet(ref mut m)     => m.merge_from_bytes(&data),
        DemoMessage::FullPacket(ref mut m) => m.merge_from_bytes(&data),
    }?;

    // parse inner messages from byte stream
    // TODO:
    //   - handle FullPacket
    //   - actually use contents of packet
    let _res = match msg {
        DemoMessage::Packet(ref p) => read_data_messages_from_packet(p),
        _                          => Ok(Vec::new()),
    };

    Ok(CDemoMessage{ kind, msg, compressed, tick, size })
}

pub fn read_message_data<'a>(cis: &'a mut CodedInputStream, size: usize, compressed: bool) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![0; size];
    cis.read_exact(&mut buffer).unwrap();

    let decomp_buffer = if compressed {
        Decoder::new().decompress_vec(&buffer).unwrap()
    } else {
        buffer
    };

    decomp_buffer
}

pub fn read_data_messages_from_packet(p: &CDemoPacket) -> Result<Vec<Box<dyn protobuf::Message>>, Error> {
    let inner_packets = Vec::new();

    // pad packet data with an extra byte to appease bitvec Read implementation
    // https://github.com/bitvecto-rs/bitvec/issues/170
    let mut packet_data: Vec<u8> = Vec::new();
    packet_data.extend_from_slice(p.get_data());
    packet_data.extend_from_slice(&[0_u8]);

    // extract packet data as bitvec to parse sub-byte encoded header
    let mut bv = BitVec::<_, Lsb0>::from_slice(&packet_data);

    // while there's at least one byte in the data packet
    while bv.len() > 7 + 8 {
        // read bit-variable command enum from bitvec directly
        let cmd = read_ubitvar(&mut bv) as i32;
        let size = read_varuint32(&mut bv) as usize;

        let mut data: Vec<u8> = vec![0; size];
        let _n = bv.read(&mut data)?;

        // try to parse command enum from raw cmd
        if let Some(kind) = NET_Messages::from_i32(cmd) {
            // println!("  [{cmd:03?}] {:?}", kind);
            if let DemoMessage::Standalone(mut inner_msg) = new_message_from_command(DemoCommand::NET(kind)) {
                let _res = inner_msg.merge_from_bytes(&data);
            }
        } else if let Some(kind) = SVC_Messages::from_i32(cmd) {
            // println!("  [{cmd:03?}] {:?}", kind);
            if let DemoMessage::Standalone(mut inner_msg) = new_message_from_command(DemoCommand::SVC(kind)) {
                let _res = inner_msg.merge_from_bytes(&data);
            }
        } else {
            // println!("  [{:03?}] Unknown Message ({:?} bytes)", cmd, size);
        };
    }

    Ok(inner_packets)
}
