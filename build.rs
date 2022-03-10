fn main() {
    tonic_build::configure()
        .build_server(true)
        // TODO build_client only needed for tests
        .build_client(true)
        .compile(
            // list of protos
            &[
                "deps/protos/api_garble/api.proto",
                "deps/protos/api_garble/garble_routes.proto",
            ],
            // includes
            &["deps/protos"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
