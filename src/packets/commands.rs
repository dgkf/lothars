use protobuf::Message;
use crate::protobufs::demo::*;
use crate::protobufs::netmessages::*;
use crate::protobufs::networkbasetypes::*;

#[derive(Debug)]
pub enum DemoCommand {
    EDemo(EDemoCommands),
    NET(NET_Messages),
    SVC(SVC_Messages),
}


#[derive(Debug)]
pub enum DemoMessage<T> {
    Standalone(T),
    Packet(Box<CDemoPacket>),
    FullPacket(Box<CDemoFullPacket>),
}

pub fn new_message_from_command(cmd: DemoCommand) -> DemoMessage<Box<dyn Message>> {
    match cmd {
        DemoCommand::EDemo(c) => match c {
            EDemoCommands::DEM_ClassInfo                 => DemoMessage::Standalone(Box::new(CDemoClassInfo::new())),
            EDemoCommands::DEM_ConsoleCmd                => DemoMessage::Standalone(Box::new(CDemoConsoleCmd::new())),
            EDemoCommands::DEM_CustomData                => DemoMessage::Standalone(Box::new(CDemoCustomData::new())),
            EDemoCommands::DEM_CustomDataCallbacks       => DemoMessage::Standalone(Box::new(CDemoCustomDataCallbacks::new())),
            EDemoCommands::DEM_FileInfo                  => DemoMessage::Standalone(Box::new(CDemoFileInfo::new())),
            EDemoCommands::DEM_FileHeader                => DemoMessage::Standalone(Box::new(CDemoFileHeader::new())),
            EDemoCommands::DEM_FullPacket                => DemoMessage::FullPacket(Box::new(CDemoFullPacket::new())),
            EDemoCommands::DEM_Packet                    => DemoMessage::Packet(    Box::new(CDemoPacket::new())),
            EDemoCommands::DEM_SaveGame                  => DemoMessage::Standalone(Box::new(CDemoSaveGame::new())),
            EDemoCommands::DEM_SendTables                => DemoMessage::Standalone(Box::new(CDemoSendTables::new())),
            EDemoCommands::DEM_SignonPacket              => DemoMessage::Packet(    Box::new(CDemoPacket::new())),
            EDemoCommands::DEM_SpawnGroups               => DemoMessage::Standalone(Box::new(CDemoSpawnGroups::new())),
            EDemoCommands::DEM_Stop                      => DemoMessage::Standalone(Box::new(CDemoStop::new())),
            EDemoCommands::DEM_StringTables              => DemoMessage::Standalone(Box::new(CDemoStringTables::new())),
            EDemoCommands::DEM_SyncTick                  => DemoMessage::Standalone(Box::new(CDemoSyncTick::new())),
            EDemoCommands::DEM_UserCmd                   => DemoMessage::Standalone(Box::new(CDemoUserCmd::new())),
            _                                            => DemoMessage::Standalone(Box::new(CDemoStop::new())),
        },
        DemoCommand::NET(c) => match c {
            NET_Messages::net_NOP                        => DemoMessage::Standalone(Box::new(CNETMsg_NOP::new())),
            NET_Messages::net_Disconnect                 => DemoMessage::Standalone(Box::new(CNETMsg_Disconnect::new())),
            NET_Messages::net_SplitScreenUser            => DemoMessage::Standalone(Box::new(CNETMsg_SplitScreenUser::new())),
            NET_Messages::net_Tick                       => DemoMessage::Standalone(Box::new(CNETMsg_Tick::new())),
            NET_Messages::net_StringCmd                  => DemoMessage::Standalone(Box::new(CNETMsg_StringCmd::new())),
            NET_Messages::net_SetConVar                  => DemoMessage::Standalone(Box::new(CNETMsg_SetConVar::new())),
            NET_Messages::net_SignonState                => DemoMessage::Standalone(Box::new(CNETMsg_SignonState::new())),
            NET_Messages::net_SpawnGroup_Load            => DemoMessage::Standalone(Box::new(CNETMsg_SpawnGroup_Load::new())),
            NET_Messages::net_SpawnGroup_ManifestUpdate  => DemoMessage::Standalone(Box::new(CNETMsg_SpawnGroup_ManifestUpdate::new())),
            NET_Messages::net_SpawnGroup_SetCreationTick => DemoMessage::Standalone(Box::new(CNETMsg_SpawnGroup_SetCreationTick::new())),
            NET_Messages::net_SpawnGroup_Unload          => DemoMessage::Standalone(Box::new(CNETMsg_SpawnGroup_Unload::new())),
            NET_Messages::net_SpawnGroup_LoadCompleted   => DemoMessage::Standalone(Box::new(CNETMsg_SpawnGroup_LoadCompleted::new())),
        },
        DemoCommand::SVC(c) => match c {
            SVC_Messages::svc_ServerInfo                 => DemoMessage::Standalone(Box::new(CSVCMsg_ServerInfo::new())),
            SVC_Messages::svc_FlattenedSerializer        => DemoMessage::Standalone(Box::new(CSVCMsg_FlattenedSerializer::new())),
            SVC_Messages::svc_ClassInfo                  => DemoMessage::Standalone(Box::new(CSVCMsg_ClassInfo::new())),
            SVC_Messages::svc_SetPause                   => DemoMessage::Standalone(Box::new(CSVCMsg_SetPause::new())),
            SVC_Messages::svc_CreateStringTable          => DemoMessage::Standalone(Box::new(CSVCMsg_CreateStringTable::new())),
            SVC_Messages::svc_UpdateStringTable          => DemoMessage::Standalone(Box::new(CSVCMsg_UpdateStringTable::new())),
            SVC_Messages::svc_VoiceInit                  => DemoMessage::Standalone(Box::new(CSVCMsg_VoiceInit::new())),
            SVC_Messages::svc_VoiceData                  => DemoMessage::Standalone(Box::new(CSVCMsg_VoiceData::new())),
            SVC_Messages::svc_Print                      => DemoMessage::Standalone(Box::new(CSVCMsg_Print::new())),
            SVC_Messages::svc_Sounds                     => DemoMessage::Standalone(Box::new(CSVCMsg_Sounds::new())),
            SVC_Messages::svc_SetView                    => DemoMessage::Standalone(Box::new(CSVCMsg_SetView::new())),
            SVC_Messages::svc_ClearAllStringTables       => DemoMessage::Standalone(Box::new(CSVCMsg_ClearAllStringTables::new())),
            SVC_Messages::svc_CmdKeyValues               => DemoMessage::Standalone(Box::new(CSVCMsg_CmdKeyValues::new())),
            SVC_Messages::svc_BSPDecal                   => DemoMessage::Standalone(Box::new(CSVCMsg_BSPDecal::new())),
            SVC_Messages::svc_SplitScreen                => DemoMessage::Standalone(Box::new(CSVCMsg_SplitScreen::new())),
            SVC_Messages::svc_PacketEntities             => DemoMessage::Standalone(Box::new(CSVCMsg_PacketEntities::new())),
            SVC_Messages::svc_Prefetch                   => DemoMessage::Standalone(Box::new(CSVCMsg_Prefetch::new())),
            SVC_Messages::svc_Menu                       => DemoMessage::Standalone(Box::new(CSVCMsg_Menu::new())),
            SVC_Messages::svc_GetCvarValue               => DemoMessage::Standalone(Box::new(CSVCMsg_GetCvarValue::new())),
            SVC_Messages::svc_StopSound                  => DemoMessage::Standalone(Box::new(CSVCMsg_StopSound::new())),
            SVC_Messages::svc_PeerList                   => DemoMessage::Standalone(Box::new(CSVCMsg_PeerList::new())),
            SVC_Messages::svc_PacketReliable             => DemoMessage::Standalone(Box::new(CSVCMsg_PacketReliable::new())),
            SVC_Messages::svc_HLTVStatus                 => DemoMessage::Standalone(Box::new(CSVCMsg_HLTVStatus::new())),
            SVC_Messages::svc_ServerSteamID              => DemoMessage::Standalone(Box::new(CSVCMsg_ServerSteamID::new())),
            SVC_Messages::svc_FullFrameSplit             => DemoMessage::Standalone(Box::new(CSVCMsg_FullFrameSplit::new())),
            SVC_Messages::svc_RconServerDetails          => DemoMessage::Standalone(Box::new(CSVCMsg_RconServerDetails::new())),
            SVC_Messages::svc_UserMessage                => DemoMessage::Standalone(Box::new(CSVCMsg_UserMessage::new())),
        }
    }
}
