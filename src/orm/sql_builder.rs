use crate::model::ModelDef;
use crate::orm::query::{FieldFilter, FilterOp, LogicalFilter, OrderBy, OrderDirection, Page, ScalarValue};

pub struct BuiltQuery {
    pub sql: String,
    pub bind_values: Vec<ScalarValue>,
}

pub fn build_select(
    model: &ModelDef,
    filter: Option<&LogicalFilter>,
    order_by: &[OrderBy],
    page: Option<Page>,
) -> BuiltQuery {
    let mut sql = String::new();
    let mut binds = Vec::new();

    sql.push_str("SELECT ");
    let scalar_fields: Vec<_> = model
        .fields
        .iter()
        .filter(|f| f.scalar.is_some())
        .collect();

    for (i, f) in scalar_fields.iter().enumerate() {
        if i > 0 {
            sql.push_str(", ");
        }
        sql.push_str(model.table);
        sql.push('.');
        sql.push_str(f.column);
        sql.push_str(" AS ");
        sql.push_str(f.name);
    }

    sql.push_str(" FROM ");
    sql.push_str(model.table);

    if let Some(filter) = filter {
        sql.push_str(" WHERE ");
        append_logical_filter(model, filter, &mut sql, &mut binds);
    }

    if !order_by.is_empty() {
        sql.push_str(" ORDER BY ");
        for (i, ob) in order_by.iter().enumerate() {
            if i > 0 {
                sql.push_str(", ");
            }
            let field = model.field(&ob.field).expect("unknown order field");

            sql.push_str(model.table);
            sql.push('.');
            sql.push_str(field.column);
            sql.push(' ');

            match ob.direction {
                OrderDirection::Asc => sql.push_str("ASC"),
                OrderDirection::Desc => sql.push_str("DESC"),
            }
        }
    }

    if let Some(page) = page {
        sql.push_str(" LIMIT ");
        sql.push_str(&page.limit.to_string());
        sql.push_str(" OFFSET ");
        sql.push_str(&page.offset.to_string());
    }

    BuiltQuery { sql, bind_values: binds }
}

fn append_logical_filter(
    model: &ModelDef,
    lf: &LogicalFilter,
    sql: &mut String,
    binds: &mut Vec<ScalarValue>,
) {
    match lf {
        LogicalFilter::Simple(sf) => append_simple_filter(model, sf, sql, binds),
        LogicalFilter::And(list) => {
            sql.push('(');
            for (i, f) in list.iter().enumerate() {
                if i > 0 {
                    sql.push_str(" AND ");
                }
                append_logical_filter(model, f, sql, binds);
            }
            sql.push(')');
        }
        LogicalFilter::Or(list) => {
            sql.push('(');
            for (i, f) in list.iter().enumerate() {
                if i > 0 {
                    sql.push_str(" OR ");
                }
                append_logical_filter(model, f, sql, binds);
            }
            sql.push(')');
        }
        LogicalFilter::Not(inner) => {
            sql.push_str("NOT ");
            append_logical_filter(model, inner, sql, binds);
        }
    }
}

fn append_simple_filter(
    model: &ModelDef,
    sf: &FieldFilter,
    sql: &mut String,
    binds: &mut Vec<ScalarValue>,
) {
    let field = model.field(&sf.field).expect("unknown field");
    let placeholder = format!("${}", binds.len() + 1);

    sql.push_str(model.table);
    sql.push('.');
    sql.push_str(field.column);
    sql.push(' ');

    match sf.op {
        FilterOp::Eq => {
            sql.push_str("= ");
            sql.push_str(&placeholder);
            binds.push(sf.value.clone());
        }
        FilterOp::Ne => {
            sql.push_str("!= ");
            sql.push_str(&placeholder);
            binds.push(sf.value.clone());
        }
        FilterOp::Lt => {
            sql.push_str("< ");
            sql.push_str(&placeholder);
            binds.push(sf.value.clone());
        }
        FilterOp::Lte => {
            sql.push_str("<= ");
            sql.push_str(&placeholder);
            binds.push(sf.value.clone());
        }
        FilterOp::Gt => {
            sql.push_str("> ");
            sql.push_str(&placeholder);
            binds.push(sf.value.clone());
        }
        FilterOp::Gte => {
            sql.push_str(">= ");
            sql.push_str(&placeholder);
            binds.push(sf.value.clone());
        }
        FilterOp::Contains => {
            sql.push_str("ILIKE ");
            sql.push_str(&placeholder);
            if let ScalarValue::String(s) = &sf.value {
                binds.push(ScalarValue::String(format!("%{}%", s)));
            } else {
                panic!("Contains only valid for String");
            }
        }
        FilterOp::StartsWith => {
            sql.push_str("ILIKE ");
            sql.push_str(&placeholder);
            if let ScalarValue::String(s) = &sf.value {
                binds.push(ScalarValue::String(format!("{}%", s)));
            } else {
                panic!("StartsWith only valid for String");
            }
        }
        FilterOp::EndsWith => {
            sql.push_str("ILIKE ");
            sql.push_str(&placeholder);
            if let ScalarValue::String(s) = &sf.value {
                binds.push(ScalarValue::String(format!("%{}", s)));
            } else {
                panic!("EndsWith only valid for String");
            }
        }
        FilterOp::In | FilterOp::NotIn => {
            unimplemented!("IN/NOT IN not implemented in this skeleton");
        }
    }
}
