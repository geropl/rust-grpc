// extern crate protoc_rust_grpc;

// fn main() {
//     protoc_rust_grpc::run(protoc_rust_grpc::Args {
//         out_dir: "src/gen",
//         includes: &[,
//         input: &["proto/helloworld.proto"],
//         rust_protobuf: true, // also generate protobuf messages, not just services
//         ..Default::default()
//     }).expect("protoc-rust-grpc");
// }

extern crate protoc_grpcio;

fn main() {
    let proto_root = "src/protos";
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(
        &["src/protos/helloworld.proto"],
        &[proto_root],
        &proto_root
    ).expect("Failed to compile gRPC definitions!");
}