//! Semantic colors, corner styles, and glass materials for design systems.

use liquid_rs::{
    geometry::{APPLE_CORNER_SMOOTHING, SquircleParams},
    scene::{
        Color, GlareStyle, GlassMaterial, GlassShape, GlassVariant, ShadowStyle,
    },
};

/// The corner model following Apple continuous curvature (Squircle).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiCornerStyle {
    radius: f32,
    smoothing: f32,
}

impl UiCornerStyle {
    pub const GROUP: Self = Self::new(10.0);
    pub const CONTROL: Self = Self::new(8.0);
    pub const PICKER: Self = Self::new(6.0);
    pub const MENU: Self = Self::new(8.0);
    pub const SCROLLBAR: Self = Self::new(8.0);
    pub const ICON_CHIP: Self = Self::new(8.0);
    pub const WINDOW: Self = Self::new(14.0);

    #[must_use]
    pub const fn new(radius: f32) -> Self {
        Self { radius, smoothing: APPLE_CORNER_SMOOTHING }
    }

    #[must_use]
    pub const fn with_smoothing(self, smoothing: f32) -> Self {
        Self { smoothing, ..self }
    }

    #[must_use]
    pub const fn with_radius(self, radius: f32) -> Self {
        Self { radius, ..self }
    }

    #[must_use]
    pub const fn radius(self) -> f32 {
        self.radius
    }

    #[must_use]
    pub const fn smoothing(self) -> f32 {
        self.smoothing
    }

    #[must_use]
    pub fn params(self, width: f32, height: f32) -> SquircleParams {
        SquircleParams::new(width, height, self.radius).with_smoothing(self.smoothing)
    }
}

/// The resolved light or dark tone used to render an application window.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum UiColorScheme {
    Light,
    #[default]
    Dark,
}

/// A semantic role whose material can vary with the active color scheme.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GlassRole {
    /// A full-height split-view surface that reveals and heavily blurs the
    /// desktop backdrop.
    Sidebar,
    Toolbar,
    InputField,
    /// Compatibility role for search-specific input fields.
    SearchField,
    FloatingControl,
}

/// Chrome drawn above a compositor-provided glass surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlassChrome {
    pub border: Color,
    pub hover_border: Color,
    pub divider: Color,
    pub shadow: Color,
    pub text: Color,
    pub disabled_text: Color,
    pub hover_overlay: Color,
    pub pressed_overlay: Color,
    pub shadow_offset_y: f32,
    pub shadow_blur: f32,
}

/// Semantic colors for regular application UI.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiPalette {
    pub window_background: Color,
    pub sidebar_background: Color,
    pub content_background: Color,
    pub group_background: Color,
    pub group_border: Color,
    pub separator: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    /// De-emphasized text such as disclosure chevrons and placeholders.
    pub text_tertiary: Color,
    pub accent: Color,
    /// Accent-tinted selection used inside lists and text fields.
    pub selection: Color,
    /// Neutral pill drawn behind the selected sidebar item.
    pub sidebar_selection: Color,
    pub hover: Color,
    /// The track of an off toggle switch.
    pub control_track_off: Color,
    pub shadow: Color,
}

/// Theme object used by UI integrations and compositor adapters.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiTheme {
    scheme: UiColorScheme,
}

impl UiTheme {
    #[must_use]
    pub const fn new(scheme: UiColorScheme) -> Self {
        Self { scheme }
    }

    #[must_use]
    pub const fn light() -> Self {
        Self::new(UiColorScheme::Light)
    }

    #[must_use]
    pub const fn dark() -> Self {
        Self::new(UiColorScheme::Dark)
    }

    #[must_use]
    pub const fn scheme(self) -> UiColorScheme {
        self.scheme
    }

