use gpui::{FontFallbacks, FontFeatures, FontWeight};

#[derive(serde::Deserialize, serde::Serialize, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum FontStyle {
    /// A face that is neither italic not obliqued.
    #[default]
    Normal,
    /// A form that is generally cursive in nature.
    Italic,
    /// A typically-sloped version of the regular face.
    Oblique,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Typography {
    pub family: String,

    /// The font features to use.
    pub features: FontFeatures,

    /// The fallbacks fonts to use.
    pub fallbacks: Option<FontFallbacks>,

    /// The font weight.
    pub weight: FontWeight,

    /// The font style.
    pub style: FontStyle,
}

impl From<&'static str> for Typography {
    fn from(family: &'static str) -> Self {
        Self {
            family: String::from(family),
            features: FontFeatures::default(),
            fallbacks: None,
            weight: FontWeight(400.0),
            style: FontStyle::Normal,
        }
    }
}
