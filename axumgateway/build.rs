fn main() {
    tonic_build::configure()
        .compile(
            &[
                "proto/payments.proto",
                "proto/properties.proto",
                "proto/helloworld.proto",
            ],
            &["proto"],
        )
        .expect("compile gRPC proto files.");
}
