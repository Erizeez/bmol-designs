//! Standard metrics, layout constants, and physical measurements.

/// Default window corner radius for macOS modern borderless windows.
///
/// Always derived from the *separate* titlebar height (see
/// [`window_geometry`]), even for unified/fused chrome, so the outer
/// curvature is constant across layout modes.
pub const DEFAULT_WINDOW_CORNER_RADIUS: f64 = window_geometry::CORNER_RADIUS as f64;

/// Apple traffic lights metrics.
pub mod traffic_lights {
    /// Authentic macOS traffic light button diameter (strictly 14.0 pt / 28 px on 2x Retina).
    pub const DIAMETER: f32 = 14.0;
    /// Gap between adjacent traffic light buttons (`H / 2`, 16.0 pt for `H = 32`).
    pub const SPACING: f32 = 16.0;
    /// Distance from the left window edge to the leftmost edge of the red light (9.0 pt).
    pub const LEADING_MARGIN: f32 = 9.0;
    /// Total width across the three lights (14 + 16 + 14 + 16 + 14 = 74.0 pt).
    pub const TOTAL_WIDTH: f32 = 74.0;
    /// Authentic height matches the button diameter (14.0 pt).
    pub const HEIGHT: f32 = DIAMETER;
    /// Clearance distance from the right edge of traffic lights to the first letter of title (strictly 15.0 px).
    pub const TITLE_CLEARANCE: f32 = 15.0;
    /// Recommended horizontal clearance width including padding (9 + 74 + 8 = 91.0 pt).
    pub const EXCLUSION_WIDTH: f32 = 91.0;

    /// Standard hover expansion slop around traffic light buttons for hit testing and gesture tracking.
    #[must_use]
    pub const fn control_hover_slop(size: f32) -> f32 {
        if size > 32.0 { 16.0 } else { 6.0 }
    }
}

/// Window geometry derived from the standalone titlebar height `H`.
///
/// `H` is the measured separate-mode titlebar height. Curvature, traffic-light
/// placement, and spacing are all derived from it, so the whole window geometry
/// has a single source of truth and the red light stays concentric with the
/// window corner.
pub mod window_geometry {
    use super::traffic_lights;

    /// Measured standalone titlebar height (32.0 pt).
    pub const TITLEBAR_HEIGHT: f32 = 32.0;

    /// Window corner radius, always derived from the *separate* titlebar height
    /// so unified/fused chrome keeps the same outer curvature.
    pub const CORNER_RADIUS: f32 = corner_radius(TITLEBAR_HEIGHT);

    /// Window corner radius `R = H / 2`, concentric with the red traffic light.
    #[must_use]
    pub const fn corner_radius(h: f32) -> f32 {
        h / 2.0
    }

    /// Red (close) traffic-light centre `(H / 2, H / 2)`.
    #[must_use]
    pub const fn traffic_light_center(h: f32) -> (f32, f32) {
        (h / 2.0, h / 2.0)
    }

    /// Gap between adjacent traffic-light circles `H / 2`.
    #[must_use]
    pub const fn traffic_light_spacing(h: f32) -> f32 {
        h / 2.0
    }

    /// Centre-to-centre pitch between traffic lights (diameter + spacing).
    #[must_use]
    pub const fn traffic_light_pitch(h: f32) -> f32 {
        traffic_lights::DIAMETER + traffic_light_spacing(h)
    }

