pub mod client_main;
pub mod server;

pub mod grpcmock {
    tonic::include_proto!("grpcmock");
}

pub use grpcmock::grpc_mock_client;

pub use server::run_server;

pub use client_main::run_basic_req_resp_test;
pub use client_main::run_test_both_stream;
pub use client_main::run_test_client_stream;
pub use client_main::run_test_serv_stream;



