> A basic code generator based on MiniJinja

This crate does the bare minimum to ease the generation of code, documentation or whatever from a OpenAPI YAML
definition.

## Example

```rust
use openapi_codegen::{rustfmt, OpenAPI, Generator};

let generator = Generator::from_templates(vec![
    ("enum", include_str!("../assets/templates/rust/enum.j2")),
    ("object", include_str!("../assets/templates/rust/object.j2")),
]).unwrap();
let openapi = OpenAPI::from_yaml_str(include_str!("../assets/openapi.yaml")).unwrap();
println!("{}", openapi
    .apply_templates(
        &generator,
        &vec![
            "#/components/schemas/QueryExecutionStatus",
            "#/components/schemas/QueryExecutionError",
        ],
    )
    .and_then(rustfmt).unwrap());
```

Should display:

```rust,ignore
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryExecutionError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}
```

> [!NOTE]
> The Jinja templates available in the create are only an example and should be adapted for your own requirements.