    /// Total width across the three traffic lights.
    #[must_use]
    pub const fn traffic_lights_total_width(h: f32) -> f32 {
        traffic_lights::DIAMETER * 3.0 + traffic_light_spacing(h) * 2.0
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

    /// Standard continuous corner curvature radius for frameless windows.
    ///
    /// Always the separate-mode `H / 2`, independent of the active layout mode.
    pub const DEFAULT_CORNER_RADIUS: f32 = super::window_geometry::CORNER_RADIUS;

    /// Border hit zone thickness for edge resize handles (6.0 pt).
    pub const RESIZE_BORDER_THICKNESS: f32 = 6.0;

    /// Corner hit zone square size for corner resize handles (14.0 pt).
    pub const RESIZE_CORNER_SIZE: f32 = 14.0;
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

/// Standard popover arrow / beak metrics and anchor placement matching Apple HIG.
pub mod popover_metrics {
    /// Standard base width of the popover arrow at the card junction (21.0 pt).
    ///
    /// Calibrated to native macOS Dock context menu specifications: 21.0 pt base span with 9.0 pt height.
    pub const ARROW_BASE_WIDTH: f32 = 21.0;

    /// Compact popover arrow base width (20.0 pt).
    pub const ARROW_BASE_WIDTH_COMPACT: f32 = 20.0;

    /// Large popover arrow base width for prominent HUDs (32.0 pt).
    pub const ARROW_BASE_WIDTH_LARGE: f32 = 32.0;

    /// Standard protruding height of the popover arrow (9.0 pt).
    ///
    /// Extends outward from the squircle bounding box towards the target anchor (18 px @2x).
    pub const ARROW_HEIGHT: f32 = 9.0;

    /// Compact popover arrow height (7.0 pt).
    pub const ARROW_HEIGHT_COMPACT: f32 = 7.0;

    /// Large popover arrow height (12.0 pt).
    pub const ARROW_HEIGHT_LARGE: f32 = 12.0;

    /// Tip corner radius of the arrow (1.8 pt).
    ///
    /// Ensures the apex transitions smoothly while allowing the flanks to maintain
    /// their signature concave (flared) silhouette rather than a bulging convex dome.
    pub const ARROW_TIP_RADIUS: f32 = 1.8;

    /// Smooth base fillet radius connecting the arrow sloped sides into the card edge (5.5 pt).
    ///
    /// Provides continuous tangent transitions (G1/G2) to avoid sharp reentrant corners.
    pub const ARROW_BASE_FILLET: f32 = 5.5;

    /// Minimum clearance from arrow base to the nearest corner of the card container (16.0 pt).
    ///
    /// Prevents the arrow fillet from clashing with the container squircle arc.
    pub const MIN_CORNER_CLEARANCE: f32 = 16.0;

    /// Default center distance of the popover arrow from the left edge of macOS Dock menus (27.0 pt).
    ///
    /// In native macOS, context menus triggered by clicking or long-pressing Dock icons
    /// anchor their bottom protruding arrow apex at exactly 27.0 pt from the card's left boundary.
    pub const DOCK_MENU_ARROW_CENTER_OFFSET: f32 = 27.0;

    // --- Subpixel-fitted Apple Popover Bézier Spline Constants ---
    // Derived from Nelder-Mead optimization against native macOS screenshots (RMSE < 0.1 px).
    // The profile consists of two C1-continuous cubic Bézier segments per symmetrical half:
    // Apex P0 = (0, 1), P1 = (APEX_CTRL_U, 1), P2 = (UPPER_FLANK_U, UPPER_FLANK_V), P3 = (INFLECTION_U, INFLECTION_V)
    // Inflection Q0 = P3, Q1 = (LOWER_FLANK_U, LOWER_FLANK_V), Q2 = (BASE_CTRL_U, 0), Q3 = (1, 0)

    /// Symmetrical half Bézier spline parameters (normalized to u in [0, 1], v in [0, 1]).
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct PopoverSplineConstants {
        pub apex_ctrl_u: f32,
        pub upper_flank_u: f32,
        pub upper_flank_v: f32,
        pub inflection_u: f32,
        pub inflection_v: f32,
        pub lower_flank_u: f32,
        pub lower_flank_v: f32,
        pub base_ctrl_u: f32,
    }

    /// Subpixel-fitted spline constants for broad popover / Dock context menu arrows (RMSE = 0.081 pt).
    /// Measured from native macOS Dock context menu / `NSPopover` with gentle organic root flare.
    pub const MENU_WIDE_SPLINE: PopoverSplineConstants = PopoverSplineConstants {
        apex_ctrl_u: 0.20930,
        upper_flank_u: 0.30582,
        upper_flank_v: 0.84793,
        inflection_u: 0.43264,
        inflection_v: 0.61628,
        lower_flank_u: 0.52699,
        lower_flank_v: 0.36000,
        base_ctrl_u: 0.83000,
    };

    /// Unified continuous curvature spline constants for macOS popover arrows (RMSE = 0.081 pt).
    /// Both wide (21×9) and narrow (20×7) macOS popover arrows share this exact
    /// broad-dome continuous curvature spline profile.
    pub const TOOLTIP_NARROW_SPLINE: PopoverSplineConstants = MENU_WIDE_SPLINE;

    // Aliases for MenuWide spline constants for backwards-compatibility:
    pub const ARROW_SPLINE_APEX_CTRL_U: f32 = MENU_WIDE_SPLINE.apex_ctrl_u;
    pub const ARROW_SPLINE_UPPER_FLANK_U: f32 = MENU_WIDE_SPLINE.upper_flank_u;
    pub const ARROW_SPLINE_UPPER_FLANK_V: f32 = MENU_WIDE_SPLINE.upper_flank_v;
    pub const ARROW_SPLINE_INFLECTION_U: f32 = MENU_WIDE_SPLINE.inflection_u;
    pub const ARROW_SPLINE_INFLECTION_V: f32 = MENU_WIDE_SPLINE.inflection_v;
    pub const ARROW_SPLINE_LOWER_FLANK_U: f32 = MENU_WIDE_SPLINE.lower_flank_u;
    pub const ARROW_SPLINE_LOWER_FLANK_V: f32 = MENU_WIDE_SPLINE.lower_flank_v;
    pub const ARROW_SPLINE_BASE_CTRL_U: f32 = MENU_WIDE_SPLINE.base_ctrl_u;

    /// Evaluates the normalized height v/ha in [0.0, 1.0] for a given lateral distance |u|/wb in [0.0, 1.0]
    /// using the specified spline constants.
    /// Solves the subpixel-fitted Apple popover Bézier spline using Newton-Raphson iterations.
    #[must_use]
    pub fn popover_arrow_profile_height_with_spline(
        spline: &PopoverSplineConstants,
        normalized_u: f32,
    ) -> f32 {
        let u = normalized_u.clamp(0.0, 1.0);
        if u <= 0.0 {
            return 1.0;
        }
        if u >= 1.0 {
            return 0.0;
        }

        if u < spline.inflection_u {
            let u_end = spline.inflection_u;
            let mut t = (u / u_end).clamp(0.0, 1.0);
            for _ in 0..3 {
                let inv = 1.0 - t;
                let x = 3.0 * inv * inv * t * spline.apex_ctrl_u
                    + 3.0 * inv * t * t * spline.upper_flank_u
                    + t * t * t * spline.inflection_u;
                let dx = 3.0 * inv * inv * spline.apex_ctrl_u
                    + 6.0 * inv * t * (spline.upper_flank_u - spline.apex_ctrl_u)
                    + 3.0 * t * t * (spline.inflection_u - spline.upper_flank_u);
                if dx.abs() > 1e-5 {
                    t = (t - (x - u) / dx).clamp(0.0, 1.0);
                }
            }
            let inv = 1.0 - t;
            inv * inv * inv
                + 3.0 * inv * inv * t
                + 3.0 * inv * t * t * spline.upper_flank_v
                + t * t * t * spline.inflection_v
        } else {
            let u_start = spline.inflection_u;
            let mut t = ((u - u_start) / (1.0 - u_start)).clamp(0.0, 1.0);
            for _ in 0..3 {
                let inv = 1.0 - t;
                let x = inv * inv * inv * spline.inflection_u
                    + 3.0 * inv * inv * t * spline.lower_flank_u
                    + 3.0 * inv * t * t * spline.base_ctrl_u
                    + t * t * t;
                let dx = 3.0 * inv * inv * (spline.lower_flank_u - spline.inflection_u)
                    + 6.0 * inv * t * (spline.base_ctrl_u - spline.lower_flank_u)
                    + 3.0 * t * t * (1.0 - spline.base_ctrl_u);
                if dx.abs() > 1e-5 {
                    t = (t - (x - u) / dx).clamp(0.0, 1.0);
                }
            }
            let inv = 1.0 - t;
            inv * inv * inv * spline.inflection_v
                + 3.0 * inv * inv * t * spline.lower_flank_v
        }
    }

    /// Evaluates the normalized height v/ha in [0.0, 1.0] for a given lateral distance |u|/wb in [0.0, 1.0]
    /// using the default `MenuWide` spline.
    #[must_use]
    pub fn popover_arrow_profile_height(normalized_u: f32) -> f32 {
        popover_arrow_profile_height_with_spline(&MENU_WIDE_SPLINE, normalized_u)
    }

    /// Edge on which the popover arrow protrudes towards its anchor target.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum PopoverArrowEdge {
        /// Protrudes from the top edge (pointing upwards at anchor above).
        Top,
        /// Protrudes from the bottom edge (pointing downwards at anchor below).
        Bottom,
        /// Protrudes from the left edge (pointing leftwards).
        Left,
        /// Protrudes from the right edge (pointing rightwards).
        Right,
        /// No arrow protruding (standalone floating card).
        #[default]
        None,
    }

    /// Preset styles for macOS popover and tooltip arrows matching system behaviors.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum PopoverArrowPreset {
        /// Authentic native macOS Dock context menu arrow with smooth organic root flare (Base 21.0 pt, Height 9.0 pt, Tip 1.8 pt, Fillet 5.0 pt).
        #[default]
        MenuWide,
        /// Standard `AppKit` / `SwiftUI` `NSPopover` system default (Base 27.5 pt, Height 13.0 pt, Tip 2.0 pt, Fillet 6.0 pt).
        AppKitStandard,
        /// Narrow, slender tooltip arrow used in Dock icon hover labels, Cartouche popovers,
        /// and compact tooltips (Base 20.0 pt, Height 7.0 pt, Tip 1.0 pt, Fillet 4.0 pt).
        TooltipNarrow,
        /// Minimal subtle pointer for ultra-compact controls (Base 12.0 pt, Height 5.0 pt, Tip 1.0 pt, Fillet 2.5 pt).
        SubtleCompact,
    }

