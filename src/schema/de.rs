// #![allow(dead_code, unused_imports)]

// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::marker::PhantomData;

// use crate::schema::Dialect;
// use crate::{Error, Result, Schema};
// use serde::de::Deserializer as SerdeDeserializer;
// use serde::de::{self, Visitor};
// use serde_json as json;
// use std::{io, result::Result as StdResult};
// pub struct Deserializer<R> {
//     dialect: RefCell<Dialect>,
//     refs: RefCell<Vec<String>>,
//     dynamic_refs: RefCell<Vec<String>>,
//     recursive_refs: RefCell<Vec<String>>,
//     read: R,
// }

// struct DialectVisitor {}
// impl<'de> Visitor<'de> for DialectVisitor {
//     type Value = Dialect;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("a JSON Schema dialect identifier")
//     }

//     fn visit_str<E>(self, v: &str) -> StdResult<Self::Value, E>
//     where
//         E: de::Error,
//     {
//         Dialect::try_from(v).map_err(|_| de::Error::unknown_variant(v, Dialect::supported_drafts()))
//     }
// }

// struct SchemaVisitor {}
// impl<'de> Visitor<'de> for SchemaVisitor {
//     type Value = Schema;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("a JSON Schema")
//     }
//     fn visit_enum<A>(self, data: A) -> StdResult<Self::Value, A::Error>
//     where
//         A: de::EnumAccess<'de>,
//     {
//         println!("!!!INSIDE visit_enum");
//         todo!()
//     }
//     fn visit_map<A>(self, mut access: A) -> StdResult<Self::Value, A::Error>
//     where
//         A: de::MapAccess<'de>,
//     {
//         let map = json::Map::with_capacity(access.size_hint().unwrap_or(0));
//         while let Some((key, value)) = access.next_entry()? {
//             map.insert(key, value);
//         }

//         let

//         let data = json::ser::to_string(&map).unwrap();
//     }
// }

// impl<'de, R: json::de::Read<'de>> Deserializer<R> {
//     /// new creates a new Schema from the given json_serde reader. This is
//     /// mostly for internal use. Odds are you should use the `from_reader`,
//     /// `from_str`, or `from_slice` function instead.
//     pub fn new(read: R, dialect: Dialect) -> Self {
//         Deserializer {
//             read,
//             dialect: RefCell::new(dialect),
//             refs: RefCell::new(Vec::new()),
//             dynamic_refs: RefCell::new(Vec::new()),
//             recursive_refs: RefCell::new(Vec::new()),
//         }
//     }
//     /// The `Deserializer::end` method should be called after a value has been fully deserialized.
//     /// This allows the `Deserializer` to validate that the input stream is at the end or that it
//     /// only has trailing whitespace.
//     pub fn end(self) -> StdResult<(), json::Error> {
//         json::Deserializer::new(self.read).end()
//     }
// }
// impl<'a, 'de, R: io::Read> Deserializer<json::de::IoRead<R>> {
//     pub fn from_reader(reader: R) -> Self {
//         Deserializer {
//             dialect: RefCell::new(Dialect::Draft202012),
//             refs: RefCell::new(Vec::new()),
//             dynamic_refs: RefCell::new(Vec::new()),
//             recursive_refs: RefCell::new(Vec::new()),
//             read: json::de::IoRead::new(reader),
//         }
//     }
// }
// #[allow(clippy::should_implement_trait)]
// impl<'de> Deserializer<json::de::StrRead<'de>> {
//     pub fn from_str(s: &'de str) -> Self {
//         Deserializer {
//             dialect: RefCell::new(Dialect::Draft202012),
//             refs: RefCell::new(Vec::new()),
//             dynamic_refs: RefCell::new(Vec::new()),
//             recursive_refs: RefCell::new(Vec::new()),
//             read: json::de::StrRead::new(s),
//         }
//     }
// }
// impl<'de> Deserializer<json::de::SliceRead<'de>> {
//     pub fn from_slice(bytes: &'de [u8]) -> Self {
//         Deserializer {
//             dialect: RefCell::new(Dialect::Draft202012),
//             refs: RefCell::new(Vec::new()),
//             dynamic_refs: RefCell::new(Vec::new()),
//             recursive_refs: RefCell::new(Vec::new()),
//             read: json::de::SliceRead::new(bytes),
//         }
//     }
// }

