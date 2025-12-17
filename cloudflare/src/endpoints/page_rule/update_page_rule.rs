use crate::endpoints::page_rule::types::{
    PageRule, PageRuleAction, PageRuleStatus, PageRuleTarget,
};
use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct UpdatePageRuleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<PageRuleAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<PageRuleTarget>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PageRuleStatus>,
}

#[derive(Debug)]
pub struct UpdatePageRule<'a> {
    pub zone_identifier: &'a str,
    pub rule_identifier: &'a str,
    pub params: UpdatePageRuleParams,
}
impl EndpointSpec for UpdatePageRule<'_> {
    type JsonResponse = PageRule;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "zones/{}/pagerules/{}",
            self.zone_identifier, self.rule_identifier
        )
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}
