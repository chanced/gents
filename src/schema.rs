pub mod de;
mod dialect;
mod draft_202012;
use crate::tri;
use crate::{Error, Result};
pub use dialect::*;
pub use draft_202012::*;
use serde::{
    de::{DeserializeSeed, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_json as json;
use std::{result::Result as StdResult, str::FromStr};

#[derive(Debug, Deserialize, Serialize)]
pub struct Reference {}

pub enum SchemaOrSchemas {
    Schema(Schema),
    Schemas(Vec<Schema>),
}

#[derive(Debug, Serialize)]
pub enum Schema {
    Draft202012(Draft202012),
}

struct DialectVisitor {}

impl<'de> Visitor<'de> for DialectVisitor {
    type Value = Dialect;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a jsons schema draft uri")
    }
    fn visit_str<E>(self, s: &str) -> StdResult<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Dialect::from_str(s).map_err(|_| E::unknown_variant(s, Dialect::supported_drafts()))
    }
}

struct SchemaData {
    data: json::Map<String, json::Value>,
    dialect: Dialect,
}
impl<'de> DeserializeSeed<'de> for SchemaData {
    type Value = Schema;

    fn deserialize<D>(self, de: D) -> StdResult<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let dialect = self
            .data
            .get("$schema")
            .map_or(Ok(self.dialect), |v| {
                v.deserialize_string(DialectVisitor {})
            })
            .map_err(|e| todo!());

        dbg!(dialect);
        todo!()
    }
}

struct SchemaVisitor {
    dialect: Dialect,
}

impl<'de> Visitor<'de> for SchemaVisitor {
    type Value = Schema;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a json schema")
    }
    fn visit_bool<E>(self, v: bool) -> StdResult<Self::Value, E>
    where
        E: serde::de::Error,
    {
        println!("visit_bool: {}", v);
        todo!()
    }

    fn visit_str<E>(self, v: &str) -> StdResult<Self::Value, E>
    where
        E: serde::de::Error,
    {
        println!("visit_str: {}", v);
        todo!()
    }

    fn visit_map<A>(self, mut access: A) -> StdResult<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut map = json::Map::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        todo!()
    }
}

pub fn deserialize_str(s: &str, dialect: Dialect) -> Result<Schema> {
    let mut de = serde_json::Deserializer::from_str(s);
    de.deserialize_any(SchemaVisitor { dialect })
        .map_err(Error::from)
}

//
// ═══════════════════════════════════════════════
// ═══════════════════════════════════════════════
// ═══════════════════════════════════════════════
//
#[cfg(test)]
mod test {
    use crate::Schema;

    use super::{deserialize_str, Dialect, SchemaVisitor, DEFAULT_VERSION};
    use serde::Deserializer;
    fn set_default_version() {
        DEFAULT_VERSION.set(Dialect::Draft202012).unwrap();
    }
    #[test]
    fn testing_schema_deserialization() {
        set_default_version();
        let s = r###"
        {
            "$id": "https://example.com/tree",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$dynamicAnchor": "node",
            "type": "object",
            "properties": {
              "data": true,
              "children": {
                "type": "array",
                "items": { "$dynamicRef": "#node" }
              }
            }
          }
        "###;
        let s = deserialize_str(s, Dialect::Draft202012).unwrap();
        dbg!(s);
    }
}

// let schema: serde_json::Value = serde_json::from_str(val)?;
// let version = schema["$schema"].as_str().ok_or(Error::UnsupportedSchema {
//     schema: val.to_string(),
// })?;
// let version = Version::try_from(version)?;
// Ok(match version {
//     Version::V202012 => Schema::V202012(serde_json::from_str(val)?),
// })
