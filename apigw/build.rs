fn main() {
    tonic_build::configure()
        .compile(
            &[
                "proto/properties.proto",
                "proto/users.proto",
            ],
            &["proto"],
        )
        .expect("compile gRPC proto files.");
}
