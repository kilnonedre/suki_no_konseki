use tonic::{transport::Server, Request, Response, Status};

use common_grpc::crm::{
    login_server::{Login, LoginServer},
    LoginVerifyReq, LoginVerifyResp,
};
#[derive(Debug, Default)]
pub struct CrmServer {}

#[tonic::async_trait]
impl Login for CrmServer {
    async fn verify(
        &self,
        request: Request<LoginVerifyReq>,
    ) -> Result<Response<LoginVerifyResp>, Status> {
        println!("Got a request: {:?}", request);

        let reply = LoginVerifyResp {
            id: "00000000-0000-0000-0000-000000000001".to_string(),
        };

        Ok(Response::new(reply))
    }
}

pub async fn start_grpc_server(addr: &str) {
    let addr = addr.parse().expect("Failed to parse socket address");
    let crm = CrmServer::default();

    if let Err(e) = Server::builder()
        .add_service(LoginServer::new(crm))
        .serve(addr)
        .await
    {
        eprintln!("gRPC server error: {}", e);
    }
}
