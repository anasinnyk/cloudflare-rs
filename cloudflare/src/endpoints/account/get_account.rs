use super::Account;

use crate::framework::endpoint::{EndpointSpec, Method};

use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Get Account
/// <https://developers.cloudflare.com/api/resources/accounts/methods/get/>
#[derive(Debug)]
pub struct GetAccount<'a> {
    pub identifier: &'a str,
}

impl EndpointSpec for GetAccount<'_> {
    type JsonResponse = Account;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("accounts/{}", self.identifier)
    }
}
