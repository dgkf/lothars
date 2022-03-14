use std::io::{Read,Error,ErrorKind};
use protobuf::{CodedInputStream,Message,ProtobufResult,ProtobufEnum};
use snap::raw::Decoder;

use crate::protobufs::demo::*;

#[derive(Debug)]
pub struct CDemoPacketMetadata {
    pub kind: EDemoCommands,
    pub compressed: bool,
    pub tick: u32,
    pub size: usize,
}

impl CDemoPacketMetadata {
    pub fn new_message(&self) -> Result<Box<dyn protobuf::Message>, Error> {
        let e = Err(Error::new(
            ErrorKind::Other, 
            format!("Failed to initialize message for EDemoCommands::{:?}", self.kind)
        ));

        match self.kind {
            EDemoCommands::DEM_ClassInfo           => return Ok(Box::new(CDemoClassInfo::new())),
            EDemoCommands::DEM_ConsoleCmd          => return Ok(Box::new(CDemoConsoleCmd::new())),
            EDemoCommands::DEM_CustomData          => return Ok(Box::new(CDemoCustomData::new())),
            EDemoCommands::DEM_CustomDataCallbacks => return Ok(Box::new(CDemoCustomDataCallbacks::new())),
            EDemoCommands::DEM_FileHeader          => return Ok(Box::new(CDemoFileHeader::new())),
            EDemoCommands::DEM_FileInfo            => return Ok(Box::new(CDemoFileInfo::new())),
            EDemoCommands::DEM_FullPacket          => return Ok(Box::new(CDemoFullPacket::new())),
            EDemoCommands::DEM_SendTables          => return Ok(Box::new(CDemoSendTables::new())),
            EDemoCommands::DEM_SignonPacket        => return Ok(Box::new(CDemoPacket::new())),
            EDemoCommands::DEM_Stop                => return Ok(Box::new(CDemoStop::new())),
            EDemoCommands::DEM_StringTables        => return Ok(Box::new(CDemoStringTables::new())),
            EDemoCommands::DEM_SyncTick            => return Ok(Box::new(CDemoSyncTick::new())),
            EDemoCommands::DEM_UserCmd             => return Ok(Box::new(CDemoUserCmd::new())),
            EDemoCommands::DEM_Packet              => return Ok(Box::new(CDemoPacket::new())),
            _                                      => return e
        }
    }

    pub fn read_from(cis: &mut CodedInputStream) -> ProtobufResult<CDemoPacketMetadata> {
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

        Ok(CDemoPacketMetadata{ kind, compressed, tick, size })
    }

    pub fn read_message_from(&self, cis: &mut CodedInputStream) -> ProtobufResult<Box<dyn protobuf::Message>> {
        let mut buffer: Vec<u8> = vec![0; self.size];
        cis.read_exact(&mut buffer).unwrap();

        let decomp_buffer = if self.compressed {
            // decompressed snappy-compression buffer
            Decoder::new().decompress_vec(&buffer).unwrap()
        } else {
            // already decompressed
            buffer
        };

        self.parse_message_from_bytes(&decomp_buffer)
    }

    pub fn parse_message_from_bytes(&self, data: &[u8]) -> ProtobufResult<Box<dyn protobuf::Message>> {
        let mut msg = self.new_message()?;
        let res = msg.merge_from_bytes(data);

        match res {
            Ok(_)  => return Ok(msg),
            Err(e) => return Err(e),
        }
    }
}