    impl PopoverArrowPreset {
        /// Returns `(base_width, height, tip_radius, base_fillet)` for this preset.
        #[must_use]
        pub const fn metrics(self) -> (f32, f32, f32, f32) {
            match self {
                Self::MenuWide => (ARROW_BASE_WIDTH, ARROW_HEIGHT, ARROW_TIP_RADIUS, 5.0),
                Self::AppKitStandard => (27.5, 13.0, 2.0, 6.0),
                Self::TooltipNarrow => (ARROW_BASE_WIDTH_COMPACT, ARROW_HEIGHT_COMPACT, 1.0, 4.0),
                Self::SubtleCompact => (12.0, 5.0, 1.0, 2.5),
            }
        }

        /// Returns the authentic subpixel-fitted Bézier spline parameters for this preset.
        /// All presets share the unified generous continuous curvature profile.
        #[must_use]
        pub const fn spline(self) -> PopoverSplineConstants {
            MENU_WIDE_SPLINE
        }
    }

    /// Configuration parameters for a popover arrow.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct PopoverArrowConfig {
        pub edge: PopoverArrowEdge,
        pub base_width: f32,
        pub height: f32,
        pub tip_radius: f32,
        pub base_fillet: f32,
        /// Normalized position along the edge (0.0 = start, 0.5 = center, 1.0 = end).
        pub offset: f32,
        /// Spline curvature constants determining the flank and apex profile.
        pub spline: PopoverSplineConstants,
    }

    impl Default for PopoverArrowConfig {
        fn default() -> Self {
            Self::from_preset(PopoverArrowEdge::None, PopoverArrowPreset::MenuWide)
        }
    }

    impl PopoverArrowConfig {
        #[must_use]
        pub const fn new(edge: PopoverArrowEdge) -> Self {
            Self::from_preset(edge, PopoverArrowPreset::MenuWide)
        }

        #[must_use]
        pub const fn from_preset(edge: PopoverArrowEdge, preset: PopoverArrowPreset) -> Self {
            let (base_width, height, tip_radius, base_fillet) = preset.metrics();
            Self {
                edge,
                base_width,
                height,
                tip_radius,
                base_fillet,
                offset: 0.5,
                spline: preset.spline(),
            }
        }

        #[must_use]
        pub const fn with_preset(mut self, preset: PopoverArrowPreset) -> Self {
            let (base_width, height, tip_radius, base_fillet) = preset.metrics();
            self.base_width = base_width;
            self.height = height;
            self.tip_radius = tip_radius;
            self.base_fillet = base_fillet;
            self.spline = preset.spline();
            self
        }

        #[must_use]
        pub const fn with_spline(mut self, spline: PopoverSplineConstants) -> Self {
            self.spline = spline;
            self
        }

        #[must_use]
        pub const fn with_offset(mut self, offset: f32) -> Self {
            self.offset = offset;
            self
        }

        #[must_use]
        pub const fn with_size(mut self, base_width: f32, height: f32) -> Self {
            self.base_width = base_width;
            self.height = height;
            self
        }

        #[must_use]
        pub const fn with_curvature(mut self, tip_radius: f32, base_fillet: f32) -> Self {
            self.tip_radius = tip_radius;
            self.base_fillet = base_fillet;
            self
        }

        #[must_use]
        pub const fn is_visible(&self) -> bool {
            !matches!(self.edge, PopoverArrowEdge::None) && self.height > 0.0 && self.base_width > 0.0
        }

        /// Computes the normalized arrow offset [0.0, 1.0] for a standard Dock context menu
        /// of the given card width based on the default 27.0 pt anchor center distance.
        #[must_use]
        pub const fn dock_menu_offset(card_width: f32) -> f32 {
            if card_width > 0.0 {
                DOCK_MENU_ARROW_CENTER_OFFSET / card_width
            } else {
                0.5
            }
        }
    }
}

