//! Framework-agnostic design tokens, metrics, and styling primitives.

pub mod font;
pub mod metrics;
pub mod theme;

pub use font::size as font_size;
pub use metrics::{
    dock as dock_metrics, DEFAULT_WINDOW_CORNER_RADIUS, menu_metrics, popover_metrics, scrollbar,
    traffic_lights, window_geometry, window_metrics,
};
pub use theme::{
    GlassChrome, GlassRole, UiColorScheme, UiCornerStyle, UiPalette, UiTheme,
};
