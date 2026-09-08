//! Standard metrics, layout constants, and physical measurements.

/// Default window corner radius for macOS modern borderless windows.
pub const DEFAULT_WINDOW_CORNER_RADIUS: f64 = 14.0;

/// Apple traffic lights metrics.
pub mod traffic_lights {
    /// Authentic macOS traffic light button diameter (strictly 14.0 pt / 28 px on 2x Retina).
    pub const DIAMETER: f32 = 14.0;
    /// Authentic macOS spacing between traffic light buttons (strictly 9.0 px).
    pub const SPACING: f32 = 9.0;
    /// Total width across the three lights (14 + 9 + 14 + 9 + 14 = 60.0 px).
    pub const TOTAL_WIDTH: f32 = 60.0;
    /// Authentic height matches the button diameter (14.0 pt).
    pub const HEIGHT: f32 = DIAMETER;
    /// Clearance distance from the right edge of traffic lights to the first letter of title (strictly 15.0 px).
    pub const TITLE_CLEARANCE: f32 = 15.0;
    /// Recommended horizontal clearance width including padding (78.0 px).
    pub const EXCLUSION_WIDTH: f32 = 78.0;

    /// Standard hover expansion slop around traffic light buttons for hit testing and gesture tracking.
    #[must_use]
    pub const fn control_hover_slop(size: f32) -> f32 {
        if size > 32.0 { 16.0 } else { 6.0 }
    }
}

/// Standard window layout metrics, header heights, and sidebar widths.
pub mod window_metrics {
    /// Modern macOS unified toolbar/header height (e.g. System Settings AXToolbar, strictly 52.0 pt).
    pub const FUSED_HEADER_HEIGHT: f32 = 52.0;

    /// Classic macOS standalone titlebar height (strictly 32.0 pt).
    pub const COMPACT_TITLEBAR_HEIGHT: f32 = 32.0;

    /// Comfortable standalone titlebar height with generous action clearance (38.0 pt).
    pub const COMFORTABLE_TITLEBAR_HEIGHT: f32 = 38.0;

    /// Modern macOS regular card-style sidebar width (232.0 pt, matching System Settings).
    pub const SIDEBAR_WIDTH_REGULAR: f32 = 232.0;

    /// Classic macOS compact sidebar width (220.0 pt).
    pub const SIDEBAR_WIDTH_COMPACT: f32 = 220.0;

    /// Recommended horizontal inset for sidebar items (10.0 pt).
    pub const SIDEBAR_CONTENT_INSET: f32 = 10.0;

    /// Standard continuous corner curvature radius for frameless windows (14.0 pt).
    pub const DEFAULT_CORNER_RADIUS: f32 = 14.0;
}

/// Spring scrollbar dimensions.
pub mod scrollbar {
    pub const WIDTH_RESTING: f32 = 6.0;
    pub const WIDTH_EXPANDED: f32 = 11.0;
    pub const MIN_THUMB_LENGTH: f32 = 24.0;
    pub const INSET: f32 = 3.0;
}

/// Standard context menu and popover metrics matching Apple HIG.
pub mod menu_metrics {
    /// Corner radius of the floating menu container (strictly 8.0 pt).
    pub const CONTAINER_CORNER_RADIUS: f32 = 8.0;

    /// Inner padding around the menu items list (strictly 5.0 pt).
    pub const CONTAINER_PADDING: f32 = 5.0;

    /// Standard single-line menu item row height (24.0 pt).
    pub const ITEM_HEIGHT: f32 = 24.0;

    /// Compact single-line menu item row height (20.0 pt).
    pub const ITEM_HEIGHT_COMPACT: f32 = 20.0;

    /// Corner radius of the selection highlight pill (4.5 pt).
    pub const ITEM_HIGHLIGHT_RADIUS: f32 = 4.5;

    /// Horizontal padding inside each menu item (8.0 pt).
    pub const ITEM_HORIZONTAL_PADDING: f32 = 8.0;

    /// Width reserved for the leading icon or checkmark slot (18.0 pt).
    pub const LEADING_SLOT_WIDTH: f32 = 18.0;

    /// Icon display size inside a menu item (14.0 pt).
    pub const ICON_SIZE: f32 = 14.0;

    /// Height of a menu separator line (1.0 pt).
    pub const SEPARATOR_HEIGHT: f32 = 1.0;

    /// Vertical margin above and below a separator line (5.0 pt).
    pub const SEPARATOR_MARGIN_V: f32 = 5.0;

    /// Default recommended minimum width for context menus (180.0 pt).
    pub const MIN_WIDTH: f32 = 180.0;

    /// Default recommended width for general desktop context menus (220.0 pt).
    pub const DEFAULT_WIDTH: f32 = 220.0;

    /// Section header label height (18.0 pt).
    pub const SECTION_HEADER_HEIGHT: f32 = 18.0;
}
