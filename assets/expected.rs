#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
}
