use crate::endpoints::page_rule::PageRuleResponse;
use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

#[derive(Debug)]
pub struct ListPageRules<'a> {
    pub zone_identifier: &'a str,
}

impl EndpointSpec for ListPageRules<'_> {
    type JsonResponse = Vec<PageRuleResponse>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("zones/{}/pagerules", self.zone_identifier)
    }
}
