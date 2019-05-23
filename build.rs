extern crate protoc_rust_grpc;
extern crate dirs;

use std::path::PathBuf;
use std::env;

// TODO do not use user specific folders
fn main() {
    let current_dir = env::current_dir().unwrap();
    let mut proto_root = PathBuf::new();
    proto_root.push(current_dir);
    proto_root.push("src");

    let mut google_path = PathBuf::new();
    google_path.push(dirs::home_dir().unwrap().to_str().unwrap());
    google_path.push("go");
    google_path.push("include");

    let input_files_ttn = vec![
        "github.com/TheThingsNetwork/api/handler/handler.proto",
        "github.com/TheThingsNetwork/api/trace/trace.proto",
        "github.com/TheThingsNetwork/api/protocol/protocol.proto",
        "github.com/TheThingsNetwork/api/broker/broker.proto",
        "github.com/TheThingsNetwork/api/api.proto",
        "github.com/TheThingsNetwork/api/protocol/lorawan/device.proto",
        "github.com/TheThingsNetwork/api/protocol/lorawan/lorawan.proto",
        "github.com/TheThingsNetwork/api/gateway/gateway.proto",
        "github.com/TheThingsNetwork/api/trace/trace.proto",
    ];
    let input_files_google = vec![
        "google/api/annotations.proto",
        "google/api/http.proto",
        "google/protobuf/empty.proto",
    ];

    let mut input_raw = vec![];

    let mut input_files_ttn_full: Vec<String> = input_files_ttn
        .iter()
        .map(|val| {
            proto_root.join(val).into_os_string().into_string().unwrap()
        })
        .collect();
    let mut input_files_google_full: Vec<String> = input_files_google
        .iter()
        .map(|val| {
            google_path
                .join(val)
                .into_os_string()
                .into_string()
                .unwrap()
        })
        .collect();

    input_raw.append(&mut input_files_ttn_full);
    input_raw.append(&mut input_files_google_full);

    let input: Vec<&str> = input_raw.iter().map(|val| val.as_str()).collect();

    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: proto_root.to_str().unwrap(),
        includes: &[proto_root.to_str().unwrap(), google_path.to_str().unwrap()],
        input: &input,
        rust_protobuf: true, // also generate protobuf messages, not just services
        ..Default::default()
    }).expect("protoc-rust-grpc");
}
