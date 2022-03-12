#![cfg_attr(debug_assertions, allow(unused_imports))]

use std::fs::File;
use std::marker::PhantomData;
use std::io::{Read,Error,ErrorKind};

use std::io::BufRead;
use protobuf::{CodedInputStream,ProtobufEnum,ProtobufResult,Message};
use protobuf::error::*;
use snap;

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

    // try to parse command enum from raw cmd
    let kind = match EDemoCommands::from_i32(raw_cmd) {
        Some(cmd) => cmd,
        None      => EDemoCommands::DEM_Error,
    };

    Ok(CDemoPacketMetadata { kind, compressed, tick, size })
}


fn message_data(cis: &mut CodedInputStream, size: usize, compressed: bool) -> Result<Vec<u8>, std::io::Error> {
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

fn print_message<T: protobuf::Message>(cis: &mut CodedInputStream, md: CDemoPacketMetadata, print: bool) -> Result<(), std::io::Error> {
    // read message from decomprssed buffer
    let data = message_data(cis, md.size, md.compressed)?;
    let message = T::parse_from_bytes(&data)?;
    if print { println!("{:?}", message); }
    Ok(())
}

fn print_packet_message(cis: &mut CodedInputStream, md: CDemoPacketMetadata, print: bool) -> Result<(), std::io::Error> {
    // read message from decomprssed buffer
    let data = message_data(cis, md.size, md.compressed)?;
    let message = CDemoPacket::parse_from_bytes(&data)?;
    let mut packet_data = message.get_data();
    let mut packet = CodedInputStream::from_bytes(&mut packet_data);

    packet.push_limit(md.size as u64);
    while packet.bytes_until_limit() > 0 {

        let cmd  = packet.read_raw_varint32()? as i32;
        let size = packet.read_raw_varint32()? as usize;

        if size > packet.bytes_until_limit() as usize {
            break;
        }

        // try to parse command enum from raw cmd
        if let Some(kind) = NET_Messages::from_i32(cmd) {
            let data = message_data(&mut packet, size, false)?;
            println!("[{:?}] {:?} ({:?})", size, kind, cmd);

            match kind {
                NET_Messages::net_SpawnGroup_Load => {
                    let packet_message = CNETMsg_SpawnGroup_Load::parse_from_bytes(&data);
                    println!("{:?}", packet_message);
                },
                _ => ()
            }
            
        } else if let Some(kind) = SVC_Messages::from_i32(cmd) {
            println!("[{:?}] {:?} ({:?})", size, kind, cmd);

            match kind {
                SVC_Messages::svc_UserMessage => {
                    let packet_message = CSVCMsg_UserMessage::parse_from_bytes(&data);
                    println!("{:?}", packet_message);
                },
                _ => ()
            }

        } else {
            println!("[{:?}] Unknown SVC_Message ({:?})", size, cmd);
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
                print_packet_message(&mut replay, metadata, false),
            EDemoCommands::DEM_SendTables => 
                print_message::<CDemoSendTables>(&mut replay, metadata, false),
            EDemoCommands::DEM_SignonPacket => 
                print_message::<CDemoPacket>(&mut replay, metadata, false),
            EDemoCommands::DEM_SyncTick => 
                print_message::<CDemoSyncTick>(&mut replay, metadata, true),
            EDemoCommands::DEM_StringTables => 
                print_message::<CDemoStringTables>(&mut replay, metadata, false),
            _ => Ok(())
        }?;
    }

    Ok(())
}
