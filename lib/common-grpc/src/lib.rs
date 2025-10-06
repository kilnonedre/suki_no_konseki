mod client;

pub use client::crm::LoginGrpcClient;

pub mod crm {
    tonic::include_proto!("crm");
}
