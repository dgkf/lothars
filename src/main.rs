#![cfg_attr(debug_assertions, allow(unused_imports))]

use std::fs::File;
use std::io::{Read,Error,ErrorKind};

use protobuf::{CodedInputStream,ProtobufEnum,ProtobufResult,Message};
use protobuf::error::*;
use snap;

use bitvec::prelude::*;
use bitvec::store::BitStore;
use bitvec::order::BitOrder;

mod protobufs;
use crate::protobufs::demo::*;
use crate::protobufs::networkbasetypes::*;
use crate::protobufs::netmessages::*;

#[derive(Debug)]
struct CDemoPacketMetadata {
    kind: EDemoCommands,
    compressed: bool,
    tick: u32,
    size: usize,
}

fn read_demo_message_metadata(cis: &mut CodedInputStream) -> ProtobufResult<CDemoPacketMetadata> {
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

    Ok(CDemoPacketMetadata { kind, compressed, tick, size })
}


fn message_data(cis: &mut CodedInputStream, size: usize, compressed: bool)
    -> Result<Vec<u8>, Error> {
    let mut buffer: Vec<u8> = vec![0; size];
    cis.read_exact(&mut buffer).unwrap();

    let decomp_buffer = if compressed {
        // decompressed snappy-compression buffer
        snap::raw::Decoder::new().decompress_vec(&buffer)?
    } else {
        // already decompressed
        buffer
    };

    Ok(decomp_buffer)
}

fn print_message<T: protobuf::Message>(cis: &mut CodedInputStream, md: CDemoPacketMetadata, print: bool) 
-> Result<(), Error> {
    let data = message_data(cis, md.size, md.compressed)?;
    let message = T::parse_from_bytes(&data)?;
    if print { println!("{:?}", message); }
    Ok(())
}

fn read_ubitvar<T: BitStore>(bv: &mut BitVec<T, Lsb0>) -> u32 {
    // read 6 bits; 4 first bits of value, followed by 2 encoding remainder length
    let h = &bv[..6].load::<u32>();

    // determine total number of bits based on length encoding
    let nbits: usize = match h & 0b110000 {
        16 => 10,
        32 => 14,
        48 => 34,
        _  => 6,
    };

    // drain total bits from buffer, add remaining bits as high bits to return
    let all_bits: BitVec = bv.drain(..nbits).collect();
    let remainder = all_bits.load::<u32>();

    // use leading 4 bits + trailing bits
    h & 0b001111 | ((remainder >> 6) << 4)
}

fn print_packet_message(cis: &mut CodedInputStream, md: CDemoPacketMetadata) 
-> Result<(), Error> {
    let data = message_data(cis, md.size, md.compressed)?;
    let packet = CDemoPacket::parse_from_bytes(&data)?;
    let packet_data = packet.get_data();

    // extract packet data as bitvec to parse sub-type encoded header
    let mut bv = BitVec::<_, Lsb0>::from_slice(packet_data);

    // while there's at least one byte in the data packet
    while bv.len() > 7 {
        // read bit-variable command enum from bitvec directly
        let cmd = read_ubitvar(&mut bv) as i32;

        // hand bitvec back to input stream to continue parsing data
        let mut subpacket_cis = CodedInputStream::new(&mut bv);
        let size = subpacket_cis.read_raw_varint32()? as usize;
        let data = message_data(&mut subpacket_cis, size, false)?;

        // try to parse command enum from raw cmd
        if let Some(kind) = NET_Messages::from_i32(cmd) {
            println!("  [{:03?}] {:?} ({:?} bytes)", cmd, kind, size);

            match kind {
                NET_Messages::net_SpawnGroup_Load => {
                    let packet_message = CNETMsg_SpawnGroup_Load::parse_from_bytes(&data)?;
                },
                NET_Messages::net_Tick => {
                    let packet_message = CNETMsg_Tick::parse_from_bytes(&data)?;
                },
                _ => ()
            }

        } else if let Some(kind) = SVC_Messages::from_i32(cmd) {
            println!("  [{:03?}] {:?} ({:?} bytes)", cmd, kind, size);

            match kind {
                SVC_Messages::svc_ClearAllStringTables => {
                    let packet_message = CSVCMsg_ClearAllStringTables::parse_from_bytes(&data)?;
                },
                SVC_Messages::svc_ServerInfo => {
                    println!("trying to parse server message");
                    let packet_message = CSVCMsg_ServerInfo::parse_from_bytes(&data)?;
                    println!("{:?}", packet_message);
                },
                SVC_Messages::svc_UserMessage => {
                    let packet_message = CSVCMsg_UserMessage::parse_from_bytes(&data)?;
                },
                _ => ()
            }

        } else {
            println!("  [{:03?}] Unknown Message ({:?} bytes)", cmd, size);
        };
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("replays/example.dem")?;
    let mut replay = CodedInputStream::new(&mut file);

    // File header
    let engine_type = replay.read_raw_bytes(8)?;
    let _fileinfo_packet_pos = replay.read_fixed32()?;
    let _spawngroups_packet_pos = replay.read_fixed32()?;
    println!("Replay Type: {}", std::str::from_utf8(&engine_type).unwrap_or("Unknown"));

    for i in 0..30 {
        // next packet metadata
        let metadata = read_demo_message_metadata(&mut replay)?;
        println!("[{:?}] {:?}", i, metadata);

        match metadata.kind {
            EDemoCommands::DEM_ClassInfo => 
                print_message::<CDemoClassInfo>(&mut replay, metadata, false),
            EDemoCommands::DEM_CustomData => 
                print_message::<CDemoCustomData>(&mut replay, metadata, true),
            EDemoCommands::DEM_FileHeader => 
                print_message::<CDemoFileHeader>(&mut replay, metadata, true),
            EDemoCommands::DEM_FullPacket => 
                print_message::<CDemoFullPacket>(&mut replay, metadata, false),
            EDemoCommands::DEM_Packet => 
                // print_message::<CDemoPacket>(&mut replay, metadata, false),
                print_packet_message(&mut replay, metadata),
            EDemoCommands::DEM_SendTables => 
                print_message::<CDemoSendTables>(&mut replay, metadata, false),
            EDemoCommands::DEM_SignonPacket => 
                // print_message::<CDemoPacket>(&mut replay, metadata, false),
                print_packet_message(&mut replay, metadata),
            EDemoCommands::DEM_SyncTick => 
                print_message::<CDemoSyncTick>(&mut replay, metadata, true),
            EDemoCommands::DEM_StringTables => 
                print_message::<CDemoStringTables>(&mut replay, metadata, false),
            unknown => {
                println!("Unknown Message '{:?}'", unknown);
                Ok(())
            }
        }?;
    }

    Ok(())
}
