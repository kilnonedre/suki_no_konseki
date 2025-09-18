use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, time::Duration};

#[derive(Debug, Serialize)]
pub struct EvaluateReq<'a> {
    pub user_id: Cow<'a, str>,
    pub resource_type: Cow<'a, str>,
    pub resource_id: Cow<'a, str>,
    pub action: Cow<'a, str>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EvaluateResp {
    pub allow: bool,
    pub visible_fields: Vec<String>,
    pub masked_fields: Vec<String>,
}

#[derive(Clone)]
pub struct PolicyClient {
    base: String,
    http: reqwest::Client,
}

impl PolicyClient {
    pub fn new(base: impl Into<String>) -> Result<Self> {
        let mut base = base.into();
        // 规范化：去掉末尾斜杠
        if base.ends_with('/') {
            base.pop();
        }

        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(5)) // 统一超时
            // .user_agent("policy-client/0.1.0") // 可选
            // .use_rustls_tls() // 若你更偏好 rustls
            .build()
            .context("build reqwest client")?;

        Ok(Self { base, http })
    }

    pub async fn evaluate(&self, req: &EvaluateReq<'_>) -> Result<EvaluateResp> {
        let url = format!("{}/v1/evaluate", self.base);
        let resp = self
            .http
            .post(url)
            .json(req)
            .send()
            .await
            .context("send request to policy service")?
            .error_for_status()
            .context("policy service returned non-2xx")?
            .json::<EvaluateResp>()
            .await
            .context("deserialize EvaluateResp")?;

        Ok(resp)
    }
}
