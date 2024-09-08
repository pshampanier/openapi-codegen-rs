use crate::context::Context;
use crate::generator::Generator;
use crate::Result;
use yaml_rust::{yaml::Hash, Yaml, YamlLoader};

pub struct OpenAPI {
    pub definition: Vec<Yaml>,
}

impl OpenAPI {
    pub fn from_yaml_str(yaml: &str) -> Result<Self> {
        Ok(OpenAPI {
            definition: YamlLoader::load_from_str(yaml)?,
        })
    }

    pub fn into_context(&self, path: &str) -> Result<Context> {
        match self.get(path) {
            Some(yaml) => Ok(Context::from(&yaml)),
            None => Err(format!("{path}: Path not found").into()),
        }
    }

    pub fn apply_templates(&self, generator: &Generator<'_>, paths: &Vec<&str>) -> Result<String> {
        let mut result = String::new();
        for path in paths {
            let context = self.into_context(path)?;
            if let Some(template_name) = context.as_hash().and_then(|map| {
                map.get("enum")
                    .is_some()
                    .then_some("enum")
                    .or_else(|| map.get("type").and_then(Context::as_str))
            }) {
                result.push_str(&generator.render_template(template_name, &context)?)
            } else {
                return Err(format!("{path}: Path is not a supported schema.").into());
            }
        }
        Ok(result)
    }

    /// Find a path in the OpenAPI definition.
    fn find(&self, path: &str) -> Option<Yaml> {
        let mut path_iter = path.split('/');
        if path_iter.next() != Some("#") {
            None
        } else {
            let mut yaml = &self.definition[0];
            for key in path_iter {
                yaml = match yaml
                    .as_hash()
                    .and_then(|h| h.get(&Yaml::String(key.to_string())))
                {
                    Some(y) => y,
                    None => return None,
                };
            }
            Some(yaml.clone())
        }
    }

    /// Get a path in the OpenAPI definition.
    ///
    /// The path should be in the format of `#/path/to/definition`.
    /// The returned value is an altered version of the definition with:
    /// - the `name` field set to the last part of the path (e.g. `name: definition` in the example above)
    /// - a field `references` that contains all definitions used by returned definition.
    fn get(&self, path: &str) -> Option<Yaml> {
        let yaml = self.find(path)?;
        let mut refs = Vec::<String>::new();
        let mut hash = yaml.as_hash().unwrap().clone();
        if let Some(v) = hash.get(&Yaml::String("properties".to_string())) {
            if v.as_hash().is_some() {
                for (_k, v) in v.as_hash().unwrap() {
                    if let Some(v) = v
                        .as_hash()
                        .and_then(|h| h.get(&Yaml::String("$ref".to_string())))
                    {
                        refs.push(v.as_str().unwrap().to_string());
                    }
                }
            }
        }

        // adding the `name` field
        let name = path.split('/').last().unwrap().to_string();
        hash.insert(Yaml::String("name".to_string()), Yaml::String(name));

        // adding the `$$refs` field
        if !refs.is_empty() {
            let mut refs_field = Hash::new();
            for ref_name in refs.iter() {
                if let Some(v) = self.find(ref_name) {
                    refs_field.insert(Yaml::String(ref_name.to_string()), v);
                }
            }
            hash.insert(
                Yaml::String("references".to_string()),
                Yaml::Hash(refs_field),
            );
        }

        Some(Yaml::Hash(hash))
    }
}
