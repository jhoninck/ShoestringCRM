use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::model::ScalarType;

#[derive(Debug, Clone)]
pub enum ScalarValue {
    Id(Uuid),
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    DateTime(DateTime<Utc>),
    Json(JsonValue),
}

#[derive(Debug, Clone, Copy)]
pub enum FilterOp {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
    Contains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
}

#[derive(Debug, Clone)]
pub struct FieldFilter {
    pub field: String,
    pub op: FilterOp,
    pub value: ScalarValue,
}

#[derive(Debug, Clone)]
pub enum LogicalFilter {
    Simple(FieldFilter),
    And(Vec<LogicalFilter>),
    Or(Vec<LogicalFilter>),
    Not(Box<LogicalFilter>),
}

#[derive(Debug, Clone)]
pub struct OrderBy {
    pub field: String,
    pub direction: OrderDirection,
}

#[derive(Debug, Clone, Copy)]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy)]
pub struct Page {
    pub limit: i64,
    pub offset: i64,
}
