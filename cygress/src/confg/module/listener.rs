use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Listener {
    pub connection_buffer_limits: u32,
    pub http2: Http2,
}

impl Default for Listener {
    fn default() -> Self {
        Self {
            connection_buffer_limits: 32768,
            http2: Http2::default(),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Http2 {
    pub initial_connection_window_size: u32,
    pub initial_stream_window_size: u32,
    pub max_concurrent_streams: u32,
}

impl Default for Http2 {
    fn default() -> Self {
        Self {
            initial_connection_window_size: 1048576,
            initial_stream_window_size: 65535,
            max_concurrent_streams: 100,
        }
    }
}