/// Standard Dock & icon layout metrics adhering to Apple concentric squircle geometry.
pub mod dock {
    /// Standard base icon width and height (40.0 pt).
    pub const BASE_ICON_SIZE: f32 = 40.0;

    /// Ratio of icon corner radius to icon dimension (10 / 40 = 0.25).
    /// Satisfies the authentic 10 : 20 : 10 curvature ratio:
    /// - 10px corner arc (25%)
    /// - 20px straight flat edge (50%)
    /// - 10px corner arc (25%)
    pub const ICON_CORNER_RATIO: f32 = 0.25;

    /// Ratio of dock padding relative to icon dimension (13 / 40 = 0.325).
    pub const PADDING_RATIO: f32 = 13.0 / 40.0;

    /// Ratio of icon gap relative to icon dimension (13 / 40 = 0.325).
    pub const GAP_RATIO: f32 = 13.0 / 40.0;

    /// Standard icon corner radius (10.0 pt).
    pub const ICON_CORNER_RADIUS: f32 = BASE_ICON_SIZE * ICON_CORNER_RATIO;

    /// Standard inner padding around the dock icons (13.0 pt).
    pub const DOCK_PADDING: f32 = BASE_ICON_SIZE * PADDING_RATIO;

    /// Standard gap between dock icons (13.0 pt).
    pub const ICON_GAP: f32 = BASE_ICON_SIZE * GAP_RATIO;

