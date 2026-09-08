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
    /// Modern macOS unified toolbar/header height (e.g. System Settings `AXToolbar`, strictly 52.0 pt).
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
    /// Corner radius of the floating menu container in modern macOS Liquid Glass (strictly 12.0 pt).
    ///
    /// Empirically verified via native `AppKit` `NSPopupMenuWindow` runtime inspection:
    /// - `CASDFElementLayer`: `cornerRadius = 12.0`, `cornerCurve = .continuous` (Apple Squircle)
    /// - Clipping `CALayer`: `cornerRadius = 12.0`, `cornerCurve = .continuous`
    ///
    /// Adheres to Apple HIG Concentric Radius Formula:
    /// `CONTAINER_CORNER_RADIUS` (12.0) = `ITEM_HIGHLIGHT_RADIUS` (7.0) + `CONTAINER_PADDING` (5.0)
    pub const CONTAINER_CORNER_RADIUS: f32 = 12.0;

    /// Classic macOS Big Sur / Monterey context menu container corner radius (10.0 pt).
    pub const CONTAINER_CORNER_RADIUS_CLASSIC: f32 = 10.0;

    /// Compact context menu container corner radius (8.0 pt).
    pub const CONTAINER_CORNER_RADIUS_COMPACT: f32 = 8.0;

    /// Inner padding around the menu items list (strictly 5.0 pt).
    ///
    /// Verified via native `AppKit` `NSTableRowView`: first row y-offset = 5.0 pt, container padding = 5.0 pt.
    pub const CONTAINER_PADDING: f32 = 5.0;

    /// Compact menu container padding (4.0 pt).
    pub const CONTAINER_PADDING_COMPACT: f32 = 4.0;

    /// Standard single-line menu item row height (24.0 pt).
    pub const ITEM_HEIGHT: f32 = 24.0;

    /// Compact single-line menu item row height (20.0 pt).
    pub const ITEM_HEIGHT_COMPACT: f32 = 20.0;

    /// Corner radius of the selection highlight pill for modern Liquid Glass menus (7.0 pt).
    ///
    /// Empirically verified via native `AppKit` `NSRootMenuWindowBackgroundView` runtime inspection:
    /// selection highlight `CALayer` has `cornerRadius = 7.0`.
    ///
    /// Satisfies Apple HIG Concentricity:
    /// `ITEM_HIGHLIGHT_RADIUS` = `CONTAINER_CORNER_RADIUS` (12.0) - `CONTAINER_PADDING` (5.0) = 7.0 pt.
    pub const ITEM_HIGHLIGHT_RADIUS: f32 = 7.0;

    /// Classic selection highlight pill radius for macOS Big Sur (5.0 pt = 10.0 - 5.0).
    pub const ITEM_HIGHLIGHT_RADIUS_CLASSIC: f32 = 5.0;

    /// Compact selection highlight pill radius (4.0 pt = 8.0 - 4.0).
    pub const ITEM_HIGHLIGHT_RADIUS_COMPACT: f32 = 4.0;

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

    // =========================================================================
    // Calibrated Physical Colorimetry (Ground Truth Measurement Data)
    // =========================================================================

    /// Calibrated opacity for Dark Mode context menu backdrops (202 / 255 ≈ 0.7922).
    ///
    /// Derived from physical measurement:
    /// - On white background (255, 255, 255): measured RGB = (86, 86, 86)
    /// - On black background (0, 0, 0): measured RGB = (33, 33, 33)
    pub const DARK_MENU_OPACITY: f32 = 202.0 / 255.0;

    /// Calibrated base RGB color for Dark Mode context menu (42, 42, 42).
    pub const DARK_MENU_BASE_RGB: (u8, u8, u8) = (42, 42, 42);

    /// Calibrated RGBA float components (r, g, b, a) for Dark Mode context menu.
    pub const DARK_MENU_BASE_RGBA_F32: (f32, f32, f32, f32) = (
        42.0 / 255.0,
        42.0 / 255.0,
        42.0 / 255.0,
        DARK_MENU_OPACITY,
    );

    /// Calibrated opacity for Light Mode context menu backdrops (185 / 255 ≈ 0.7255).
    ///
    /// Derived from physical measurement:
    /// - On white background (255, 255, 255): measured RGB = (255, 255, 255)
    /// - On black background (0, 0, 0): measured RGB = (185, 185, 185)
    pub const LIGHT_MENU_OPACITY: f32 = 185.0 / 255.0;

    /// Calibrated base RGB color for Light Mode context menu (255, 255, 255).
    pub const LIGHT_MENU_BASE_RGB: (u8, u8, u8) = (255, 255, 255);

    /// Calibrated RGBA float components (r, g, b, a) for Light Mode context menu.
    pub const LIGHT_MENU_BASE_RGBA_F32: (f32, f32, f32, f32) = (
        1.0,
        1.0,
        1.0,
        LIGHT_MENU_OPACITY,
    );
}

