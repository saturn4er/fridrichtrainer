use std::sync::LazyLock;

use crate::cube::{Cube, Face, V3, invert, parse};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Set {
    F2l,
    Oll,
    Pll,
}

impl Set {
    pub const ALL: [Set; 3] = [Set::F2l, Set::Oll, Set::Pll];

    pub fn slug(self) -> &'static str {
        match self {
            Set::F2l => "f2l",
            Set::Oll => "oll",
            Set::Pll => "pll",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Set::F2l => "F2L",
            Set::Oll => "OLL",
            Set::Pll => "PLL",
        }
    }

    pub fn from_slug(slug: &str) -> Option<Set> {
        Set::ALL.into_iter().find(|s| s.slug() == slug)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Group {
    Basic,
    BothInTop,
    EdgeInSlot,
    CornerInSlot,
    BothInSlot,
    Dot,
    Square,
    Lightning,
    Fish,
    KnightMove,
    Cross,
    CornersOriented,
    Awkward,
    P,
    T,
    C,
    W,
    L,
    I,
    CornersOnly,
    EdgesOnly,
    AdjacentSwap,
    DiagonalSwap,
    G,
}

#[derive(Debug, PartialEq)]
pub struct Case {
    pub id: String,
    pub set: Set,
    pub name: String,
    pub group: Group,
    pub alg: &'static str,
}

impl Case {
    pub fn setup(&self) -> String {
        invert(self.alg)
    }
}

const F2L_CORNER: V3 = [1, -1, 1];
const F2L_EDGE: V3 = [1, 0, 1];

const F2L: [&str; 41] = [
    // Basic inserts
    "U R U' R'",
    "U' F' U F",
    "F' U' F",
    "R U R'",
    // Corner and edge in the top layer, edge colours split
    "U' R U R' U2 R U' R'",
    "U F' U' F U2 F' U F",
    "U' R U2 R' U2 R U' R'",
    "U F' U2 F U2 F' U F",
    "U' R U' R' U F' U' F",
    "U' R U R' U R U R'",
    "U' R U2 R' U F' U' F",
    "R U' R' U R U' R' U2 R U' R'",
    "U F' U F U' F' U' F",
    "U' R U' R' U R U R'",
    "R U' R' U2 F' U' F",
    "R U R' U2 R U R' U' R U R'",
    "R U2 R' U' R U R'",
    "F' U2 F U F' U' F",
    "U R U2 R' U R U' R'",
    "U' F' U2 F U' F' U F",
    "U2 R U R' U R U' R'",
    "U2 F' U' F U' F' U F",
    "F' U F U2 R U R'",
    "U' F' U F U F' U F U' F' U F",
    // Corner in the top layer, edge in the slot
    "U' R' F R F' R U R'",
    "U R U' R' F R' F' R",
    "R U' R' U R U' R'",
    "F' U F U' F' U F",
    "R U R' U' R U R'",
    "F' U' F U F' U' F",
    "F' U' F U R U' R'",
    "R U R' F R' F' R",
    // Corner in the slot, edge in the top layer
    "U' R U' R' U2 R U' R'",
    "U F' U F U2 F' U F",
    "R U' R' U' R U R' U2 R U' R'",
    "R U R' U' R U R' U' R U R'",
    // Corner and edge in the slot
    "R U' R' U R U2 R' U R U' R'",
    "R F U R U' R' F' U' R'",
    "R U F R U R' U' F' R'",
    "R2 U2 F R2 F' U2 R' U R'",
    "R U' R' F' U2 F",
];

const OLL: [(Group, &str); 57] = [
    (Group::Dot, "R U2 R2 F R F' U2 R' F R F'"),
    (Group::Dot, "F R U R' U' F' f R U R' U' f'"),
    (Group::Dot, "f R U R' U' f' U' F R U R' U' F'"),
    (Group::Dot, "f R U R' U' f' U F R U R' U' F'"),
    (Group::Square, "r' U2 R U R' U r"),
    (Group::Square, "r U2 R' U' R U' r'"),
    (Group::Lightning, "r U R' U R U2 r'"),
    (Group::Lightning, "r' U' R U' R' U2 r"),
    (Group::Fish, "R U R' U' R' F R2 U R' U' F'"),
    (Group::Fish, "R U R' U R' F R F' R U2 R'"),
    (Group::Lightning, "r U R' U R' F R F' R U2 r'"),
    (Group::Lightning, "M' R' U' R U' R' U2 R U' R r'"),
    (Group::KnightMove, "F U R U' R2 F' R U R U' R'"),
    (Group::KnightMove, "R' F R U R' F' R F U' F'"),
    (Group::KnightMove, "r' U' r R' U' R U r' U r"),
    (Group::KnightMove, "r U r' R U R' U' r U' r'"),
    (Group::Dot, "R U R' U R' F R F' U2 R' F R F'"),
    (Group::Dot, "r U R' U R U2 r2 U' R U' R' U2 r"),
    (Group::Dot, "M U R U R' U' M' R' F R F'"),
    (Group::Dot, "r U R' U' M2 U R U' R' U' M'"),
    (Group::Cross, "R U2 R' U' R U R' U' R U' R'"),
    (Group::Cross, "R U2 R2 U' R2 U' R2 U2 R"),
    (Group::Cross, "R2 D' R U2 R' D R U2 R"),
    (Group::Cross, "r U R' U' r' F R F'"),
    (Group::Cross, "F' r U R' U' r' F R"),
    (Group::Cross, "R U2 R' U' R U' R'"),
    (Group::Cross, "R U R' U R U2 R'"),
    (Group::CornersOriented, "r U R' U' r' R U R U' R'"),
    (Group::Awkward, "R U R' U' R U' R' F' U' F R U R'"),
    (Group::Awkward, "F R' F R2 U' R' U' R U R' F2"),
    (Group::P, "R' U' F U R U' R' F' R"),
    (Group::P, "L U F' U' L' U L F L'"),
    (Group::T, "R U R' U' R' F R F'"),
    (Group::C, "R U R2 U' R' F R U R U' F'"),
    (Group::Fish, "R U2 R2 F R F' R U2 R'"),
    (Group::W, "L' U' L U' L' U L U L F' L' F"),
    (Group::Fish, "F R' F' R U R U' R'"),
    (Group::W, "R U R' U R U' R' U' R' F R F'"),
    (Group::Lightning, "L F' L' U' L U F U' L'"),
    (Group::Lightning, "R' F R U R' U' F' U R"),
    (Group::Awkward, "R U R' U R U2 R' F R U R' U' F'"),
    (Group::Awkward, "R' U' R U' R' U2 R F R U R' U' F'"),
    (Group::P, "F' U' L' U L F"),
    (Group::P, "F U R U' R' F'"),
    (Group::T, "F R U R' U' F'"),
    (Group::C, "R' U' R' F R F' U R"),
    (Group::L, "F' L' U' L U L' U' L U F"),
    (Group::L, "F R U R' U' R U R' U' F'"),
    (Group::L, "r U' r2 U r2 U r2 U' r"),
    (Group::L, "r' U r2 U' r2 U' r2 U r'"),
    (Group::I, "F U R U' R' U R U' R' F'"),
    (Group::I, "R U R' U R U' B U' B' R'"),
    (Group::L, "r' U' R U' R' U R U' R' U2 r"),
    (Group::L, "r U R' U R U' R' U R U2 r'"),
    (Group::I, "R U2 R2 U' R U' R' U2 F R F'"),
    (Group::I, "r U r' U R U' R' U R U' R' r U' r'"),
    (Group::CornersOriented, "R U R' U' M' U R U' r'"),
];

const PLL: [(&str, Group, &str); 21] = [
    ("Aa", Group::CornersOnly, "x R' U R' D2 R U' R' D2 R2 x'"),
    ("Ab", Group::CornersOnly, "x R2 D2 R U R' D2 R U' R x'"),
    ("E", Group::CornersOnly, "x' R U' R' D R U R' D' R U R' D R U' R' D' x"),
    ("H", Group::EdgesOnly, "M2 U M2 U2 M2 U M2"),
    ("Ua", Group::EdgesOnly, "M2 U M U2 M' U M2"),
    ("Ub", Group::EdgesOnly, "M2 U' M U2 M' U' M2"),
    ("Z", Group::EdgesOnly, "M' U M2 U M2 U M' U2 M2 U'"),
    ("F", Group::AdjacentSwap, "R' U' F' R U R' U' R' F R2 U' R' U' R U R' U R"),
    ("Ja", Group::AdjacentSwap, "R' U L' U2 R U' R' U2 R L U'"),
    ("Jb", Group::AdjacentSwap, "R U R' F' R U R' U' R' F R2 U' R' U'"),
    ("Ra", Group::AdjacentSwap, "R U' R' U' R U R D R' U' R D' R' U2 R' U'"),
    ("Rb", Group::AdjacentSwap, "R2 F R U R U' R' F' R U2 R' U2 R U"),
    ("T", Group::AdjacentSwap, "R U R' U' R' F R2 U' R' U' R U R' F'"),
    ("Na", Group::DiagonalSwap, "R U R' U R U R' F' R U R' U' R' F R2 U' R' U2 R U' R'"),
    ("Nb", Group::DiagonalSwap, "R' U R U' R' F' U' F R U R' F R' F' R U' R"),
    ("V", Group::DiagonalSwap, "R' U R' U' y R' F' R2 U' R' U R' F R F"),
    ("Y", Group::DiagonalSwap, "F R U' R' U' R U R' F' R U R' U' R' F R F'"),
    ("Ga", Group::G, "R2 U R' U R' U' R U' R2 U' D R' U R D'"),
    ("Gb", Group::G, "R' U' R U D' R2 U R' U R U' R U' R2 D"),
    ("Gc", Group::G, "R2 U' R U' R U R' U R2 U D' R U' R' D"),
    ("Gd", Group::G, "R U R' U' D R2 U' R U' R' U R' U R2 D'"),
];

pub static CASES: LazyLock<Vec<Case>> = LazyLock::new(|| {
    let mut cases = Vec::new();
    for (i, alg) in F2L.iter().enumerate() {
        let cube = Cube::case_of(alg).expect("built-in algorithms parse");
        cases.push(Case {
            id: format!("f2l-{}", i + 1),
            set: Set::F2l,
            name: format!("F2L {}", i + 1),
            group: f2l_group(&cube),
            alg,
        });
    }
    for (i, (group, alg)) in OLL.iter().enumerate() {
        cases.push(Case {
            id: format!("oll-{}", i + 1),
            set: Set::Oll,
            name: format!("OLL {}", i + 1),
            group: *group,
            alg,
        });
    }
    for (name, group, alg) in PLL {
        cases.push(Case {
            id: format!("pll-{}", name.to_lowercase()),
            set: Set::Pll,
            name: format!("{name}-perm"),
            group,
            alg,
        });
    }
    cases
});

pub fn find(id: &str) -> Option<&'static Case> {
    CASES.iter().find(|c| c.id == id)
}