    /// Returns semantic colors modelled after macOS split-view settings windows.
    #[must_use]
    pub fn palette(self) -> UiPalette {
        match self.scheme {
            UiColorScheme::Light => UiPalette {
                window_background: Color::rgba(0.950, 0.950, 0.965, 1.0),
                sidebar_background: Color::rgba(0.900, 0.900, 0.920, 0.72),
                content_background: Color::rgba(1.0, 1.0, 1.0, 1.0),
                group_background: Color::rgba(1.0, 1.0, 1.0, 0.94),
                group_border: Color::rgba(0.0, 0.0, 0.0, 0.10),
                separator: Color::rgba(0.0, 0.0, 0.0, 0.10),
                text_primary: Color::rgba(0.08, 0.08, 0.09, 1.0),
                text_secondary: Color::rgba(0.32, 0.32, 0.35, 1.0),
                text_tertiary: Color::rgba(0.0, 0.0, 0.0, 0.26),
                accent: Color::rgba(0.04, 0.42, 0.95, 1.0),
                selection: Color::rgba(0.12, 0.46, 0.95, 0.18),
                sidebar_selection: Color::rgba(0.0, 0.0, 0.0, 0.085),
                hover: Color::rgba(0.0, 0.0, 0.0, 0.055),
                control_track_off: Color::rgba(0.0, 0.0, 0.0, 0.10),
                shadow: Color::rgba(0.0, 0.0, 0.0, 0.16),
            },
            UiColorScheme::Dark => UiPalette {
                window_background: Color::rgba(0.105, 0.105, 0.115, 1.0),
                sidebar_background: Color::rgba(0.145, 0.145, 0.155, 0.72),
                content_background: Color::rgba(0.105, 0.105, 0.115, 1.0),
                group_background: Color::rgba(0.175, 0.175, 0.190, 0.96),
                group_border: Color::rgba(1.0, 1.0, 1.0, 0.085),
                separator: Color::rgba(1.0, 1.0, 1.0, 0.085),
                text_primary: Color::rgba(0.94, 0.94, 0.96, 1.0),
                text_secondary: Color::rgba(0.66, 0.66, 0.69, 1.0),
                text_tertiary: Color::rgba(1.0, 1.0, 1.0, 0.25),
                accent: Color::rgba(0.24, 0.55, 1.0, 1.0),
                selection: Color::rgba(0.20, 0.48, 0.95, 0.30),
                sidebar_selection: Color::rgba(1.0, 1.0, 1.0, 0.12),
                hover: Color::rgba(1.0, 1.0, 1.0, 0.065),
                control_track_off: Color::rgba(1.0, 1.0, 1.0, 0.17),
                shadow: Color::rgba(0.0, 0.0, 0.0, 0.34),
            },
        }
    }

    /// Builds a scheme-aware material for a selective glass surface.
    #[must_use]
    pub fn glass_material(self, role: GlassRole) -> GlassMaterial {
        let (blur_radius, tint, whiteness) = match (self.scheme, role) {
            (UiColorScheme::Light, GlassRole::Sidebar) => {
                (48.0, Color::rgba(1.0, 1.0, 1.0, 0.22), 0.68)
            }
            (UiColorScheme::Light, GlassRole::Toolbar) => {
                (8.0, Color::rgba(1.0, 1.0, 1.0, 0.04), 0.04)
            }
            (UiColorScheme::Light, GlassRole::InputField) => {
                (7.0, Color::rgba(1.0, 1.0, 1.0, 0.06), 0.14)
            }
            (UiColorScheme::Light, GlassRole::SearchField) => {
                (5.0, Color::rgba(1.0, 1.0, 1.0, 0.12), 0.24)
            }
            (UiColorScheme::Light, GlassRole::FloatingControl) => {
                (2.0, Color::rgba(1.0, 1.0, 1.0, 0.06), 0.06)
            }
            (UiColorScheme::Dark, GlassRole::Sidebar) => {
                (32.0, Color::rgba(0.12, 0.16, 0.25, 0.14), 0.36)
            }
            (UiColorScheme::Dark, GlassRole::Toolbar) => {
                (8.0, Color::rgba(0.10, 0.13, 0.20, 0.08), 0.025)
            }
            (UiColorScheme::Dark, GlassRole::InputField) => {
                (7.0, Color::rgba(0.14, 0.18, 0.27, 0.09), 0.10)
            }
            (UiColorScheme::Dark, GlassRole::SearchField) => {
                (5.0, Color::rgba(0.14, 0.18, 0.27, 0.15), 0.18)
            }
            (UiColorScheme::Dark, GlassRole::FloatingControl) => {
                (2.0, Color::rgba(0.17, 0.22, 0.34, 0.10), 0.05)
            }
        };

        let mut material = GlassMaterial::clear();
        material.variant = GlassVariant::Regular;
        material.blur.radius = blur_radius;
        material.tint = tint;
        material.whiteness = whiteness;
        material.refraction.thickness = 0.20;
        material.refraction.index = 1.40;
        material.refraction.strength = 0.70;
        material.dispersion.strength = 0.07;
        material.fresnel.range = 0.75;
        material.fresnel.hardness = 0.20;
        material.fresnel.strength = 0.20;
        if role == GlassRole::SearchField {
            material.refraction.strength = 0.82;
            material.fresnel.range = 0.60;
            material.fresnel.hardness = 0.20;
            material.fresnel.strength = 0.50;
            material.glare = GlareStyle {
                range: 22.0,
                hardness: 0.14,
                convergence: 0.42,
                opposite_factor: 0.84,
                factor: 0.64,
            };
        }
        if role == GlassRole::FloatingControl {
            material.fresnel.range = 0.56;
            material.fresnel.hardness = 0.22;
            material.fresnel.strength = 0.46;
            material.glare = GlareStyle {
                range: 20.0,
                hardness: 0.16,
                convergence: 0.46,
                opposite_factor: 0.84,
                factor: 0.59,
            };
        }
        if role == GlassRole::Sidebar {
            material.refraction.strength = 0.0;
            material.dispersion.strength = 0.0;
            material.fresnel.strength = 0.0;
        }
        material.opacity = match role {
            GlassRole::Sidebar => match self.scheme {
                UiColorScheme::Light => 0.88,
                UiColorScheme::Dark => 0.76,
            },
            GlassRole::Toolbar => 0.54,
            GlassRole::InputField => 0.64,
            GlassRole::SearchField => 0.72,
            GlassRole::FloatingControl => 0.72,
        };
        material.shadow = match role {
            GlassRole::FloatingControl => ShadowStyle::elevated(),
            GlassRole::SearchField => ShadowStyle::control(),
            GlassRole::Sidebar | GlassRole::Toolbar | GlassRole::InputField => {
                ShadowStyle::subtle()
            }
        };
        material
    }