#[cfg(test)]
mod tests {
    use super::menu_metrics::*;

    #[test]
    fn test_menu_colorimetry_dark_mode_calibration() {
        let (r, _, _, a) = DARK_MENU_BASE_RGBA_F32;
        let base_val = r * 255.0;

        // 1. Overlay on black background (0, 0, 0): C_out = base * a + 0 * (1 - a)
        let black_out = base_val * a;
        assert_eq!(black_out.round() as u8, 33);

        // 2. Overlay on white background (255, 255, 255): C_out = base * a + 255 * (1 - a)
        let white_out = base_val * a + 255.0 * (1.0 - a);
        assert_eq!(white_out.round() as u8, 86);
    }

    #[test]
    fn test_menu_colorimetry_light_mode_calibration() {
        let (r, _, _, a) = LIGHT_MENU_BASE_RGBA_F32;
        let base_val = r * 255.0;

        // 1. Overlay on black background (0, 0, 0): C_out = 255 * a + 0
        let black_out = base_val * a;
        assert_eq!(black_out.round() as u8, 185);

        // 2. Overlay on white background (255, 255, 255): C_out = 255 * a + 255 * (1 - a) = 255
        let white_out = base_val * a + 255.0 * (1.0 - a);
        assert_eq!(white_out.round() as u8, 255);
    }

    #[test]
    fn test_menu_concentric_corner_radius_invariance() {
        // 1. Modern Liquid Glass concentricity: R_outer = R_inner + Padding
        assert_eq!(
            CONTAINER_CORNER_RADIUS,
            ITEM_HIGHLIGHT_RADIUS + CONTAINER_PADDING,
            "Apple HIG concentricity violation: outer radius must equal inner radius plus padding"
        );
        assert_eq!(CONTAINER_CORNER_RADIUS, 12.0);
        assert_eq!(ITEM_HIGHLIGHT_RADIUS, 7.0);
        assert_eq!(CONTAINER_PADDING, 5.0);

        // 2. Classic macOS Big Sur concentricity: 10.0 = 5.0 + 5.0
        assert_eq!(
            CONTAINER_CORNER_RADIUS_CLASSIC,
            ITEM_HIGHLIGHT_RADIUS_CLASSIC + CONTAINER_PADDING
        );
        assert_eq!(CONTAINER_CORNER_RADIUS_CLASSIC, 10.0);
        assert_eq!(ITEM_HIGHLIGHT_RADIUS_CLASSIC, 5.0);

        // 3. Compact variant concentricity: 8.0 = 4.0 + 4.0
        assert_eq!(
            CONTAINER_CORNER_RADIUS_COMPACT,
            ITEM_HIGHLIGHT_RADIUS_COMPACT + CONTAINER_PADDING_COMPACT
        );
        assert_eq!(CONTAINER_CORNER_RADIUS_COMPACT, 8.0);
        assert_eq!(ITEM_HIGHLIGHT_RADIUS_COMPACT, 4.0);
    }
}
