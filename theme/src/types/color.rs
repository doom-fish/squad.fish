#![allow(dead_code)]
use std::{
    error::Error,
    fmt::{self, Display},
};

use gpui::Fill;
use hex::FromHex;

use palette::FromColor;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Rgb { r: f32, g: f32, b: f32 },
    Rgba { r: f32, b: f32, g: f32, a: f32 },
    OkLab { l: f32, a: f32, b: f32 },
    OkLaba { l: f32, a: f32, b: f32, alpha: f32 },
    OkLch { l: f32, c: f32, h: f32 },
    OkLcha { l: f32, c: f32, h: f32, a: f32 },
    Hsl { h: f32, s: f32, l: f32 },
    Hsla { h: f32, s: f32, l: f32, a: f32 },
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Color::Rgb { r, g, b } => format!("rgb({},{},{})", r, g, b),
            Color::Rgba { r, b, g, a } => {
                format!("rgb({},{},{}, {})", r, g, b, a)
            }
            Color::OkLab { l, a, b } => {
                format!("rgb({},{},{})", l, a, b)
            }
            Color::OkLaba { l, a, b, alpha } => {
                format!("oklaba({},{},{}, {})", l, a, b, alpha)
            }
            Color::OkLch { l, c, h } => {
                format!("oklch({},{},{})", l, c, h)
            }
            Color::OkLcha { l, c, h, a } => {
                format!("oklcha({},{},{}, {})", l, c, h, a)
            }
            Color::Hsl { h, s, l } => {
                format!("hsl({},{},{})", h, s, l)
            }
            Color::Hsla { h, s, l, a } => {
                format!("hsla({},{},{}, {})", h, s, l, a)
            }
        };
        f.write_str(&val)
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
struct ColorVisitor;

fn parse_color_values(
    color_type: &str,
    value: &str,
    construct: fn(f32, f32, f32) -> Color,

    construct_alpha: fn(f32, f32, f32, f32) -> Color,
) -> Option<Color> {
    if !value.starts_with(color_type) {
        return None;
    }

    let values = value
        .replace(&format!("{}(", color_type), "")
        .replace(")", "")
        .replace(" ", "")
        .split(",")
        .map(|v| v.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();
    if values.len() == 3 {
        Some(construct(values[0], values[1], values[2]))
    } else if values.len() == 4 {
        Some(construct_alpha(values[0], values[1], values[2], values[3]))
    } else {
        None
    }
}

fn parse_color_string(value: &str) -> Result<Color, Box<dyn Error>> {
    if value.starts_with("#") {
        if value.len() == 7 {
            let [r, g, b] = <[u8; 3]>::from_hex(value.replace("#", ""))
                .map_err(|e| e.to_string())?
                .map(<f32>::from);
            return Ok(Color::Rgb { r, g, b });
        } else if value.len() == 9 {
            let [r, g, b, a] = <[u8; 4]>::from_hex(value.replace("#", ""))
                .map_err(|e| e.to_string())?
                .map(<f32>::from);
            return Ok(Color::Rgba { r, g, b, a });
        }
    }
    if let Some(color) = parse_color_values(
        "rgb",
        value,
        |r, g, b| Color::Rgb { r, g, b },
        |r, g, b, a| Color::Rgba { r, g, b, a },
    ) {
        Ok(color)
    } else if let Some(color) = parse_color_values(
        "oklch",
        value,
        |l, c, h| Color::OkLch { l, c, h },
        |l, c, h, a| Color::OkLcha { l, c, h, a },
    ) {
        Ok(color)
    } else if let Some(color) = parse_color_values(
        "oklab",
        value,
        |l, a, b| Color::OkLab { l, a, b },
        |l, a, b, alpha| Color::OkLaba { l, a, b, alpha },
    ) {
        Ok(color)
    } else if let Some(color) = parse_color_values(
        "hsl",
        value,
        |h, s, l| Color::Hsl { h, s, l },
        |h, s, l, a| Color::Hsla { h, s, l, a },
    ) {
        Ok(color)
    } else {
        Err(format!("Error parsing color: {value}").into())
    }
}

impl TryFrom<&str> for Color {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_color_string(value)
    }
}

impl From<Color> for Fill {
    fn from(val: Color) -> Self {
        let b = gpui::Hsla::from(val);
        b.into()
    }
}

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a color value in rgb, rgba, hsl, hsla, oklab, oklaba, oklch, oklcha")
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Color::try_from(value).or_else(|e| {
            eprintln!("{e}. Color will be set to magenta (#ff00ff)");
            Ok(rgba(1.0, 0.0, 1.0, 1.0))
        })
    }
}
impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ColorVisitor)
    }
}

pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::Rgb { r, g, b }
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::Rgba { r, g, b, a }
}
pub fn oklab(l: f32, a: f32, b: f32) -> Color {
    Color::OkLab { l, a, b }
}
pub fn oklaba(l: f32, a: f32, b: f32, alpha: f32) -> Color {
    Color::OkLaba { l, a, b, alpha }
}
pub fn oklch(l: f32, c: f32, h: f32) -> Color {
    Color::OkLch { l, c, h }
}
pub fn oklcha(l: f32, c: f32, h: f32, a: f32) -> Color {
    Color::OkLcha { l, c, h, a }
}
pub fn hsl(h: f32, s: f32, l: f32) -> Color {
    Color::Hsl { h, s, l }
}
pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Color {
    Color::Hsla { h, s, l, a }
}
impl From<Color> for gpui::Hsla {
    fn from(val: Color) -> Self {
        match val {
            Color::Rgb { r, g, b } => gpui::Rgba { r, g, b, a: 1.0 }.into(),
            Color::Rgba { r, g, b, a } => gpui::Rgba { r, g, b, a }.into(),
            Color::OkLab { l, a, b } => {
                let c: palette::Hsla = palette::Hsla::from_color(palette::Lab::new(l, a, b));
                gpui::Hsla {
                    h: f64::from(c.hue) as f32 / 360.,
                    s: c.saturation as f32,
                    l: c.lightness as f32,
                    a: c.alpha as f32,
                }
            }
            Color::OkLaba { l, a, b, alpha } => {
                let c: palette::Hsla =
                    palette::Hsla::from_color(palette::Laba::new(l, a, b, alpha));

                gpui::Hsla {
                    h: f64::from(c.hue) as f32 / 360.,
                    s: c.saturation as f32,
                    l: c.lightness as f32,
                    a: c.alpha as f32,
                }
            }
            Color::OkLcha { l, c, h, a } => {
                let c: palette::Hsla = palette::Hsla::from_color(palette::Oklcha::new(l, c, h, a));

                gpui::Hsla {
                    h: f64::from(c.hue) as f32 / 360.,
                    s: c.saturation as f32,
                    l: c.lightness as f32,
                    a: c.alpha as f32,
                }
            }
            Color::OkLch { l, c, h } => {
                let t = palette::Oklch::new(l, c, h);
                let c: palette::Hsla = palette::Hsla::from_color(t);

                gpui::Hsla {
                    h: f64::from(c.hue) as f32 / 360.,
                    s: c.saturation,
                    l: c.lightness,
                    a: c.alpha,
                }
            }
            Color::Hsl { h, s, l } => gpui::hsla(h, s, l, 1.0),
            Color::Hsla { h, s, l, a } => gpui::hsla(h, s, l, a),
        }
    }
}
