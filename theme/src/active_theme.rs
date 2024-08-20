use std::sync::Arc;

use derive_more::derive::{Deref, DerefMut};
use gpui::{AppContext, Global};
use ui::ViewContext;

use crate::Theme;

#[derive(Default, Deref, DerefMut)]
pub struct ActiveTheme(Arc<Theme>);
impl Global for ActiveTheme {}

impl ActiveTheme {
    pub fn global(cx: &AppContext) -> Arc<Theme> {
        cx.global::<ActiveTheme>().0.clone()
    }

    pub fn default_global(cx: &mut AppContext) -> Arc<Theme> {
        cx.default_global::<ActiveTheme>().0.clone()
    }

    pub(crate) fn set_global(cx: &mut AppContext) {
        cx.set_global(ActiveTheme(Arc::new(Theme::default())))
    }

    pub fn init(cx: &mut AppContext) {
        Self::set_global(cx)
    }
}
pub trait ActiveThemeTrait {
    fn theme(&self) -> Arc<Theme>;
}
pub trait ActiveThemeTraitMut {
    fn theme(&mut self) -> Arc<Theme>;
}
impl ActiveThemeTrait for AppContext {
    fn theme(&self) -> Arc<Theme> {
        ActiveTheme::global(self)
    }
}
impl ActiveThemeTraitMut for AppContext {
    fn theme(&mut self) -> Arc<Theme> {
        ActiveTheme::global(self)
    }
}
impl<'a, T> ActiveThemeTrait for ViewContext<'a, T> {
    fn theme(&self) -> Arc<Theme> {
        ActiveTheme::global(self)
    }
}
impl<'a, T> ActiveThemeTraitMut for ViewContext<'a, T> {
    fn theme(&mut self) -> Arc<Theme> {
        ActiveTheme::global(self)
    }
}
