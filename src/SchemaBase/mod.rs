//! Scala SchemaBase interface, independent of JSON spelling.
pub trait SchemaBase {
    fn doc_index(&self) -> i32;
    fn description(&self) -> &str;
    fn provided_by_frontend(&self) -> bool {
        false
    }
}
