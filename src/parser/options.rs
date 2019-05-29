/// Options that defines SVG parsing.
#[derive(Debug)]
pub struct ParseOptions {
    /// Skip unresolved references inside the `class` attribute.
    ///
    /// It's enabled by default, but if you disable it - all unresolved classes will be kept
    /// in the `class` attribute.
    ///
    /// Default: `true`
    pub skip_unresolved_classes: bool,

    /// Skip attributes with invalid values.
    ///
    /// By default, attribute with an invalid value will lead to a parsing error.
    /// This flag allows converting this error into a warning.
    ///
    /// Default: `false`
    pub skip_invalid_attributes: bool,

    /// Skip invalid/unsupported CSS.
    ///
    /// By default, CSS with an invalid/unsupported value will lead to a parsing error.
    /// This flag allows converting this error into a warning.
    ///
    /// Default: `false`
    pub skip_invalid_css: bool,
}

impl Default for ParseOptions {
    fn default() -> ParseOptions {
        ParseOptions {
            skip_unresolved_classes: true,
            skip_invalid_attributes: false,
            skip_invalid_css: false,
        }
    }
}
