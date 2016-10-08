// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Options that defines SVG paths writing.
pub struct WriteOptionsPaths {
    /// Use compact path notation.
    ///
    /// SVG allow us to remove some symbols from path notation without breaking parsing.
    ///
    /// Example:
    ///
    /// `M 10 -20 A 5.5 0.3 -4 1 1 0 -0.1` -> `M10-20A5.5.3-4 1 1 0-.1`
    ///
    /// Default: disabled
    pub use_compact_notation: bool,

    /// Join ArcTo flags.
    ///
    /// Elliptical arc curve segment has flags parameters, which can have values of `0` or `1`.
    /// Since we have fixed-width values, we can skip spaces between them.
    ///
    /// Example:
    ///
    /// `A 5 5 30 1 1 10 10` -> `A 5 5 30 1110 10`
    ///
    /// Default: disabled
    ///
    /// **Note:** Sadly, but most of the viewers doesn't support such notation,
    /// even throw it's valid by SVG spec.
    pub join_arc_to_flags: bool,

    /// Remove duplicated commands.
    ///
    /// If the segment has the same type as previous - we can skip command specifier.
    ///
    /// Example:
    ///
    /// `M 10 10 L 20 20 L 30 30 L 40 40` -> `M 10 10 L 20 20 30 30 40 40`
    ///
    /// Default: disabled
    pub remove_duplicated_commands: bool,
}

/// Options that defines SVG writing.
pub struct WriteOptions {
    /// Set XML nodes indention.
    ///
    /// Range: -1..4 (-1 indicates no spaces and new lines)
    ///
    /// Example:
    ///
    /// Before:
    ///
    /// ```text
    ///     <svg>
    ///         <rect fill="red"/>
    ///     </svg>
    ///
    /// ```
    ///
    /// After:
    ///
    /// ```text
    ///     <svg><rect fill="red"/></svg>
    /// ```
    ///
    /// Default: 4
    pub indent: i8,

    /// Use single quote marks instead of double quote.
    ///
    /// Example:
    ///
    /// ```text
    /// <rect fill="red"/>
    /// <rect fill='red'/>
    /// ```
    ///
    /// Default: disabled
    pub use_single_quote: bool,

    /// Use #RGB color notation when possible.
    ///
    /// By default all colors written using #RRGGBB notation.
    ///
    /// Example:
    ///
    /// `#ff0000` -> `#f00`, `#000000` -> `#000`, `#00aa00` -> `#0a0`
    ///
    /// Default: disabled
    pub trim_hex_colors: bool,

    /// Write hidden attributes.
    ///
    /// `libsvgdom` support invisible attributes, which can be dumped to output using this option.
    ///
    /// See `svgdom::Attribute` documentation.
    ///
    /// Default: disabled
    pub write_hidden_attributes: bool,

    /// Remove leading zero from numbers.
    ///
    /// Example:
    ///
    /// `0.1` -> `.1`, `-0.1` -> `-.1`
    ///
    /// Default: disabled
    pub remove_leading_zero: bool,

    /// Paths options.
    ///
    /// See `WriteOptionsPaths` documentation.
    pub paths: WriteOptionsPaths,

    /// Simplify transform matrices into short equivalent when possible.
    ///
    /// If not set - all transform will be saved as 'matrix'.
    ///
    /// Examples:
    ///
    /// ```text
    /// matrix(1 0 0 1 10 20) -> translate(10 20)
    /// matrix(1 0 0 1 10 0)  -> translate(10)
    /// matrix(2 0 0 3 0 0)   -> scale(2 3)
    /// matrix(2 0 0 2 0 0)   -> scale(2)
    /// matrix(0 1 -1 0 0 0)  -> rotate(-90)
    /// ```
    ///
    /// Default: disabled
    pub simplify_transform_matrices: bool,
}

impl Default for WriteOptions {
    fn default() -> WriteOptions {
        WriteOptions {
            indent: 4,
            use_single_quote: false,
            trim_hex_colors: false,
            write_hidden_attributes: false,
            remove_leading_zero: false,
            paths: WriteOptionsPaths {
                use_compact_notation: false,
                join_arc_to_flags: false,
                remove_duplicated_commands: false,
            },
            simplify_transform_matrices: false,
        }
    }
}
