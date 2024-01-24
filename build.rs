fn main() {
    tonic_build::configure()
        .build_server(true)
        .out_dir("examples/protos/") // you can change the generated code's location
        .compile(
            &["examples/protos/zkp_protocol.proto"],
            &["examples/protos/"], // specify the root location to search proto dependencies
        )
        .unwrap();
}
