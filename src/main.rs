#![cfg_attr(debug_assertions, allow(unused_imports))]

use std::fs::File;
use protobuf::CodedInputStream;

mod protobufs;
mod packets;
mod utils;

use crate::packets::messages::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("replays/example.dem")?;
    let mut replay = CodedInputStream::new(&mut file);

    // File header
    let engine_type = replay.read_raw_bytes(8)?;
    let _fileinfo_packet_pos = replay.read_fixed32()?;
    let _spawngroups_packet_pos = replay.read_fixed32()?;
    println!("Replay Type: {}", std::str::from_utf8(&engine_type).unwrap_or("Unknown"));

    loop {
        match read_next_message_from(&mut replay) {
            Ok(message) => println!("{message}"),
            _           => break,
        }
    }

    Ok(())
}
