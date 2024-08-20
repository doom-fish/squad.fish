pub mod active_theme;
pub mod styles;
pub mod types;

use std::{error::Error, fmt::Display};

use styles::{colors::Colors, typography::Typography};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Theme {
    pub name: String,
    pub colors: Colors,
    pub typography: Typography,
}

impl Theme {
    pub fn load_default() -> Result<Self, Box<dyn Error>> {
        let theme = include_str!("../themes/default.yaml");
        serde_yml::from_str(theme).map_err(Into::into)
    }
}
impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Theme")
            .field("name", &self.name)
            .field("colors", &self.colors)
            .field("typography", &self.typography)
            .finish()
    }
}
impl Default for Theme {
    fn default() -> Self {
        Self::load_default().expect("Failed to load default theme")
    }
}
