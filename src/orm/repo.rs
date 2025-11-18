use sqlx::{PgPool, postgres::PgRow, Row};
use uuid::Uuid;

use crate::model::ModelDef;
use crate::orm::query::{LogicalFilter, OrderBy, Page, ScalarValue};
use crate::orm::sql_builder::{build_select, BuiltQuery};

pub async fn fetch_many(
    pool: &PgPool,
    model: &ModelDef,
    filter: Option<&LogicalFilter>,
    order_by: &[OrderBy],
    page: Option<Page>,
) -> anyhow::Result<Vec<PgRow>> {
    let BuiltQuery { sql, bind_values } =
        build_select(model, filter, order_by, page);

    let mut query = sqlx::query(&sql);
    for v in bind_values {
        query = match v {
            ScalarValue::String(s) => query.bind(s),
            ScalarValue::Id(id) => query.bind(id),
            ScalarValue::Int(i) => query.bind(i),
            ScalarValue::Float(f) => query.bind(f),
            ScalarValue::Bool(b) => query.bind(b),
            ScalarValue::DateTime(dt) => query.bind(dt),
            ScalarValue::Json(j) => query.bind(j),
        };
    }

    let rows = query.fetch_all(pool).await?;
    Ok(rows)
}
