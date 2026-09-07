//! Framework-agnostic design tokens, metrics, and styling primitives.

pub mod font;
pub mod metrics;
pub mod theme;

pub use font::size as font_size;
pub use metrics::{DEFAULT_WINDOW_CORNER_RADIUS, scrollbar, traffic_lights};
pub use theme::{
    GlassChrome, GlassRole, UiColorScheme, UiCornerStyle, UiPalette, UiTheme,
};
