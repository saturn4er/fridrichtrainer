//! Sticker geometry for case diagrams. Polygons carry a CSS class per colour
//! so the page stylesheet decides the palette.

use crate::{
    algs::Set,
    cube::{Cube, Face, Sticker, V3},
};

pub struct Diagram {
    pub view_box: &'static str,
    pub polys: Vec<Poly>,
    pub arrows: Vec<Arrow>,
}

impl Diagram {
    /// Identifies the picture: equal signatures draw the same thing.
    pub fn signature(&self) -> String {
        let mut sig: String = self.polys.iter().map(|p| p.class).collect();
        for a in &self.arrows {
            sig.push_str(&format!("|{:.0},{:.0},{:.0},{:.0},{}", a.x1, a.y1, a.x2, a.y2, a.heads.len()));
        }
        sig
    }

    /// The stickers as a self-contained SVG document with the palette inlined,
    /// for places the page stylesheet does not reach, like the tab icon.
    pub fn to_svg(&self) -> String {
        let mut svg = format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{}">"#, self.view_box);
        for p in &self.polys {
            let fill = match p.class {
                "st st-u" => "#ffd500",
                "st st-d" => "#ffffff",
                "st st-f" => "#009b48",
                "st st-b" => "#0046ad",
                "st st-r" => "#ff5800",
                "st st-l" => "#c41e3a",
                "st st-x" => "#9a9fa8",
                _ => "#1b1d22",
            };
            svg.push_str(&format!(r##"<polygon points="{}" fill="{fill}" stroke="#1b1d22" stroke-width="1.2"/>"##, p.points));
        }
        svg.push_str("</svg>");
        svg
    }
}

/// A line from one top-layer piece to the slot it is moved into, with an
/// arrowhead at the destination, or at both ends for a swap.
pub struct Arrow {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub heads: Vec<String>,
}

pub struct Poly {
    pub points: String,
    pub class: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Style {
    /// Top layer seen from above, only U stickers coloured.
    Orientation,
    /// Top layer seen from above in full colour.
    Permutation,
    /// Three visible faces, last-layer pieces greyed out.
    Pair,
    /// Three visible faces in full colour.
    Full,
}

impl Style {
    pub fn for_set(set: Set) -> Style {
        match set {
            Set::F2l => Style::Pair,
            Set::Oll => Style::Orientation,
            Set::Pll => Style::Permutation,
        }
    }
}

fn class(face: Face) -> &'static str {
    match face {
        Face::U => "st st-u",
        Face::D => "st st-d",
        Face::F => "st st-f",
        Face::B => "st st-b",
        Face::R => "st st-r",
        Face::L => "st st-l",
    }
}

const GREY: &str = "st st-x";
const FRAME: &str = "frame";

fn sticker_class(style: Style, s: &Sticker) -> &'static str {
    match style {
        Style::Orientation if s.color() != Face::U => GREY,
        Style::Pair if s.home_pos[1] == 1 => GREY,
        _ => class(s.color()),
    }
}

fn points(pts: &[(f64, f64)]) -> String {
    pts.iter()
        .map(|(x, y)| format!("{x:.2},{y:.2}"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn diagram(cube: &Cube, style: Style) -> Diagram {
    match style {
        Style::Orientation | Style::Permutation => plan(cube, style),
        Style::Pair | Style::Full => iso(cube, style),
    }
}

/// U face as a 3×3 grid with the top-layer side stickers as thin strips.
fn plan(cube: &Cube, style: Style) -> Diagram {
    const CELL: f64 = 30.0;
    const STRIP: f64 = 9.0;
    const GAP: f64 = 3.0;
    const PAD: f64 = 2.0;
    let origin = PAD + STRIP + GAP;
    let mut polys = vec![Poly {
        points: points(&rect(origin, origin, 3.0 * CELL, 3.0 * CELL)),
        class: FRAME,
    }];
    let mut push = |pos: V3, normal: V3, r: [(f64, f64); 4]| {
        polys.push(Poly {
            points: points(&r),
            class: sticker_class(style, cube.sticker_at(pos, normal)),
        });
    };
    let inset = 1.5;
    for z in -1..=1i8 {
        for x in -1..=1i8 {
            let left = origin + f64::from(x + 1) * CELL + inset;
            let top = origin + f64::from(z + 1) * CELL + inset;
            push([x, 1, z], [0, 1, 0], rect(left, top, CELL - 2.0 * inset, CELL - 2.0 * inset));
        }
    }
    let far = origin + 3.0 * CELL + GAP;
    for i in -1..=1i8 {
        let along = origin + f64::from(i + 1) * CELL + inset;
        let len = CELL - 2.0 * inset;
        push([i, 1, -1], [0, 0, -1], rect(along, PAD, len, STRIP));
        push([i, 1, 1], [0, 0, 1], rect(along, far, len, STRIP));
        push([-1, 1, i], [-1, 0, 0], rect(PAD, along, STRIP, len));
        push([1, 1, i], [1, 0, 0], rect(far, along, STRIP, len));
    }
    let center = |x: i8, z: i8| {
        (origin + (f64::from(x) + 1.5) * CELL, origin + (f64::from(z) + 1.5) * CELL)
    };
    let arrows = if style == Style::Permutation { arrows(cube, center) } else { Vec::new() };
    Diagram { view_box: "0 0 118 118", polys, arrows }
}

/// A top-layer slot on the x/z grid.
type Slot = (i8, i8);

/// Moves of the top-layer pieces, as (current slot, destination, swap).
/// A swap is listed once.
fn piece_moves(cube: &Cube) -> Vec<(Slot, Slot, bool)> {
    let mut moves: Vec<(Slot, Slot)> = Vec::new();
    for s in &cube.stickers {
        let home = (s.home_pos[0], s.home_pos[2]);
        let at = (s.pos[0], s.pos[2]);
        if s.home_pos[1] == 1 && home != (0, 0) && at != home && !moves.contains(&(at, home)) {
            moves.push((at, home));
        }
    }
    let mut result = Vec::new();
    for &(from, to) in &moves {
        let swap = moves.contains(&(to, from));
        if swap && from > to {
            continue;
        }
        result.push((from, to, swap));
    }
    result
}

fn arrows(cube: &Cube, center: impl Fn(i8, i8) -> (f64, f64)) -> Vec<Arrow> {
    const TRIM: f64 = 7.0;
    const HEAD_LEN: f64 = 7.5;
    const HEAD_HALF: f64 = 4.0;
    piece_moves(cube)
        .into_iter()
        .map(|(from, to, both)| {
            let (ax, ay) = center(from.0, from.1);
            let (bx, by) = center(to.0, to.1);
            let len = ((bx - ax).powi(2) + (by - ay).powi(2)).sqrt();
            let (ux, uy) = ((bx - ax) / len, (by - ay) / len);
            let (x1, y1) = (ax + ux * TRIM, ay + uy * TRIM);
            let (x2, y2) = (bx - ux * TRIM, by - uy * TRIM);
            let head = |tx: f64, ty: f64, dx: f64, dy: f64| {
                let (bx, by) = (tx - dx * HEAD_LEN, ty - dy * HEAD_LEN);
                points(&[
                    (tx, ty),
                    (bx - dy * HEAD_HALF, by + dx * HEAD_HALF),
                    (bx + dy * HEAD_HALF, by - dx * HEAD_HALF),
                ])
            };
            let mut heads = vec![head(x2, y2, ux, uy)];
            if both {
                heads.push(head(x1, y1, -ux, -uy));
            }
            let shaft = HEAD_LEN * 0.8;
            Arrow {
                x1: if both { x1 + ux * shaft } else { x1 },
                y1: if both { y1 + uy * shaft } else { y1 },
                x2: x2 - ux * shaft,
                y2: y2 - uy * shaft,
                heads,
            }
        })
        .collect()
}

fn rect(x: f64, y: f64, w: f64, h: f64) -> [(f64, f64); 4] {
    [(x, y), (x + w, y), (x + w, y + h), (x, y + h)]
}

/// Isometric view of the U, F and R faces.
fn iso(cube: &Cube, style: Style) -> Diagram {
    const SCALE: f64 = 20.0;
    const CX: f64 = 64.0;
    const CY: f64 = 62.0;
    let project = |p: [f64; 3]| -> (f64, f64) {
        let [x, y, z] = p;
        (CX + (x - z) * 0.866 * SCALE, CY + ((x + z) * 0.5 - y) * SCALE)
    };
    let quad = |center: [f64; 3], t1: [f64; 3], t2: [f64; 3], half: f64| -> [(f64, f64); 4] {
        let at = |a: f64, b: f64| {
            project([
                center[0] + (a * t1[0] + b * t2[0]) * half,
                center[1] + (a * t1[1] + b * t2[1]) * half,
                center[2] + (a * t1[2] + b * t2[2]) * half,
            ])
        };
        [at(-1.0, -1.0), at(1.0, -1.0), at(1.0, 1.0), at(-1.0, 1.0)]
    };
    let faces = [Face::U, Face::F, Face::R];
    let mut polys = Vec::new();
    for face in faces {
        let n = face.normal().map(f64::from);
        let (t1, t2) = tangents(face);
        polys.push(Poly {
            points: points(&quad(n.map(|c| c * 1.5), t1, t2, 1.5)),
            class: FRAME,
        });
    }
    for face in faces {
        let n = face.normal();
        let (t1, t2) = tangents(face);
        for s in cube.stickers.iter().filter(|s| s.normal == n) {
            let center = [0, 1, 2].map(|i| f64::from(s.pos[i]) + f64::from(n[i]) * 0.5);
            polys.push(Poly {
                points: points(&quad(center, t1, t2, 0.44)),
                class: sticker_class(style, s),
            });
        }
    }
    Diagram { view_box: "0 0 128 128", polys, arrows: Vec::new() }
}

fn tangents(face: Face) -> ([f64; 3], [f64; 3]) {
    match face {
        Face::U | Face::D => ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        Face::F | Face::B => ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        Face::R | Face::L => ([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algs::find;

    fn moves_of(id: &str) -> Vec<(Slot, Slot, bool)> {
        piece_moves(&Cube::case_of(find(id).unwrap().alg).unwrap())
    }

    #[test]
    fn pll_algorithms_need_no_final_u_turn() {
        for case in crate::algs::of_set(Set::Pll) {
            let moved = |auf: &str| piece_moves(&Cube::case_of(&format!("{}{auf}", case.alg)).unwrap()).len();
            let best = [" U", " U2", " U'"].into_iter().map(moved).min().unwrap();
            assert!(moved("") <= best, "{} moves fewer pieces with a final U turn", case.id);
        }
    }

    #[test]
    fn svg_inlines_every_sticker_colour() {
        let d = diagram(&Cube::case_of(find("pll-t").unwrap().alg).unwrap(), Style::Permutation);
        let svg = d.to_svg();
        assert_eq!(svg.matches("<polygon").count(), d.polys.len());
        assert!(!svg.contains("class="));
        assert!(svg.contains(r##"fill="#ffd500""##));
    }

    #[test]
    fn t_perm_swaps_two_corners_and_two_edges() {
        let moves = moves_of("pll-t");
        assert_eq!(moves.len(), 2);
        assert!(moves.iter().all(|m| m.2));
    }

    #[test]
    fn u_perm_cycles_three_edges() {
        let moves = moves_of("pll-ua");
        assert_eq!(moves.len(), 3);
        assert!(moves.iter().all(|m| !m.2));
    }

    #[test]
    fn oll_has_no_arrows() {
        let cube = Cube::case_of(find("oll-27").unwrap().alg).unwrap();
        assert!(diagram(&cube, Style::Orientation).arrows.is_empty());
    }
}

