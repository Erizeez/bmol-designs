//! Typography metrics and font family names for modern Apple-style UIs.

/// Interface text sizes following macOS settings typography.
pub mod size {
    /// Large in-page section titles ("General").
    pub const TITLE: f32 = 20.0;
    /// Prominent labels such as the toolbar title.
    pub const HEADLINE: f32 = 15.0;
    /// Fused titlebar/toolbar title. Kept separate so platform chrome can be
    /// calibrated without changing ordinary headline text.
    pub const TOOLBAR_TITLE: f32 = 15.0;
    /// Standard control and row labels.
    pub const BODY: f32 = 13.0;
    /// Supporting detail text under a row label.
    pub const CAPTION: f32 = 11.0;
}

/// Known font family names in priority order for Apple-style interfaces.
pub const PREFERRED_FONT_FAMILIES: &[&str] = &[
    "System Font",
    "SF Pro",
    "SF Pro Display",
    "SF Pro Text",
    "PingFang SC",
    "-apple-system",
    "BlinkMacSystemFont",
    "Segoe UI",
    "sans-serif",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typography_scale_decreases() {
        const {
            assert!(size::TITLE > size::HEADLINE);
            assert!(size::HEADLINE > size::BODY);
            assert!(size::BODY > size::CAPTION);
        }
    }
}
