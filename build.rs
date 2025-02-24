// Custom build script to generate the Rust code from .proto and .fbs
// files programmatically. This script uses the `tonic-build` crate to generate the Rust
// code from the .proto files and the `flatc-rust` crate to generate the
// Rust code from the .fbs files.
//
// The generated Rust code is written to the `src/generated` and `src/flatbuffers`

//Path is needed to generate the Rust code from .proto and .fbs files
//as the inputs and out_dire requires teh argument of std::path::Path
use std::path::Path;

fn main() {
    // Compile the proto files and generate the Rust code for the gRPC
    tonic_build::compile_protos("src/grpc/coordinator.proto").unwrap();
    // Compile the schema files and generate the Rust code for the flatbuffers
    flatc_rust::run(flatc_rust::Args {
        inputs: &[Path::new("src/flatbuffers/messages.fbs")],
        out_dir: Path::new("src/generated"),
        ..Default::default()
    })
    .expect("flatc-rust failed to compile the schema and the code generation failed as well");
}
