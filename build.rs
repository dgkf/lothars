// use std::fs;

fn main() {

    // println!("cargo:rerun-if-changed=protobufs/dota2/*.proto");

    // let protos_paths = fs::read_dir("protobufs/dota2/demo.").unwrap()
    //     .map(|p| p.unwrap().path());

    protobuf_codegen_pure::Codegen::new() 
        .customize(protobuf_codegen_pure::Customize {
            expose_oneof: Some(true),
            expose_fields: Some(true),
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .out_dir("src/protobufs")
        .inputs(&[
            "protobufs/dota2/demo.proto", 
            "protobufs/dota2/network_connection.proto",
            "protobufs/dota2/networkbasetypes.proto", 
            "protobufs/dota2/netmessages.proto"
        ])
        .include("protobufs/dota2")
        .run()
        .expect("failed to codegen protobuf message definitions");
}
