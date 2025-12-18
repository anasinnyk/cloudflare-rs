use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

pub mod tokens;
pub use tokens::TokenVerification;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TokenVerify {
    /// Token identifier.
    pub id: String,
    /// Token status
    pub status: String,
}

impl ApiResult for TokenVerify {}
