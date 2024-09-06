#![doc = include_str!("../README.md")]
pub mod context;
pub mod generator;
pub mod openapi;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub use generator::Generator;
pub use openapi::OpenAPI;

pub fn rustfmt(source: String) -> Result<String> {
    rustfmt_wrapper::rustfmt(source).map_err(|e| format!("rust formatting failed: {}", e).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi() {
        let generator = Generator::from_templates(vec![
            ("enum", include_str!("../assets/templates/rust/enum.j2")),
            ("object", include_str!("../assets/templates/rust/object.j2")),
        ])
        .unwrap();
        let openapi = OpenAPI::from_yaml_str(include_str!("../assets/openapi.yaml")).unwrap();
        let result = openapi
            .apply_templates(
                &generator,
                &vec![
                    "#/components/schemas/QueryExecutionStatus",
                    "#/components/schemas/QueryExecutionError",
                    "#/components/schemas/QueryExecution",
                ],
            )
            .and_then(rustfmt);
        assert_eq!(result.unwrap(), include_str!("../assets/expected.rs"));
    }
}
