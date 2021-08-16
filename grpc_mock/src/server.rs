use std::pin::Pin;

use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Response, Status};

pub mod grpcmock {
    tonic::include_proto!("grpcmock");
}

use grpcmock::ServerResponse;
use grpcmock::ClientRequest;
use grpcmock::grpc_mock_server::GrpcMock;
use grpcmock::grpc_mock_server::GrpcMockServer;

#[derive(Debug)]
pub struct GrpcMockService {}

impl GrpcMockService {
    fn new() -> GrpcMockService {
        GrpcMockService{}
    }
}
pub struct RespGen {
    // Stateless
    // Total number of responses to send back
    total_resp: u32,
    // Number of requests recieved from the client
    num_client_reqs: u32,

    // Stateful
    // Number of responses remaining
    num_resp: u32,
}

impl RespGen {

    fn new(num_resp: u32, num_client_reqs: u32) -> RespGen {
        RespGen {
            total_resp: num_resp,
            num_resp: num_resp,
            num_client_reqs: num_client_reqs,
        }
    }
}

impl Iterator for RespGen {
    type Item = ServerResponse;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> { 
        match self.num_resp {
            0 => None,
            _ => {
                let r = Some(
                    ServerResponse {
                        resp_idx: self.total_resp - self.num_resp,
                        num_reqs: self.num_client_reqs,
                });
                self.num_resp = self.num_resp - 1;
                r
            }
        }
    }
}

#[tonic::async_trait]
impl GrpcMock for GrpcMockService {
    async fn basic_req_resp(
        &self,
        request: tonic::Request<ClientRequest>,
    ) -> Result<tonic::Response<ServerResponse>, tonic::Status> {
        let m = request.into_inner();
        
        match m.num_resp {
            1 => {
                Ok(Response::new((RespGen::new(1, 1)).next().unwrap()))
            }
            _ => {
                Err(tonic::Status::failed_precondition(
            "client was only expected to request 1 response"))
            }
        }
    }

    #[doc = "Server streaming response type for the ServerStreamResp method."]
    type ServerStreamRespStream = ReceiverStream<Result<ServerResponse, Status>>;
    async fn server_stream_resp(
        &self,
        request: tonic::Request<ClientRequest>,
    ) -> Result<tonic::Response<Self::ServerStreamRespStream>, tonic::Status> {
        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            let m = request.into_inner();

            let g = RespGen::new(m.num_resp, 1);
            for i in g {
                tx.send(Ok(i)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn client_stream_resp(
        &self,
        request: tonic::Request<tonic::Streaming<ClientRequest>>,
    ) -> Result<tonic::Response<ServerResponse>, tonic::Status> {
        let mut stream = request.into_inner();

        let mut num_c_reqs = 0;
        while let Some(_) = stream.next().await {
            num_c_reqs = num_c_reqs + 1
        }

        println!("client_stream_resp: recieved {} requests", num_c_reqs);

        // This is a spot where num_client_reqs is useful
        Ok(Response::new((RespGen::new(1, num_c_reqs)).next().unwrap()))
    }

    #[doc = "Server streaming response type for the TwoWayStream method."]
    type BidirectionalStream 
        = Pin<Box<dyn Stream<Item = Result<ServerResponse, Status>> + Send + Sync + 'static>>;

    async fn bidirectional(
        &self,
        request: tonic::Request<tonic::Streaming<ClientRequest>>,
    ) -> Result<tonic::Response<Self::BidirectionalStream>, tonic::Status> {
    let mut stream = request.into_inner();

    let mut resp_idx = 0;
    let output = async_stream::try_stream! {
        while let Some(_) = stream.next().await {
            // The response index as well as the number of client requests match here
            yield ServerResponse { resp_idx, num_reqs: resp_idx };
            resp_idx = resp_idx + 1;
        }
    };

    Ok(Response::new(Box::pin(output)
        as Self::BidirectionalStream))
    }
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    println!("GrpcMockServer listening on: {}", addr);

    let grpc_mock = GrpcMockService::new();

    let svc = GrpcMockServer::new(grpc_mock);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_server().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::server::RespGen;

    #[test]
    fn zero_msgs() {
        let mut g = RespGen::new(0, 0);
        assert_eq!(g.next(), None)
    }

    #[test]
    fn one_msg() {
        let mut g = RespGen::new(1, 0);
        let sresp 
            = g.next().expect("expected at least 1 twoway");

        // we took 1 response with .next(), so 0 should remain
        assert_eq!(sresp.resp_idx, 0);
    }

    #[test]
    fn ten_msgs() {
        let g = RespGen::new(10, 0);
        let mut expected_idx: u32 = 0;

        for i in g {
            assert_eq!(i.resp_idx, expected_idx);
            expected_idx = expected_idx + 1;
        }
    }
}