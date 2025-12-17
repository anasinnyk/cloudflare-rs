use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

// NOTE: Page Rules actions are polymorphic in Cloudflare API.
// `value` depends on `id`, so we intentionally keep this as raw JSON.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PageRuleAction {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PageRuleTarget {
    pub target: String,
    pub constraint: PageRuleConstraint,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PageRuleConstraint {
    pub operator: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PageRuleStatus {
    Active,
    Disabled,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PageRule {
    pub id: String,
    pub actions: Vec<PageRuleAction>,
    pub targets: Vec<PageRuleTarget>,
    pub priority: i64,
    pub status: PageRuleStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,
}

impl ApiResult for PageRule {}
impl ApiResult for Vec<PageRule> {}
