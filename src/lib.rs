#![doc = include_str!("../README.md")]
pub mod context;
pub mod generator;

pub use context::Context;
pub use generator::Generator;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn rustfmt(source: String) -> Result<String> {
    rustfmt_wrapper::rustfmt(source).map_err(|e| format!("rust formatting failed: {}", e).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use yaml_rust::YamlLoader;

    #[test]
    fn test_openapi() {
        let generator =
            Generator::from_templates(vec![("macros", include_str!("../assets/macros.j2"))])
                .unwrap();
        let openapi = YamlLoader::load_from_str(include_str!("../assets/openapi.yaml")).unwrap();
        let context = context::Context::from(&openapi[0]);
        let result = generator
            .render_string(
                r#"
{% from "macros" import enum %}
{{ enum("QueryExecutionStatus", components.schemas.QueryExecutionStatus) }}                    
        "#,
                &context,
            )
            .and_then(rustfmt);
        assert_eq!(result.unwrap(), include_str!("../assets/expected.rs"));
    }
}
