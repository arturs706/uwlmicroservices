fn main() {
    tonic_build::configure()
        .compile(
            &["proto/users.proto"],
            &["proto"],
        )
        .expect("compile gRPC proto files.");
}
