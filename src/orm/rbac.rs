use uuid::Uuid;

use crate::model::ModelDef;
use crate::orm::query::{FieldFilter, FilterOp, LogicalFilter, ScalarValue};

#[derive(Debug, Clone)]
pub enum PrincipalKind {
    InternalUser,
    ExternalUser,
    Machine,
}

#[derive(Debug, Clone)]
pub struct Principal {
    pub sub: String,
    pub org_id: Option<String>,
    pub kind: PrincipalKind,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub user_id: Option<Uuid>,
    pub contact_id: Option<Uuid>,
}

pub fn scope_filter_for(model: &ModelDef, principal: &Principal) -> Option<LogicalFilter> {
    if principal.roles.iter().any(|r| r == "CRM_PLATFORM_ADMIN") {
        return None;
    }

    if let Some(org_id) = &principal.org_id {
        if model.field("orgId").is_some() {
            return Some(LogicalFilter::Simple(FieldFilter {
                field: "orgId".to_string(),
                op: FilterOp::Eq,
                value: ScalarValue::String(org_id.clone()),
            }));
        }
    }

    None
}

pub fn with_scope(
    model: &ModelDef,
    principal: &Principal,
    user_filter: Option<LogicalFilter>,
) -> Option<LogicalFilter> {
    let scope = scope_filter_for(model, principal);

    match (scope, user_filter) {
        (None, uf) => uf,
        (Some(sf), None) => Some(sf),
        (Some(sf), Some(uf)) => Some(LogicalFilter::And(vec![sf, uf])),
    }
}
