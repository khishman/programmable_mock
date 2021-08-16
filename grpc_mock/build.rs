
fn main() {
    tonic_build::configure()
        .compile(&["proto/grpcmock.proto"], &["proto"])
        .unwrap();
}