// impl<'de, R> de::Deserializer<'de> for Deserializer<R>
// where
//     R: json::de::Read<'de>,
// {
//     type Error = json::Error;

//     fn deserialize_any<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_any(visitor)
//     }

//     fn deserialize_bool<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_bool(visitor)
//     }

//     fn deserialize_i8<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_i8(visitor)
//     }

//     fn deserialize_i16<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_i16(visitor)
//     }

//     fn deserialize_i32<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_i32(visitor)
//     }

//     fn deserialize_i64<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_i64(visitor)
//     }

//     fn deserialize_u8<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_u8(visitor)
//     }

//     fn deserialize_u16<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_u16(visitor)
//     }

//     fn deserialize_u32<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_u32(visitor)
//     }

//     fn deserialize_u64<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_u64(visitor)
//     }

//     fn deserialize_f32<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_f32(visitor)
//     }

//     fn deserialize_f64<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_f64(visitor)
//     }

//     fn deserialize_char<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_char(visitor)
//     }

//     fn deserialize_str<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_str(visitor)
//     }

//     fn deserialize_string<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_string(visitor)
//     }

//     fn deserialize_bytes<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_bytes(visitor)
//     }

//     fn deserialize_byte_buf<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_byte_buf(visitor)
//     }

//     fn deserialize_option<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_option(visitor)
//     }

//     fn deserialize_unit<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_unit(visitor)
//     }

//     fn deserialize_unit_struct<V>(
//         self,
//         name: &'static str,
//         visitor: V,
//     ) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_unit_struct(name, visitor)
//     }

//     fn deserialize_newtype_struct<V>(
//         self,
//         name: &'static str,
//         visitor: V,
//     ) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_newtype_struct(name, visitor)
//     }

//     fn deserialize_seq<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_seq(visitor)
//     }

//     fn deserialize_tuple<V>(self, len: usize, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_tuple(len, visitor)
//     }

//     fn deserialize_tuple_struct<V>(
//         self,
//         name: &'static str,
//         len: usize,
//         visitor: V,
//     ) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_tuple_struct(name, len, visitor)
//     }

//     fn deserialize_map<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_map(visitor)
//     }

//     fn deserialize_struct<V>(
//         self,
//         name: &'static str,
//         fields: &'static [&'static str],
//         visitor: V,
//     ) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_struct(name, fields, visitor)
//     }

//     fn deserialize_enum<V>(
//         self,
//         name: &'static str,
//         variants: &'static [&'static str],
//         visitor: V,
//     ) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         println!("deserialize_enum: {}", name);

//         json::Deserializer::new(self.read).deserialize_enum(name, variants, visitor)
//     }

//     fn deserialize_identifier<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_identifier(visitor)
//     }

//     fn deserialize_ignored_any<V>(self, visitor: V) -> StdResult<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         json::Deserializer::new(self.read).deserialize_ignored_any(visitor)
//     }
// }

// fn from_trait<'de, R: json::de::Read<'de>>(read: R) -> Result<Schema> {
//     let deserializer = Deserializer::new(read, Dialect::Draft202012);
//     deserializer
//         .deserialize_any(SchemaVisitor {})
//         .map_err(Error::from)
// }

// pub fn from_reader<R>(reader: R) -> Result<Schema>
// where
//     R: std::io::Read,
// {
//     from_trait(json::de::IoRead::new(reader))
// }

// pub fn from_str(s: &str) -> Result<Schema> {
//     from_trait(json::de::StrRead::new(s))
// }
// // ////////////////////////////////////////////////////////////////////////////////////////////////

// // ------------------------------------------------------------------------------------------------

// #[cfg(test)]
// mod test {
//     use serde::Deserialize;

//     use crate::Schema;
//     #[test]
//     fn test_deserialize() {
//         let s = r###"
//         {
//             "$schema": "https://json-schema.org/draft/2020-12/schema",
//             "$id": "https://example.com/tree",
//             "$dynamicAnchor": "node",
//             "type": "object",
//             "properties": {
//               "data": true,
//               "children": {
//                 "type": "array",
//                 "items": { "$dynamicRef": "#node" }
//               }
//             }
//           }
//         "###;

//         let r = super::from_str(s).unwrap();
//         println!("{:?}", r);
//     }
// }
