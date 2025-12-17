use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::{ApiResult, ApiSuccess};
use serde::{Deserialize, Serialize};

impl ApiResult for DeletePageRuleResult {}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeletePageRuleResult {
    pub id: String,
}

#[derive(Debug)]
pub struct DeletePageRule<'a> {
    pub zone_identifier: &'a str,
    pub rule_identifier: &'a str,
}

impl EndpointSpec for DeletePageRule<'_> {
    type JsonResponse = DeletePageRuleResult;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "zones/{}/pagerules/{}",
            self.zone_identifier, self.rule_identifier
        )
    }
}
