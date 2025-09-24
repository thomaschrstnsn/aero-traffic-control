use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ListedWindow {
    app_name: String,
    window_id: usize,
    _window_title: String,
}

impl Eq for ListedWindow {}

impl PartialEq for ListedWindow {
    fn eq(&self, other: &Self) -> bool {
        self.window_id == other.window_id
    }
}

impl PartialOrd for ListedWindow {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListedWindow {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.window_id.cmp(&other.window_id)
    }
}

impl ListedWindow {
    pub fn matches_app_name<T: AsRef<str>>(&self, app: &T) -> bool {
        let app = app.as_ref();
        self.app_name == app
    }
}

pub trait Focusable {
    fn window_id(&self) -> usize;
}

impl Focusable for usize {
    fn window_id(&self) -> usize {
        *self
    }
}

impl Focusable for ListedWindow {
    fn window_id(&self) -> usize {
        self.window_id
    }
}
