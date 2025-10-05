use common_grpc::MyGreeterClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reply = MyGreeterClient::say_hello("Alice").await.expect("msg");
    println!("Reply: {}", reply);

    let reply2 = MyGreeterClient::say_hello("Bob").await.expect("msg");
    println!("Reply: {}", reply2);

    Ok(())
}
