//! Facelet cube model. Coordinates: x points to R, y to U, z to F; cubie
//! centres lie in {-1, 0, 1}³. Every sticker remembers its solved position,
//! so the colour of a sticker is its home face.

use std::fmt;

pub type V3 = [i8; 3];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Face {
    U,
    D,
    F,
    B,
    R,
    L,
}

impl Face {
    pub fn normal(self) -> V3 {
        match self {
            Face::U => [0, 1, 0],
            Face::D => [0, -1, 0],
            Face::F => [0, 0, 1],
            Face::B => [0, 0, -1],
            Face::R => [1, 0, 0],
            Face::L => [-1, 0, 0],
        }
    }

    pub fn from_normal(n: V3) -> Face {
        match n {
            [0, 1, 0] => Face::U,
            [0, -1, 0] => Face::D,
            [0, 0, 1] => Face::F,
            [0, 0, -1] => Face::B,
            [1, 0, 0] => Face::R,
            [-1, 0, 0] => Face::L,
            _ => unreachable!("not a unit normal: {n:?}"),
        }
    }

    pub const ALL: [Face; 6] = [Face::U, Face::D, Face::F, Face::B, Face::R, Face::L];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sticker {
    pub pos: V3,
    pub normal: V3,
    pub home_pos: V3,
    pub home_normal: V3,
}

impl Sticker {
    pub fn color(&self) -> Face {
        Face::from_normal(self.home_normal)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    /// Rotation axis: 0 = x, 1 = y, 2 = z.
    axis: usize,
    /// Layers turned, as coordinates along the axis.
    layers: [bool; 3],
    /// Quarter turns counter-clockwise around the positive axis.
    quarters: u8,
}

impl Move {
    fn affects(&self, pos: V3) -> bool {
        self.layers[(pos[self.axis] + 1) as usize]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError(pub String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown move `{}`", self.0)
    }
}

impl std::error::Error for ParseError {}

/// Parses WCA-style notation: face turns, wide turns (`r`, `Rw`), slices
/// (`M`, `E`, `S`) and rotations (`x`, `y`, `z`). Brackets are ignored.
pub fn parse(alg: &str) -> Result<Vec<Move>, ParseError> {
    let cleaned: String = alg
        .chars()
        .map(|c| match c {
            '(' | ')' | '[' | ']' => ' ',
            '’' | '`' | '´' => '\'',
            c => c,
        })
        .collect();
    cleaned.split_whitespace().map(parse_token).collect()
}

fn parse_token(token: &str) -> Result<Move, ParseError> {
    let err = || ParseError(token.to_owned());
    let mut chars = token.chars().peekable();
    let base = chars.next().ok_or_else(err)?;
    let wide_suffix = chars.peek() == Some(&'w');
    if wide_suffix {
        chars.next();
    }
    let rest: String = chars.collect();
    let amount: u8 = match rest.as_str() {
        "" => 1,
        "'" => 3,
        "2" | "2'" => 2,
        _ => return Err(err()),
    };

    let outer = |sign: i8| -> [bool; 3] {
        let mut l = [false; 3];
        l[(sign + 1) as usize] = true;
        l
    };
    let wide = |sign: i8| -> [bool; 3] {
        let mut l = outer(sign);
        l[1] = true;
        l
    };
    let all = [true; 3];
    let middle = [false, true, false];

    // (axis, layers, clockwise turn seen from the positive axis?)
    let (axis, layers, positive_side) = match (base, wide_suffix) {
        ('R', false) => (0, outer(1), true),
        ('L', false) => (0, outer(-1), false),
        ('U', false) => (1, outer(1), true),
        ('D', false) => (1, outer(-1), false),
        ('F', false) => (2, outer(1), true),
        ('B', false) => (2, outer(-1), false),
        ('r', false) | ('R', true) => (0, wide(1), true),
        ('l', false) | ('L', true) => (0, wide(-1), false),
        ('u', false) | ('U', true) => (1, wide(1), true),
        ('d', false) | ('D', true) => (1, wide(-1), false),
        ('f', false) | ('F', true) => (2, wide(1), true),
        ('b', false) | ('B', true) => (2, wide(-1), false),
        ('M', false) => (0, middle, false),
        ('E', false) => (1, middle, false),
        ('S', false) => (2, middle, true),
        ('x', false) => (0, all, true),
        ('y', false) => (1, all, true),
        ('z', false) => (2, all, true),
        _ => return Err(err()),
    };

    // A clockwise quarter seen from +axis is a clockwise (negative) rotation
    // around +axis, i.e. three counter-clockwise quarters.
    let quarters = if positive_side { (4 - amount) % 4 } else { amount };
    Ok(Move { axis, layers, quarters })
}

/// Inverts a written algorithm, keeping its notation readable.
pub fn invert(alg: &str) -> String {
    let cleaned: String = alg
        .chars()
        .map(|c| if "()[]".contains(c) { ' ' } else { c })
        .collect();
    cleaned
        .split_whitespace()
        .rev()
        .map(|t| {
            if let Some(base) = t.strip_suffix("2'") {
                format!("{base}2")
            } else if let Some(base) = t.strip_suffix('\'') {
                base.to_owned()
            } else if t.ends_with('2') {
                t.to_owned()
            } else {
                format!("{t}'")
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Rewrites `alg` for the cube held a quarter turn further around U, so the
/// same case is solved from the next side without a rotation: R becomes B,
/// B becomes L, L becomes F and F becomes R.
pub fn turn_alg(alg: &str) -> String {
    let cleaned: String = alg
        .chars()
        .map(|c| if "()[]".contains(c) { ' ' } else { c })
        .collect();
    cleaned
        .split_whitespace()
        .map(|token| {
            let mut chars = token.chars();
            let base = chars.next().unwrap_or_default();
            let rest: String = chars.collect();
            let (mapped, flip) = match base {
                'R' => ('B', false),
                'B' => ('L', false),
                'L' => ('F', false),
                'F' => ('R', false),
                'r' => ('b', false),
                'b' => ('l', false),
                'l' => ('f', false),
                'f' => ('r', false),
                'M' => ('S', false),
                'S' => ('M', true),
                'x' => ('z', true),
                'z' => ('x', false),
                other => (other, false),
            };
            let rest = if !flip || rest.starts_with('2') {
                rest
            } else if let Some(r) = rest.strip_prefix('\'') {
                r.to_owned()
            } else {
                format!("{rest}'")
            };
            format!("{mapped}{rest}")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn rotate_ccw(v: V3, axis: usize) -> V3 {
    let [x, y, z] = v;
    match axis {
        0 => [x, -z, y],
        1 => [z, y, -x],
        _ => [-y, x, z],
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cube {
    pub stickers: Vec<Sticker>,
}

impl Default for Cube {
    fn default() -> Self {
        Self::solved()
    }
}

impl Cube {
    pub fn solved() -> Self {
        let mut stickers = Vec::with_capacity(54);
        for face in Face::ALL {
            let n = face.normal();
            let axis = n.iter().position(|&c| c != 0).unwrap();
            for a in -1..=1i8 {
                for b in -1..=1i8 {
                    let mut pos = [0i8; 3];
                    pos[axis] = n[axis];
                    pos[(axis + 1) % 3] = a;
                    pos[(axis + 2) % 3] = b;
                    stickers.push(Sticker { pos, normal: n, home_pos: pos, home_normal: n });
                }
            }
        }
        Cube { stickers }
    }

    pub fn apply_move(&mut self, m: Move) {
        for s in &mut self.stickers {
            if m.affects(s.pos) {
                for _ in 0..m.quarters {
                    s.pos = rotate_ccw(s.pos, m.axis);
                    s.normal = rotate_ccw(s.normal, m.axis);
                }
            }
        }
    }

    pub fn apply(&mut self, moves: &[Move]) {
        for &m in moves {
            self.apply_move(m);
        }
    }

    pub fn apply_alg(&mut self, alg: &str) -> Result<(), ParseError> {
        self.apply(&parse(alg)?);
        Ok(())
    }

    pub fn sticker_at(&self, pos: V3, normal: V3) -> &Sticker {
        self.stickers
            .iter()
            .find(|s| s.pos == pos && s.normal == normal)
            .expect("every position holds a sticker")
    }

    fn centers_home(&self) -> bool {
        self.stickers
            .iter()
            .filter(|s| s.home_pos.iter().filter(|&&c| c != 0).count() == 1)
            .all(|s| s.pos == s.home_pos)
    }

    /// Undoes any whole-cube rotation so the centres are back in place.
    pub fn normalize_orientation(&mut self) {
        const UPS: [&str; 6] = ["", "x", "x2", "x'", "z", "z'"];
        const TURNS: [&str; 4] = ["", "y", "y2", "y'"];
        for up in UPS {
            for turn in TURNS {
                let mut c = self.clone();
                c.apply_alg(up).unwrap();
                c.apply_alg(turn).unwrap();
                if c.centers_home() {
                    *self = c;
                    return;
                }
            }
        }
        unreachable!("one of the 24 rotations restores the centres");
    }

    /// The cube that `alg` solves: a solved cube with the inverse applied.
    pub fn case_of(alg: &str) -> Result<Cube, ParseError> {
        let moves = parse(alg)?;
        let mut cube = Cube::solved();
        for m in moves.iter().rev() {
            cube.apply_move(Move { quarters: (4 - m.quarters) % 4, ..*m });
        }
        cube.normalize_orientation();
        Ok(cube)
    }

    #[cfg(test)]
    pub fn is_solved(&self) -> bool {
        self.stickers.iter().all(|s| s.pos == s.home_pos && s.normal == s.home_normal)
    }

    /// True when every sticker whose piece passes `filter` is at home.
    pub fn pieces_home(&self, filter: impl Fn(V3) -> bool) -> bool {
        self.stickers
            .iter()
            .filter(|s| filter(s.home_pos))
            .all(|s| s.pos == s.home_pos && s.normal == s.home_normal)
    }
}
