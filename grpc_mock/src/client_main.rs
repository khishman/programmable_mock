use std::env;
use tonic::Request;

pub mod grpcmock {
    tonic::include_proto!("grpcmock");
}

use grpcmock::grpc_mock_client::GrpcMockClient;
use grpcmock::ClientRequest;

pub async fn run_basic_req_resp_test() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut client 
        = GrpcMockClient::connect(String::from(&args[1])).await?;

    let resp = client.basic_req_resp(Request::new(
       ClientRequest {
           num_resp: 1
       } 
    )).await?;

    println!("{:?}", resp.into_inner());

    Ok(())
}

pub async fn run_test_both_stream() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut client 
        = GrpcMockClient::connect(String::from(&args[1])).await?;

    let outbound = async_stream::stream! {
        for _ in 0..10 {
            yield ClientRequest {
                num_resp: 1,
            };
        }
    };

    let response = client.bidirectional(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    while let Some(note) = inbound.message().await? {
        println!("got ServerResponse = {:?}", note);
    }

    Ok(())
}

pub async fn run_test_client_stream() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut client 
        = GrpcMockClient::connect(String::from(&args[1])).await?;


    let mut stream = client
    .server_stream_resp(Request::new(
        ClientRequest { num_resp: 10 } 
    ))
    .await?
    .into_inner();

    while let Some(serv_resp) = stream.message().await? {
        println!("ServerResponse stream item = {:?}", serv_resp);
    }

    Ok(())
}

pub async fn run_test_serv_stream() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut client 
        = GrpcMockClient::connect(String::from(&args[1])).await?;


    let mut stream = client
    .server_stream_resp(Request::new(
        ClientRequest { num_resp: 10 } 
    ))
    .await?
    .into_inner();

    while let Some(serv_resp) = stream.message().await? {
        println!(" stream item = {:?}", serv_resp);
    }

    Ok(())
}