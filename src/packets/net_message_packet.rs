use bitvec::prelude::*;
use bitvec::store::BitStore;

mod protobufs;
use crate::protobufs::networkbasetypes::*;
use crate::protobufs::netmessages::*;

mod bitreaders;
use crate::bitreaders::read_ubitvar;

// fn print_packet_message(cis: &mut CodedInputStream, md: CDemoPacketMetadata) 
// -> Result<(), Error> {
//     let data = message_data(cis, md.size, md.compressed)?;
//     let packet = CDemoPacket::parse_from_bytes(&data)?;
//     let packet_data = packet.get_data();
//
//     // extract packet data as bitvec to parse sub-type encoded header
//     let mut bv = BitVec::<_, Lsb0>::from_slice(packet_data);
//
//     // while there's at least one byte in the data packet
//     while bv.len() > 7 {
//         // read bit-variable command enum from bitvec directly
//         let cmd = read_ubitvar(&mut bv) as i32;
//
//         // hand bitvec back to input stream to continue parsing data
//         let mut subpacket_cis = CodedInputStream::new(&mut bv);
//         let size = subpacket_cis.read_raw_varint32()? as usize;
//         let data = message_data(&mut subpacket_cis, size, false)?;
//
//         // try to parse command enum from raw cmd
//         if let Some(kind) = NET_Messages::from_i32(cmd) {
//             println!("  [{:03?}] {:?} ({:?} bytes)", cmd, kind, size);
//
//             match kind {
//                 NET_Messages::net_SpawnGroup_Load => {
//                     let packet_message = CNETMsg_SpawnGroup_Load::parse_from_bytes(&data)?;
//                 },
//                 NET_Messages::net_Tick => {
//                     let packet_message = CNETMsg_Tick::parse_from_bytes(&data)?;
//                 },
//                 _ => ()
//             }
//
//         } else if let Some(kind) = SVC_Messages::from_i32(cmd) {
//             println!("  [{:03?}] {:?} ({:?} bytes)", cmd, kind, size);
//
//             match kind {
//                 SVC_Messages::svc_ClearAllStringTables => {
//                     let packet_message = CSVCMsg_ClearAllStringTables::parse_from_bytes(&data)?;
//                 },
//                 SVC_Messages::svc_ServerInfo => {
//                     let packet_message = CSVCMsg_ServerInfo::parse_from_bytes(&data)?;
//                 },
//                 SVC_Messages::svc_UserMessage => {
//                     let packet_message = CSVCMsg_UserMessage::parse_from_bytes(&data)?;
//                 },
//                 _ => ()
//             }
//
//         } else {
//             println!("  [{:03?}] Unknown Message ({:?} bytes)", cmd, size);
//         };
//     }
//
//     Ok(())
// }

