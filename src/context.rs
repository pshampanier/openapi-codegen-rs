use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use yaml_rust::Yaml;

#[derive(Debug)]
pub enum Context {
    String(String),
    Integer(i64),
    Boolean(bool),
    Array(Vec<Context>),
    Hash(HashMap<String, Context>),
    Null,
}

impl Context {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Context::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Context::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Context::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Context>> {
        match self {
            Context::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_hash(&self) -> Option<&HashMap<String, Context>> {
        match self {
            Context::Hash(h) => Some(h),
            _ => None,
        }
    }
}

impl Serialize for Context {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        match self {
            Context::Null => serializer.serialize_none(),
            Context::String(s) => serializer.serialize_str(s),
            Context::Integer(i) => serializer.serialize_i64(*i),
            Context::Boolean(b) => serializer.serialize_bool(*b),
            Context::Array(a) => {
                let mut seq = serializer.serialize_seq(Some(a.len()))?;
                for item in a {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            Context::Hash(h) => {
                let mut map = serializer.serialize_map(Some(h.len()))?;
                for (key, value) in h {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
        }
    }
}

impl From<&Yaml> for Context {
    fn from(yaml: &Yaml) -> Self {
        match yaml {
            Yaml::Null => Context::Null,
            Yaml::String(s) => Context::String(s.into()),
            Yaml::Integer(i) => Context::Integer(*i),
            Yaml::Boolean(b) => Context::Boolean(*b),
            Yaml::Array(a) => {
                let mut array = Vec::new();
                for item in a {
                    array.push(Context::from(item));
                }
                Context::Array(array)
            }
            Yaml::Hash(h) => {
                let mut hash = HashMap::new();
                for (key, value) in h {
                    hash.insert(key.as_str().unwrap().into(), Context::from(value));
                }
                Context::Hash(hash)
            }
            _ => panic!("Unsupported type"),
        }
    }
}

impl From<Vec<Yaml>> for Context {
    fn from(yaml: Vec<Yaml>) -> Self {
        if yaml.len() != 1 {
            panic!("Expected a single YAML document");
        }
        match &yaml[0] {
            Yaml::Hash(map) => {
                let name = map.front().unwrap().0.as_str().unwrap();
                let values = map.front().unwrap().1.clone();
                let mut context = HashMap::<String, Context>::new();
                context.insert("name".into(), Context::String(name.into()));
                for (key, value) in values.as_hash().unwrap() {
                    context.insert(key.as_str().unwrap().to_string(), Context::from(value));
                }
                Context::Hash(context)
            }
            _ => panic!("Expected a map"),
        }
    }
}
