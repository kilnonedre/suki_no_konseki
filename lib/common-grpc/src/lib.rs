mod client;

pub use client::crm::MyGreeterClient;

pub mod crm {
    tonic::include_proto!("crm");
}
