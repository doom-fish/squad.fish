use std::fmt::Display;

use crate::types::color::Color;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Colors {
    pub text_color: Color,
    pub background_color: Color,
    pub primary_color: Color,
    pub secondary_color: Color,
    pub accent_color: Color,
    pub error_color: Color,
    pub success_color: Color,
    pub warning_color: Color,
    pub disabled_color: Color,
    pub border_color: Color,
}

impl Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Colors")
            .field("text_color", &self.text_color)
            .field("background_color", &self.background_color)
            .field("primary_color", &self.primary_color)
            .field("secondary_color", &self.secondary_color)
            .field("accent_color", &self.accent_color)
            .field("error_color", &self.error_color)
            .field("success_color", &self.success_color)
            .field("warning_color", &self.warning_color)
            .field("disabled_color", &self.disabled_color)
            .field("border_color", &self.border_color)
            .finish()
    }
}
