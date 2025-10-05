use tonic::{transport::Server, Request, Response, Status};

use common_grpc::crm::greeter_server::{Greeter, GreeterServer};
use common_grpc::crm::{HelloReply, HelloRequest};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

pub async fn start_grpc_server(addr: &str) {
    let addr = addr.parse().expect("Failed to parse socket address");
    let greeter = MyGreeter::default();

    if let Err(e) = Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await
    {
        eprintln!("gRPC server error: {}", e);
    }
}