    /// Apple concentric dock corner radius rule:
    /// R_dock = Padding + R_icon = 13.0 + 10.0 = 23.0 pt.
    pub const DOCK_CORNER_RADIUS: f32 = DOCK_PADDING + ICON_CORNER_RADIUS;

    /// Standard dock plate height (40.0 + 13.0 * 2 = 66.0 pt).
    pub const DOCK_HEIGHT: f32 = BASE_ICON_SIZE + DOCK_PADDING * 2.0;

    /// Ratio of dock corner radius relative to icon size (23 / 40 = 0.575).
    pub const DOCK_CORNER_RADIUS_RATIO: f32 = 23.0 / 40.0;

    /// Computes the concentric dock corner radius for any arbitrary icon radius and container padding:
    /// `R_dock = padding + icon_radius`.
    #[inline]
    #[must_use]
    pub const fn concentric_dock_radius(icon_radius: f32, padding: f32) -> f32 {
        icon_radius + padding
    }

    /// Computes icon corner radius for a given icon size maintaining the 10:20:10 ratio.
    #[inline]
    #[must_use]
    pub fn icon_corner_radius(icon_size: f32) -> f32 {
        icon_size * ICON_CORNER_RATIO
    }

    /// Computes dock inner padding for a given icon size maintaining the 13/40 ratio.
    #[inline]
    #[must_use]
    pub fn dock_padding(icon_size: f32) -> f32 {
        icon_size * PADDING_RATIO
    }

