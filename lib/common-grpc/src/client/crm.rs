use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::Request;

use crate::crm::login_client::LoginClient;
use crate::crm::LoginVerifyReq;

/// 全局单例 client
static GLOBAL_CLIENT: Lazy<Mutex<Option<LoginGrpcClient>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct LoginGrpcClient {
    inner: LoginClient<Channel>,
}

impl LoginGrpcClient {
    /// 内部 connect，只在第一次调用时执行
    async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = LoginClient::connect(addr.to_string()).await?;
            *lock = Some(LoginGrpcClient { inner: client });
        }
        Ok(())
    }

    /// 透明调用 say_hello
    pub async fn verify(
        account: &String,
        password: &String,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // 自动初始化
        Self::init("http://[::1]:50051").await?;

        // 获取 client
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();

        let request = Request::new(LoginVerifyReq {
            account: account.clone(),
            password: password.clone(),
        });

        let response = client.inner.verify(request).await?;
        Ok(response.into_inner().id)
    }
}
