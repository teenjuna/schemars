use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use surrealdb::sql::Thing;

forward_impl!((JsonSchema for Thing) => String);
