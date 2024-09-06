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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryExecution {
    /// The number of rows affected by the query.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affected_rows: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<uuid::Uuid>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<QueryExecutionError>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The time it took to execute the query in seconds.
    ///
    /// The time is captured in nanoseconds and converted to seconds using a 64-bit floating-point allowing for
    /// high precision on fast queries without loosing the ability to represent long running queries in seconds.
    /// This decision was made to keep that field usable in Javascript where the number type is a 64-bit
    /// floating-point but can only represent integers up to 2^53 - 1 which would be only 2.5 hours in nanoseconds
    /// before starting to loose precision. In addition seconds are more user friendly than nanoseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<f64>,

    /// The unique identifier of the query execution.
    pub id: uuid::Uuid,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub query: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryExecutionStatus>,
}
