use crate::endpoints::page_rule::types::PageRule;

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;

#[derive(Debug)]
pub struct GetPageRule<'a> {
    pub zone_identifier: &'a str,
    pub rule_identifier: &'a str,
}

impl EndpointSpec for GetPageRule<'_> {
    type JsonResponse = PageRule;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "zones/{}/pagerules/{}",
            self.zone_identifier, self.rule_identifier
        )
    }
}
