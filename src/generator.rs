use crate::context::Context;
use crate::Result;
use minijinja::Environment;
use std::{ffi::OsStr, path::Path};

pub struct Generator<'a> {
    pub env: Environment<'a>,
}

impl<'a> Generator<'a> {
    pub fn from_templates(templates: Vec<(&'a str, &'a str)>) -> Result<Generator<'a>> {
        let mut env = Environment::new();
        for (name, source) in templates {
            env.add_template(name, source)?;
        }
        Self::add_filters(&mut env);
        Ok(Generator { env })
    }

    pub fn from_templates_dir<P: AsRef<Path>>(template_dir: P) -> Result<Generator<'static>> {
        let mut env = Environment::new();
        if let Ok(entries) = std::fs::read_dir(template_dir) {
            for entry in entries {
                let path = entry?.path();
                if !path.is_dir() && path.extension() == Some(OsStr::new("j2")) {
                    let source = std::fs::read_to_string(&path)?;
                    let name = path.file_stem().unwrap().to_string_lossy().to_string();
                    env.add_template_owned(name, source)?;
                }
            }
        }
        Self::add_filters(&mut env);
        Ok(Generator { env })
    }

    pub fn render_template(&self, name: &str, context: &Context) -> Result<String> {
        let template = self.env.get_template(name)?;
        Ok(template.render(context)?)
    }

    pub fn render_string(&self, source: &str, context: &Context) -> Result<String> {
        Ok(self.env.render_str(source, context)?)
    }

    fn add_filters(env: &mut Environment<'_>) {
        env.add_filter("snake_case", |s: String| {
            let mut snake_case = String::new();
            for (i, c) in s.chars().enumerate() {
                if c.is_uppercase() {
                    if i != 0 {
                        snake_case.push('_');
                    }
                    snake_case.push(c.to_ascii_lowercase());
                } else {
                    snake_case.push(c);
                }
            }
            snake_case
        });

        env.add_filter("camel_case", |s: String| {
            let mut camel_case = String::new();
            let mut uppercase_next = false;
            for c in s.chars() {
                if c == '_' {
                    uppercase_next = true;
                } else if uppercase_next {
                    camel_case.push(c.to_ascii_uppercase());
                    uppercase_next = false;
                } else {
                    camel_case.push(c);
                }
            }
            camel_case
        });

        env.add_filter("pascal_case", |s: String| {
            let mut pascal_case = String::new();
            let mut uppercase_next = true;
            for c in s.chars() {
                if c == '_' {
                    uppercase_next = true;
                } else if uppercase_next {
                    pascal_case.push(c.to_ascii_uppercase());
                    uppercase_next = false;
                } else {
                    pascal_case.push(c);
                }
            }
            pascal_case
        });
    }
}
