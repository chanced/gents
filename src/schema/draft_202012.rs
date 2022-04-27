use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct Draft202012 {
    /// The `$schema` keyword is used to declare which dialect of JSON Schema the
    /// schema was written for. The value of the `$schema` keyword is also the
    /// identifier for a schema that can be used to verify that the schema is valid
    /// according to the dialect $schema identifies. A schema that describes another
    /// schema is called a “meta-schema”.
    ///
    /// `$schema` applies to the entire document and must be at the root level. It
    /// does not apply to externally referenced (`$ref`, `$dynamicRef`) documents. Those
    /// schemas need to declare their own $schema.
    ///
    /// If `$schema` is not used, an implementation might allow you to specify a
    /// value externally or it might make assumptions about which specification
    /// version should be used to evaluate the schema. It’s recommended that all
    /// JSON Schemas have a `$schema` keyword to communicate to readers and tooling
    /// which specification version is intended.
    ///
    /// see also:
    /// - https://json-schema.org/understanding-json-schema/reference/schema.html#schema
    /// - https://json-schema.org/draft/2020-12/json-schema-core.html#rfc.section.8.1.1
    #[serde(rename = "$schema")]
    pub syntax: Option<String>,

    pub refs: Vec<String>,
    pub dynamic_refs: Vec<String>,
    pub anchors: Vec<String>,
}