    /// Returns the default geometry associated with a semantic glass role.
    #[must_use]
    pub const fn glass_shape(self, role: GlassRole) -> GlassShape {
        match role {
            GlassRole::Sidebar | GlassRole::Toolbar => GlassShape::RoundedRect { radius: 0.0 },
            GlassRole::InputField | GlassRole::SearchField | GlassRole::FloatingControl => {
                GlassShape::Capsule
            }
        }
    }

    /// Builds the chrome border, text, state overlay, and shadow colors.
    #[must_use]
    pub fn glass_chrome(self, role: GlassRole) -> GlassChrome {
        let (
            border,
            hover_border,
            divider,
            shadow,
            text,
            disabled_text,
            hover_overlay,
            pressed_overlay,
        ) = match self.scheme {
            UiColorScheme::Light => (
                Color::rgba(0.0, 0.0, 0.0, 0.10),
                Color::rgba(0.0, 0.0, 0.0, 0.20),
                Color::rgba(0.0, 0.0, 0.0, 0.14),
                Color::rgba(0.0, 0.0, 0.0, 0.14),
                Color::rgba(0.08, 0.08, 0.09, 1.0),
                Color::rgba(0.08, 0.08, 0.09, 0.34),
                Color::rgba(0.0, 0.0, 0.0, 0.04),
                Color::rgba(0.0, 0.0, 0.0, 0.09),
            ),
            UiColorScheme::Dark => (
                Color::rgba(1.0, 1.0, 1.0, 0.16),
                Color::rgba(1.0, 1.0, 1.0, 0.30),
                Color::rgba(1.0, 1.0, 1.0, 0.18),
                Color::rgba(0.0, 0.0, 0.0, 0.30),
                Color::rgba(0.95, 0.95, 0.97, 1.0),
                Color::rgba(0.95, 0.95, 0.97, 0.34),
                Color::rgba(1.0, 1.0, 1.0, 0.055),
                Color::rgba(1.0, 1.0, 1.0, 0.11),
            ),
        };
        let (shadow_offset_y, shadow_blur) = match role {
            GlassRole::Sidebar => (1.0, 14.0),
            GlassRole::Toolbar => (2.0, 10.0),
            GlassRole::InputField => (3.0, 9.0),
            GlassRole::SearchField => (3.0, 12.0),
            GlassRole::FloatingControl => (4.0, 10.0),
        };
        GlassChrome {
            border,
            hover_border,
            divider,
            shadow,
            text,
            disabled_text,
            hover_overlay,
            pressed_overlay,
            shadow_offset_y,
            shadow_blur,
        }
    }

    /// Returns chrome for a widget layered over a real shader surface.
    #[must_use]
    pub fn compositor_chrome(self, role: GlassRole) -> GlassChrome {
        let mut chrome = self.glass_chrome(role);
        chrome.border = Color::transparent();
        chrome.hover_border = Color::transparent();
        chrome.shadow = Color::transparent();
        if role == GlassRole::Toolbar {
            chrome.hover_overlay = Color::transparent();
            chrome.pressed_overlay = Color::transparent();
        }
        chrome
    }
}

impl Default for GlassChrome {
    fn default() -> Self {
        UiTheme::dark().glass_chrome(GlassRole::FloatingControl)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_and_dark_palettes_have_opposite_tones() {
        let light = UiTheme::light().palette();
        let dark = UiTheme::dark().palette();

        assert!(light.content_background.r > dark.content_background.r);
        assert!(light.text_primary.r < dark.text_primary.r);
    }

    #[test]
    fn glass_roles_use_different_blur_radii() {
        let theme = UiTheme::dark();
        let input = theme.glass_material(GlassRole::InputField);
        let button = theme.glass_material(GlassRole::FloatingControl);

        assert!(theme.glass_material(GlassRole::Toolbar).blur.radius > button.blur.radius);
        assert!(input.blur.radius > button.blur.radius);
        assert!(input.whiteness > button.whiteness);
    }

    #[test]
    fn corner_styles_share_the_apple_squircle_smoothing() {
        let style = UiCornerStyle::CONTROL.with_radius(12.0);
        let params = style.params(120.0, 40.0);

        assert!((params.radius() - 12.0).abs() < f32::EPSILON);
        assert!((params.smoothing - APPLE_CORNER_SMOOTHING).abs() < f32::EPSILON);
    }
}
