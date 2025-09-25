use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ResponseListT<T> {
    pub list: Vec<T>,
    pub total_count: u64,
}

impl<T> ResponseListT<T> {
    pub fn new(list: Vec<T>, total_count: u64) -> Self {
        Self { list, total_count }
    }
}
