# lothars

Source demo file parser in rust

## Status

Currently extermely experimental. I'm using this as a pet project to learn 
some rust and the protobuf format. It would be lovely if this could grow 
into a competent parser, but that's not my first priority.

### Features

|Capability|Status|
|---|:--|
|Reading protobuf messages into rust|:heavy_check_mark:|
|Parsing packets from data messages|:heavy_check_mark:|
|Storing packet string tables and interpolating messages|:x:|
|Providing an interface for extracting file contents|:x:|

### Performance

This project isn't going to break any records. Currently, it's about 5x slower
than [manta](https://github.com/dotabuff/manta) while still missing quite a bit
of the functionality to fully parse the files.

## Build & Run

The SteamDatabase's protobuf registry repo is included as a submodule. 
Protobuf definitions are built from these registry files.

Pure rust protobuf and codegen crates were used, so building and running 
should be as simple as possible.

```sh
git clone --recurse-submodules git://github.com/dgkf/lothars
cd lothars
cargo run
```
