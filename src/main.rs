#![cfg_attr(debug_assertions, allow(unused_imports))]

use std::fs::File;
use protobuf::CodedInputStream;

mod protobufs;
mod packets;
use crate::packets::*;

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
        let metadata = CDemoPacketMetadata::read_from(&mut replay)?;
        println!("[{:?}] {:?}", i, metadata);

        // read next packet
        let message = metadata.read_message_from(&mut replay)?;
    }

    Ok(())
}
