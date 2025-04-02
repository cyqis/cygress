use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Backend {
    pub connection_buffer_limits: u32,
    pub idle_timeout: u32,
    pub protocol: String,
}

impl Default for Backend {
    fn default() -> Self {
        Self {
            connection_buffer_limits: 1048576,
            idle_timeout: 10,
            protocol: "http".to_string(),
        }
    }
}