pub fn of_set(set: Set) -> impl Iterator<Item = &'static Case> {
    CASES.iter().filter(move |c| c.set == set)
}

fn f2l_group(cube: &Cube) -> Group {
    let in_top = |home: V3| cube.stickers.iter().any(|s| s.home_pos == home && s.pos[1] == 1);
    match (in_top(F2L_CORNER), in_top(F2L_EDGE)) {
        (true, true) if is_basic(cube) => Group::Basic,
        (true, true) => Group::BothInTop,
        (true, false) => Group::EdgeInSlot,
        (false, true) => Group::CornerInSlot,
        (false, false) => Group::BothInSlot,
    }
}

fn is_basic(cube: &Cube) -> bool {
    ["U R U' R'", "U' F' U F", "F' U' F", "R U R'"]
        .into_iter()
        .any(|alg| case_key(Set::F2l, &Cube::case_of(alg).unwrap()) == case_key(Set::F2l, cube))
}

fn f2l_untouched(home: V3) -> bool {
    home[1] <= 0 && home != F2L_CORNER && home != F2L_EDGE
}

fn below_top(home: V3) -> bool {
    home[1] <= 0
}

/// Stickers of the top layer in a fixed reading order: the U face row by
/// row, then the side stickers of the back, right, front and left faces.
fn top_layer_order() -> Vec<(V3, V3)> {
    let mut order = Vec::with_capacity(21);
    for z in -1..=1 {
        for x in -1..=1 {
            order.push(([x, 1, z], [0, 1, 0]));
        }
    }
    for x in -1..=1 {
        order.push(([x, 1, -1], [0, 0, -1]));
    }
    for z in -1..=1 {
        order.push(([1, 1, z], [1, 0, 0]));
    }
    for x in -1..=1 {
        order.push(([x, 1, 1], [0, 0, 1]));
    }
    for z in -1..=1 {
        order.push(([-1, 1, z], [-1, 0, 0]));
    }
    order
}

