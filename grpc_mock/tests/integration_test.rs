
use grpc_mock::grpc_mock_client::GrpcMockClient;

use grpc_mock::client_main::run_basic_req_resp_test;
use grpc_mock::client_main::run_test_serv_stream;
use grpc_mock::client_main::run_test_client_stream;
use grpc_mock::client_main::run_test_both_stream;

use grpc_mock::server::run_server;

#[tokio::test]
async fn run_server_and_tests() {

    tokio::spawn(async {
        run_server()
    });

    // Here we connect in a loop waiting for the server to start
    let mut tries = 1;

    let addr = String::from("http://[::1]:10000");
    let mut conn_success 
        = GrpcMockClient::connect(addr.clone()).await;
    loop {
        match conn_success {
            Err(_) => {
                println!("client: {:?}", conn_success);

                tries = tries + 1;

                // the math here is hard, this is probably wrong math
                // but who cares, shipping code to prod is more important
                // than spending time figuring out math
                if tries == 11 {
                    panic!("Could not connect in 10 attempts")
                }

                conn_success 
                    = GrpcMockClient::connect(addr.clone()).await;
            }
            _ => {
                break
            }
        }
    }


    println!("connection succeeded after {}", tries);

    println!("testing basic request response");
    run_basic_req_resp_test();

    println!("testing streaming server response");
    run_test_serv_stream();

    println!("testing streaming client request");
    run_test_client_stream();

    println!("testing both way streaming (echo)");
    run_test_both_stream();
}