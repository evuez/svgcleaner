/****************************************************************************
**
** svgcleaner could help you to clean up your SVG files
** from unnecessary data.
** Copyright (C) 2012-2017 Evgeniy Reizner
**
** This program is free software; you can redistribute it and/or modify
** it under the terms of the GNU General Public License as published by
** the Free Software Foundation; either version 2 of the License, or
** (at your option) any later version.
**
** This program is distributed in the hope that it will be useful,
** but WITHOUT ANY WARRANTY; without even the implied warranty of
** MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
** GNU General Public License for more details.
**
** You should have received a copy of the GNU General Public License along
** with this program; if not, write to the Free Software Foundation, Inc.,
** 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
**
****************************************************************************/

use super::short::AId;

use svgdom::{Document, AttributeValue};
use svgdom::types::path;

use options::Options;

pub fn round_numbers(doc: &Document, options: &Options) {
    let coord_precision = options.coordinates_precision as usize;
    let prop_precision = options.properties_precision as usize;
    let paths_precision = options.paths_coordinates_precision as usize;
    let ts_precision = options.transforms_precision as usize;

    for node in doc.descendants().svg() {
        let mut attrs = node.attributes_mut();

        for (aid, ref mut attr) in attrs.iter_svg_mut() {
            match aid {
                AId::X  | AId::Y |
                AId::Dx | AId::Dy |
                AId::X1 | AId::Y1 |
                AId::X2 | AId::Y2 |
                AId::R  |
                AId::Rx | AId::Ry |
                AId::Cx | AId::Cy |
                AId::Fx | AId::Fy |
                AId::Width | AId::Height |
                AId::StrokeDasharray => {
                    match attr.value {
                        AttributeValue::Length(ref mut v) => {
                            round_number(&mut v.num, coord_precision);
                        }
                        AttributeValue::LengthList(ref mut list) => {
                            for n in list.iter_mut() {
                                round_number(&mut n.num, coord_precision);
                            }
                        }
                        _ => {}
                    }
                }

                AId::StrokeDashoffset |
                AId::StrokeMiterlimit |
                AId::StrokeWidth |
                AId::Opacity |
                AId::FillOpacity |
                AId::FloodOpacity |
                AId::StrokeOpacity |
                AId::StopOpacity |
                AId::FontSize => {
                    match attr.value {
                        AttributeValue::Number(ref mut num) => {
                            round_number(num, prop_precision);
                        }
                        AttributeValue::Length(ref mut v) => {
                            round_number(&mut v.num, prop_precision);
                        }
                        _ => {}
                    }
                }

                AId::Transform |
                AId::GradientTransform |
                AId::PatternTransform => {
                    match attr.value {
                        AttributeValue::Transform(ref mut ts) => {
                            round_number(&mut ts.a, ts_precision);
                            round_number(&mut ts.b, ts_precision);
                            round_number(&mut ts.c, ts_precision);
                            round_number(&mut ts.d, ts_precision);
                            round_number(&mut ts.e, coord_precision);
                            round_number(&mut ts.f, coord_precision);
                        }
                        _ => {}
                    }
                }

                AId::D => {
                    match attr.value {
                        AttributeValue::Path(ref mut p) => {
                            round_path(p, paths_precision);
                        }
                        _ => {}
                    }
                }

                AId::ViewBox |
                AId::Points => {
                    match attr.value {
                        AttributeValue::NumberList(ref mut list) => {
                            for n in list.iter_mut() {
                                round_number(n, paths_precision);
                            }
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        }
    }
}

static POW_VEC: &'static [f64] = &[
                    0.0,
                   10.0,
                  100.0,
                1_000.0,
               10_000.0,
              100_000.0,
            1_000_000.0,
           10_000_000.0,
          100_000_000.0,
        1_000_000_000.0,
       10_000_000_000.0,
      100_000_000_000.0,
    1_000_000_000_000.0,
];

fn round_number(n: &mut f64, precision: usize) {
    *n = (*n * POW_VEC[precision]).round() / POW_VEC[precision];
}

fn round_path(path: &mut path::Path, precision: usize) {
    use svgdom::types::path::SegmentData;

    for seg in &mut path.d {
        match *seg.data_mut() {
              SegmentData::MoveTo { ref mut x, ref mut y }
            | SegmentData::LineTo { ref mut x, ref mut y }
            | SegmentData::SmoothQuadratic { ref mut x, ref mut y } => {
                round_number(x, precision);
                round_number(y, precision);
            }

            SegmentData::HorizontalLineTo { ref mut x } => {
                round_number(x, precision);
            }

            SegmentData::VerticalLineTo { ref mut y } => {
                round_number(y, precision);
            }

            SegmentData::CurveTo { ref mut x1, ref mut y1, ref mut x2, ref mut y2,
                                   ref mut x, ref mut y } => {
                round_number(x1, precision);
                round_number(y1, precision);
                round_number(x2, precision);
                round_number(y2, precision);
                round_number(x, precision);
                round_number(y, precision);
            }

            SegmentData::SmoothCurveTo { ref mut x2, ref mut y2, ref mut x, ref mut y } => {
                round_number(x2, precision);
                round_number(y2, precision);
                round_number(x, precision);
                round_number(y, precision);
            }

            SegmentData::Quadratic { ref mut x1, ref mut y1, ref mut x, ref mut y } => {
                round_number(x1, precision);
                round_number(y1, precision);
                round_number(x, precision);
                round_number(y, precision);
            }

            SegmentData::EllipticalArc { ref mut rx, ref mut ry, ref mut x_axis_rotation,
                                         ref mut x, ref mut y, .. } => {
                round_number(rx, precision);
                round_number(ry, precision);
                round_number(x_axis_rotation, precision);
                round_number(x, precision);
                round_number(y, precision);
            }

            SegmentData::ClosePath => {}
        }
    }
}