fn face_letter(f: Face) -> char {
    match f {
        Face::U => 'U',
        Face::D => 'D',
        Face::F => 'F',
        Face::B => 'B',
        Face::R => 'R',
        Face::L => 'L',
    }
}

fn raw_key(set: Set, cube: &Cube) -> String {
    match set {
        Set::F2l => {
            let mut pair: Vec<_> = cube
                .stickers
                .iter()
                .filter(|s| s.home_pos == F2L_CORNER || s.home_pos == F2L_EDGE)
                .map(|s| (s.home_pos, s.home_normal, s.pos, s.normal))
                .collect();
            pair.sort();
            format!("{pair:?}")
        }
        Set::Oll => top_layer_order()
            .into_iter()
            .map(|(p, n)| if cube.sticker_at(p, n).color() == Face::U { 'U' } else { '.' })
            .collect(),
        Set::Pll => top_layer_order()
            .into_iter()
            .map(|(p, n)| face_letter(cube.sticker_at(p, n).color()))
            .collect(),
    }
}

/// Identifies a case regardless of the U turns done before or after the
/// algorithm.
pub fn case_key(set: Set, cube: &Cube) -> String {
    const AUF: [&str; 4] = ["", "U", "U2", "U'"];
    let mut keys = Vec::new();
    for pre in AUF {
        for post in AUF {
            let mut c = Cube::solved();
            c.apply_alg(pre).unwrap();
            for s in &mut c.stickers {
                let target = cube
                    .stickers
                    .iter()
                    .find(|t| t.home_pos == s.pos && t.home_normal == s.normal)
                    .unwrap();
                s.pos = target.pos;
                s.normal = target.normal;
            }
            c.apply_alg(post).unwrap();
            keys.push(raw_key(set, &c));
        }
    }
    keys.into_iter().min().unwrap()
}

