use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub page: u64,
    pub size: u64,
    pub total_page: u64,
    pub total_element: u64,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResponseListT<T> {
    pub items: Vec<T>,
    pub page_info: PageInfo,
}

impl<T> ResponseListT<T> {
    pub fn new(items: Vec<T>, page_info: PageInfo) -> Self {
        Self { items, page_info }
    }
}