    /// Computes gap between icons for a given icon size maintaining the 13/40 ratio.
    #[inline]
    #[must_use]
    pub fn icon_gap(icon_size: f32) -> f32 {
        icon_size * GAP_RATIO
    }

    /// Computes dock plate height for a given icon size (`icon_size + 2 * padding`).
    #[inline]
    #[must_use]
    pub fn dock_height(icon_size: f32) -> f32 {
        icon_size + dock_padding(icon_size) * 2.0
    }

    /// Computes total dock width for N icons given an icon size.
    #[inline]
    #[must_use]
    pub fn dock_width(icon_count: usize, icon_size: f32) -> f32 {
        if icon_count == 0 {
            return 0.0;
        }
        let padding = dock_padding(icon_size);
        let gap = icon_gap(icon_size);
        (icon_count as f32) * icon_size
            + ((icon_count.saturating_sub(1)) as f32) * gap
            + padding * 2.0
    }
}

#[cfg(test)]
#[allow(
    clippy::assertions_on_constants,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
mod tests {
    use super::menu_metrics::*;

    #[test]
    fn window_geometry_derives_from_separate_titlebar_height() {
        use super::window_geometry;

        // H = 32 (measured separate titlebar height).
        assert_eq!(window_geometry::TITLEBAR_HEIGHT, 32.0);

        // Curvature is always the separate H / 2, independent of layout mode.
        assert_eq!(window_geometry::CORNER_RADIUS, 16.0);
        assert_eq!(window_geometry::corner_radius(32.0), 16.0);

        // The red traffic light is concentric with the corner: centre (H/2, H/2).
        assert_eq!(window_geometry::traffic_light_center(32.0), (16.0, 16.0));

        // Spacing is the separate H / 2 = 16 (fixed across modes).
        assert_eq!(window_geometry::traffic_light_spacing(32.0), 16.0);
        assert_eq!(window_geometry::traffic_light_pitch(32.0), 30.0); // 14 + 16
        assert_eq!(window_geometry::traffic_lights_total_width(32.0), 74.0); // 14*3 + 16*2
    }

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

