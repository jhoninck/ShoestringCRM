use super::*;

#[derive(Debug, Clone, Copy)]
pub enum ScalarType {
    Id,
    String,
    Int,
    Float,
    Bool,
    DateTime,
    Json,
}

#[derive(Debug, Clone, Copy)]
pub enum RelationKind {
    OneToMany,
    ManyToOne,
    ManyToMany,
}

#[derive(Debug, Clone)]
pub struct RelationDef {
    pub kind: RelationKind,
    pub target_model: &'static str,
    pub fk_column: &'static str,
}

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: &'static str,
    pub column: &'static str,
    pub scalar: Option<ScalarType>,
    pub is_primary_key: bool,
    pub is_filterable: bool,
    pub is_sortable: bool,
    pub is_writable: bool,
    pub has_default: bool,
    pub relation: Option<RelationDef>,
}

#[derive(Debug, Clone, Copy)]
pub struct ModelDef {
    pub name: &'static str,
    pub table: &'static str,
    pub fields: &'static [FieldDef],
}

impl ModelDef {
    pub fn field(&self, name: &str) -> Option<&FieldDef> {
        self.fields.iter().find(|f| f.name == name)
    }

    pub fn pk_field(&self) -> &FieldDef {
        self.fields
            .iter()
            .find(|f| f.is_primary_key)
            .expect("model must have pk")
    }
}