#[derive(Debug, PartialEq, Eq)]
pub enum CheckError {
    /// The token that is not a move.
    Parse(String),
    BreaksSolvedPart,
    WrongCase,
}

/// Checks that `alg` is a valid algorithm for its set: it only disturbs the
/// pieces the set is about and does not solve the trivial case.
pub fn validate(set: Set, alg: &str) -> Result<Cube, CheckError> {
    parse(alg).map_err(|e| CheckError::Parse(e.0))?;
    let cube = Cube::case_of(alg).unwrap();
    let intact = match set {
        Set::F2l => cube.pieces_home(f2l_untouched),
        Set::Oll => cube.pieces_home(below_top),
        Set::Pll => {
            cube.pieces_home(below_top)
                && cube
                    .stickers
                    .iter()
                    .filter(|s| s.home_normal == Face::U.normal())
                    .all(|s| s.normal == Face::U.normal())
        }
    };
    if !intact {
        return Err(CheckError::BreaksSolvedPart);
    }
    Ok(cube)
}

/// Checks that `alg` solves the same case as `case`.
pub fn matches_case(case: &Case, alg: &str) -> Result<(), CheckError> {
    let cube = validate(case.set, alg)?;
    let expected = Cube::case_of(case.alg).unwrap();
    if case_key(case.set, &cube) == case_key(case.set, &expected) {
        Ok(())
    } else {
        Err(CheckError::WrongCase)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn check_set(set: Set, expected: usize) {
        let solved_key = case_key(set, &Cube::solved());
        let mut seen: HashMap<String, String> = HashMap::new();
        let mut errors = Vec::new();
        for case in of_set(set) {
            match validate(set, case.alg) {
                Err(e) => errors.push(format!("{}: {e:?} ({})", case.name, case.alg)),
                Ok(cube) => {
                    let key = case_key(set, &cube);
                    if key == solved_key {
                        errors.push(format!("{}: solves nothing", case.name));
                    } else if let Some(other) = seen.insert(key, case.name.clone()) {
                        errors.push(format!("{}: same case as {other}", case.name));
                    }
                }
            }
        }
        assert_eq!(of_set(set).count(), expected);
        assert!(errors.is_empty(), "\n{}", errors.join("\n"));
    }

    #[test]
    fn f2l_cases_are_valid_and_distinct() {
        check_set(Set::F2l, 41);
    }

    #[test]
    fn oll_cases_are_valid_and_distinct() {
        check_set(Set::Oll, 57);
    }

    #[test]
    fn pll_cases_are_valid_and_distinct() {
        check_set(Set::Pll, 21);
    }

    #[test]
    fn sexy_move_has_order_six() {
        let mut c = Cube::solved();
        for _ in 0..6 {
            c.apply_alg("R U R' U'").unwrap();
        }
        assert!(c.is_solved());
        c.apply_alg("R U R' U'").unwrap();
        assert!(!c.is_solved());
    }

    #[test]
    fn rotations_and_slices_agree() {
        let mut a = Cube::solved();
        a.apply_alg("r").unwrap();
        let mut b = Cube::solved();
        b.apply_alg("R M'").unwrap();
        assert_eq!(a, b);
        let mut c = Cube::solved();
        c.apply_alg("x").unwrap();
        let mut d = Cube::solved();
        d.apply_alg("R M' L'").unwrap();
        assert_eq!(c, d);
    }

    #[test]
    fn sune_matches_case_with_aufs() {
        let sune = find("oll-27").unwrap();
        assert_eq!(matches_case(sune, "U R U R' U R U2 R' U2"), Ok(()));
        assert_eq!(matches_case(sune, "R U2 R' U' R U' R'"), Err(CheckError::WrongCase));
        assert_eq!(matches_case(sune, "R U R"), Err(CheckError::BreaksSolvedPart));
    }

    #[test]
    fn turned_algorithms_solve_the_same_case() {
        for case in CASES.iter().filter(|c| c.set != Set::F2l) {
            let mut alg = case.alg.to_owned();
            for _ in 0..3 {
                alg = crate::cube::turn_alg(&alg);
                assert_eq!(matches_case(case, &alg), Ok(()), "{}: {alg}", case.name);
            }
        }
        assert_eq!(crate::cube::turn_alg("R U2 x' M S' z2"), "B U2 z S M x2");
    }

    #[test]
    fn invert_reverses_moves() {
        assert_eq!(invert("R U2 R' (U' F2')"), "F2 U R U2 R'");
    }
}

#[cfg(test)]
mod search {
    use std::collections::{HashMap, HashSet};

    use super::*;
    use crate::cube::Move;

    const GEN: [&str; 9] = ["R", "R'", "R2", "U", "U'", "U2", "F", "F'", "F2"];

    fn dfs(
        cube: &mut Cube,
        path: &mut Vec<usize>,
        depth: usize,
        moves: &[Move],
        known: &HashSet<String>,
        found: &mut HashMap<String, String>,
    ) {
        if !path.is_empty() && cube.pieces_home(f2l_untouched) {
            let key = case_key(Set::F2l, cube);
            if !known.contains(&key) && !found.contains_key(&key) {
                let seq: Vec<_> = path.iter().map(|&i| GEN[i]).collect();
                found.insert(key, invert(&seq.join(" ")));
            }
        }
        if depth == 0 {
            return;
        }
        for i in 0..GEN.len() {
            if path.last().is_some_and(|&p| p / 3 == i / 3) {
                continue;
            }
            let saved = cube.clone();
            cube.apply_move(moves[i]);
            path.push(i);
            dfs(cube, path, depth - 1, moves, known, found);
            path.pop();
            *cube = saved;
        }
    }

    #[test]
    #[ignore]
    fn missing_f2l() {
        let solved = case_key(Set::F2l, &Cube::solved());
        let mut known: HashSet<String> =
            of_set(Set::F2l).map(|c| case_key(Set::F2l, &Cube::case_of(c.alg).unwrap())).collect();
        known.insert(solved);
        let moves: Vec<Move> = GEN.iter().map(|m| parse(m).unwrap()[0]).collect();
        let mut found = HashMap::new();
        for depth in 1..=10 {
            dfs(&mut Cube::solved(), &mut Vec::new(), depth, &moves, &known, &mut found);
            println!("depth {depth}: {} new", found.len());
            if known.len() + found.len() == 42 {
                break;
            }
        }
        for alg in found.values() {
            let cube = Cube::case_of(alg).unwrap();
            println!("{:30?} {}", f2l_group(&cube), alg);
        }
    }
}