    #[test]
    fn test_popover_metrics_geometry_constraints() {
        use super::popover_metrics::*;

        assert!(ARROW_BASE_WIDTH > ARROW_HEIGHT);
        assert!(ARROW_BASE_WIDTH >= 16.0);
        assert!(ARROW_HEIGHT >= 8.0);
        assert!(ARROW_TIP_RADIUS >= 1.0);
        assert!(ARROW_BASE_FILLET >= 3.0);
        assert!(MIN_CORNER_CLEARANCE >= CONTAINER_CORNER_RADIUS);
        assert_eq!(DOCK_MENU_ARROW_CENTER_OFFSET, 27.0);
        let dock_offset = PopoverArrowConfig::dock_menu_offset(154.0);
        assert!((dock_offset - 27.0 / 154.0).abs() < 1e-6);

        let default_cfg = PopoverArrowConfig::default();
        assert!(!default_cfg.is_visible());

        let top_cfg = PopoverArrowConfig::new(PopoverArrowEdge::Top);
        assert!(top_cfg.is_visible());
        assert_eq!(top_cfg.offset, 0.5);

        let custom_cfg = PopoverArrowConfig::new(PopoverArrowEdge::Bottom)
            .with_offset(0.75)
            .with_size(24.0, 12.0);
        assert!(custom_cfg.is_visible());
        assert_eq!(custom_cfg.offset, 0.75);
        assert_eq!(custom_cfg.base_width, 24.0);
        assert_eq!(custom_cfg.height, 12.0);

        // Verify presets
        let menu_wide = PopoverArrowPreset::MenuWide.metrics();
        assert_eq!(menu_wide, (21.0, 9.0, 1.8, 5.0));

        let tooltip_narrow = PopoverArrowPreset::TooltipNarrow.metrics();
        assert_eq!(tooltip_narrow, (20.0, 7.0, 1.0, 4.0));

        let appkit_std = PopoverArrowPreset::AppKitStandard.metrics();
        assert_eq!(appkit_std, (27.5, 13.0, 2.0, 6.0));

        let subtle = PopoverArrowPreset::SubtleCompact.metrics();
        assert_eq!(subtle, (12.0, 5.0, 1.0, 2.5));

        let tooltip_cfg = PopoverArrowConfig::from_preset(PopoverArrowEdge::Bottom, PopoverArrowPreset::TooltipNarrow);
        assert_eq!(tooltip_cfg.base_width, 20.0);
        assert_eq!(tooltip_cfg.height, 7.0);
        assert_eq!(tooltip_cfg.tip_radius, 1.0);
        assert_eq!(tooltip_cfg.spline, TOOLTIP_NARROW_SPLINE);

        // Verify subpixel-fitted Bézier profile curve (MenuWide)
        assert!((popover_arrow_profile_height(0.0) - 1.0).abs() < 1e-4);
        assert!((popover_arrow_profile_height(1.0) - 0.0).abs() < 1e-4);
        let h_inf = popover_arrow_profile_height(ARROW_SPLINE_INFLECTION_U);
        assert!((h_inf - ARROW_SPLINE_INFLECTION_V).abs() < 0.01);

        // Verify subpixel-fitted Bézier profile curve (TooltipNarrow)
        assert!((popover_arrow_profile_height_with_spline(&TOOLTIP_NARROW_SPLINE, 0.0) - 1.0).abs() < 1e-4);
        assert!((popover_arrow_profile_height_with_spline(&TOOLTIP_NARROW_SPLINE, 1.0) - 0.0).abs() < 1e-4);
        let h_inf_tt = popover_arrow_profile_height_with_spline(&TOOLTIP_NARROW_SPLINE, TOOLTIP_NARROW_SPLINE.inflection_u);
        assert!((h_inf_tt - TOOLTIP_NARROW_SPLINE.inflection_v).abs() < 0.01);

        // Monotonic decrease
        let mut prev = 1.0;
        for i in 1..=20 {
            let u = i as f32 / 20.0;
            let val = popover_arrow_profile_height(u);
            assert!(val <= prev + 1e-5);
            prev = val;
        }
    }

    #[test]
    fn test_dock_concentric_proportions() {
        use super::dock::*;

        // 1. Icon 10:20:10 Curvature Check:
        assert_eq!(BASE_ICON_SIZE, 40.0);
        assert_eq!(ICON_CORNER_RADIUS, 10.0);
        let straight_edge = BASE_ICON_SIZE - ICON_CORNER_RADIUS * 2.0;
        assert_eq!(straight_edge, 20.0);
        assert_eq!(ICON_CORNER_RADIUS / BASE_ICON_SIZE, 0.25);
        assert_eq!(straight_edge / BASE_ICON_SIZE, 0.50);

        // 2. Padding and Gap Check:
        assert_eq!(DOCK_PADDING, 13.0);
        assert_eq!(ICON_GAP, 13.0);
        assert_eq!(PADDING_RATIO, 13.0 / 40.0);
        assert_eq!(GAP_RATIO, 13.0 / 40.0);

        // 3. Apple Concentric Dock Radius Rule:
        // R_dock = Padding + R_icon = 13 + 10 = 23.0 pt.
        assert_eq!(DOCK_CORNER_RADIUS, 23.0);
        assert_eq!(
            concentric_dock_radius(ICON_CORNER_RADIUS, DOCK_PADDING),
            23.0
        );

        // 4. Plate Height & Width:
        assert_eq!(DOCK_HEIGHT, 66.0); // 40 + 13 * 2 = 66
        assert_eq!(dock_height(BASE_ICON_SIZE), 66.0);

        // 9 icons: 9 * 40 + 8 * 13 + 2 * 13 = 360 + 104 + 26 = 490
        assert_eq!(dock_width(9, BASE_ICON_SIZE), 490.0);
    }
}
