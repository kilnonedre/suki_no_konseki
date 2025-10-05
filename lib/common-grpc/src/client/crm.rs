use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::Request;

use crate::crm::greeter_client::GreeterClient;
use crate::crm::HelloRequest;

/// 全局单例 client
static GLOBAL_CLIENT: Lazy<Mutex<Option<MyGreeterClient>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct MyGreeterClient {
    inner: GreeterClient<Channel>,
}

impl MyGreeterClient {
    /// 内部 connect，只在第一次调用时执行
    async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GreeterClient::connect(addr.to_string()).await?;
            *lock = Some(MyGreeterClient { inner: client });
        }
        Ok(())
    }

    /// 透明调用 say_hello
    pub async fn say_hello(name: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // 自动初始化
        Self::init("http://[::1]:50051").await?;

        // 获取 client
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();

        let request = Request::new(HelloRequest {
            name: name.to_string(),
        });

        let response = client.inner.say_hello(request).await?;
        Ok(response.into_inner().message)
    }
}
