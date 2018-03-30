// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::FromStr;

use svgparser::{
    self,
    StreamError,
};

use svgparser::xmlparser::{
    StrSpan,
    FromSpan,
};

use {
    ParseFromSpan
};

use super::{
    Path,
    Segment,
    SegmentData,
};

impl_from_str!(Path);

impl ParseFromSpan for Path {
    // TODO: can't fail
    fn from_span(span: StrSpan) -> Result<Path, Self::Err> {
        use svgparser::path::Token;

        let tokens = svgparser::path::Tokenizer::from_span(span);
        let mut p = Path::new();

        for seg in tokens {
            let seg = match seg {
                Token::MoveTo { abs, x, y } => {
                    Segment {
                        absolute: abs,
                        data: SegmentData::MoveTo { x, y },
                    }
                }
                Token::LineTo { abs, x, y } => {
                    Segment {
                        absolute: abs,
                        data: SegmentData::LineTo { x, y },
                    }
                }
                Token::HorizontalLineTo { abs, x } => {
                    Segment {
                        absolute: abs,
                        data: SegmentData::HorizontalLineTo { x },
                    }
                }
                Token::VerticalLineTo { abs, y } => {
                    Segment {
                        absolute: abs,
                        data: SegmentData::VerticalLineTo { y },
                    }
                }
                Token::CurveTo { abs, x1, y1, x2, y2, x, y } => {
                    Segment {
                        absolute: abs,
                        data: SegmentData::CurveTo { x1, y1, x2, y2, x, y },
                    }
                }
                Token::SmoothCurveTo { abs, x2, y2, x, y } => {
                    Segment {
                        absolute: abs,
                        data: SegmentData::SmoothCurveTo { x2, y2, x, y },
                    }
                }
                Token::Quadratic { abs, x1, y1, x, y } => {
                    Segment {
                        absolute: abs,
                        data: SegmentData::Quadratic { x1, y1, x, y },
                    }
                }
                Token::SmoothQuadratic { abs, x, y } => {
                    Segment {
                        absolute: abs,
                        data: SegmentData::SmoothQuadratic { x, y },
                    }
                }
                Token::EllipticalArc { abs, rx, ry, x_axis_rotation, large_arc, sweep, x, y } => {
                    Segment {
                        absolute: abs,
                        data: SegmentData::EllipticalArc {
                            rx, ry, x_axis_rotation, large_arc, sweep, x, y
                        },
                    }
                }
                Token::ClosePath { abs } => {
                    Segment {
                        absolute: abs,
                        data: SegmentData::ClosePath,
                    }
                }
            };

            p.push(seg);
        }

        Ok(p)
    }
}
