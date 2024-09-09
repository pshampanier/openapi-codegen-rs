> A basic code generator based on MiniJinja

This crate does the bare minimum to ease the generation of code, documentation or whatever from a OpenAPI YAML
definition.

## Example

```rust
use openapi_codegen::{rustfmt, Context, Generator};
use yaml_rust::YamlLoader;

let openapi = YamlLoader::load_from_str(include_str!("../assets/openapi.yaml")).unwrap();
let generator =
    Generator::from_templates(vec![("macros", include_str!("../assets/macros.j2"))])
        .unwrap();
let context = Context::from(&openapi[0]);
let result = generator
    .render_string(r#"
{% from "macros" import enum %}
{{ enum("QueryExecutionStatus", components.schemas.QueryExecutionStatus) }}
    "#,
        &context,
    )
    .and_then(rustfmt);
```

Should generate:

```rust,ignore
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
}
```

> [!NOTE]
> The Jinja templates available in the create are only an example and should be adapted for your own requirements.
