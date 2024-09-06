use crate::context::Context;
use crate::generator::Generator;
use crate::Result;
use yaml_rust::{Yaml, YamlLoader};

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
        match self.find(path) {
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

    pub fn find(&self, path: &str) -> Option<Yaml> {
        let mut path_iter = path.split('/');
        if path_iter.next() != Some("#") {
            None
        } else {
            let mut name: Option<String> = None;
            let mut yaml = &self.definition[0];
            for key in path_iter {
                name = Some(key.to_string());
                yaml = match yaml
                    .as_hash()
                    .and_then(|h| h.get(&Yaml::String(key.to_string())))
                {
                    Some(y) => y,
                    None => return None,
                };
            }
            match name {
                Some(name) => {
                    let mut hash = yaml.as_hash().unwrap().clone();
                    hash.insert(Yaml::String("name".to_string()), Yaml::String(name));
                    Some(Yaml::Hash(hash))
                }
                None => None,
            }
        }
    }
}
