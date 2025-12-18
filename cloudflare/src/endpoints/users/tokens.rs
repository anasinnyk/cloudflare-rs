use super::TokenVerify;
use crate::framework::endpoint::{serialize_query, EndpointSpec, Method};
use crate::framework::OrderDirection;

use crate::framework::response::ApiSuccess;
use serde::Serialize;

pub struct TokenVerification {}

impl EndpointSpec for TokenVerification {
    type JsonResponse = TokenVerify;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "users/tokens/verify".to_string()
    }
}
