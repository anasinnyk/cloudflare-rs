use crate::endpoints::page_rule::types::{
    PageRule, PageRuleAction, PageRuleStatus, PageRuleTarget,
};
use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct CreatePageRule<'a> {
    pub zone_identifier: &'a str,
    pub params: CreatePageRuleParams,
}

impl EndpointSpec for CreatePageRule<'_> {
    type JsonResponse = PageRule;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("zones/{}/pagerules", self.zone_identifier)
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreatePageRuleParams {
    pub actions: Vec<PageRuleAction>,
    pub targets: Vec<PageRuleTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PageRuleStatus>,
}
