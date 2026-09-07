//! Standard metrics, layout constants, and physical measurements.

/// Default window corner radius for macOS modern borderless windows.
pub const DEFAULT_WINDOW_CORNER_RADIUS: f64 = 14.0;

/// Apple traffic lights metrics.
pub mod traffic_lights {
    pub const DIAMETER: f32 = 12.0;
    pub const SPACING: f32 = 8.0;
    pub const LEADING_MARGIN: f32 = 20.0;
    pub const TOP_MARGIN: f32 = 18.0;
}

/// Spring scrollbar dimensions.
pub mod scrollbar {
    pub const WIDTH_RESTING: f32 = 6.0;
    pub const WIDTH_EXPANDED: f32 = 11.0;
    pub const MIN_THUMB_LENGTH: f32 = 24.0;
    pub const INSET: f32 = 3.0;
}
