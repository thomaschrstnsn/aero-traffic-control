use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListedWindow {
    app_name: String,
    window_id: usize,
    window_title: String,
}
