/// Formats milliseconds as `s.cc` or `m:ss.cc`.
pub fn format_ms(ms: i64) -> String {
    let cs = ms / 10;
    let (minutes, seconds, hundredths) = (cs / 6000, cs / 100 % 60, cs % 100);
    if minutes > 0 {
        format!("{minutes}:{seconds:02}.{hundredths:02}")
    } else {
        format!("{seconds}.{hundredths:02}")
    }
}

pub fn format_result(result: Option<i64>) -> String {
    result.map_or_else(|| "DNF".to_owned(), format_ms)
}

/// WCA average of the most recent `n` results (`None` = DNF): the best and
/// worst are dropped, and more than one DNF makes the average a DNF.
/// Returns `None` when there are fewer than `n` results.
pub fn average_of(results: &[Option<i64>], n: usize) -> Option<Option<i64>> {
    if results.len() < n || n < 3 {
        return None;
    }
    let window = &results[..n];
    let dnfs = window.iter().filter(|r| r.is_none()).count();
    if dnfs > 1 {
        return Some(None);
    }
    let mut times: Vec<i64> = window.iter().map(|r| r.unwrap_or(i64::MAX)).collect();
    times.sort_unstable();
    let counted = &times[1..n - 1];
    Some(Some(counted.iter().sum::<i64>() / counted.len() as i64))
}

/// Best rolling average of `n` over the whole history.
pub fn best_average_of(results: &[Option<i64>], n: usize) -> Option<i64> {
    (0..results.len().saturating_sub(n - 1))
        .filter_map(|i| average_of(&results[i..], n).flatten())
        .min()
}

pub fn mean(results: &[i64]) -> Option<i64> {
    (!results.is_empty()).then(|| results.iter().sum::<i64>() / results.len() as i64)
}

const FACES: [char; 6] = ['U', 'D', 'R', 'L', 'F', 'B'];
const SUFFIXES: [&str; 3] = ["", "'", "2"];

fn axis(face: usize) -> usize {
    face / 2
}

/// A 20-move random-turn scramble with no repeated face and no three
/// consecutive turns on one axis. `random(n)` returns an index below `n`.
pub fn scramble(mut random: impl FnMut(usize) -> usize) -> String {
    let mut moves: Vec<usize> = Vec::with_capacity(20);
    while moves.len() < 20 {
        let face = random(FACES.len());
        let n = moves.len();
        if n >= 1 && moves[n - 1] == face {
            continue;
        }
        if n >= 2 && axis(moves[n - 1]) == axis(face) && axis(moves[n - 2]) == axis(face) {
            continue;
        }
        moves.push(face);
    }
    moves
        .into_iter()
        .map(|f| format!("{}{}", FACES[f], SUFFIXES[random(SUFFIXES.len())]))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_times() {
        assert_eq!(format_ms(9_876), "9.87");
        assert_eq!(format_ms(61_005), "1:01.00");
    }

    #[test]
    fn ao5_drops_best_and_worst() {
        let r = [Some(10_000), Some(12_000), Some(11_000), Some(30_000), Some(9_000)];
        assert_eq!(average_of(&r, 5), Some(Some(11_000)));
    }

    #[test]
    fn ao5_counts_one_dnf_as_worst() {
        let r = [Some(10_000), None, Some(11_000), Some(12_000), Some(9_000)];
        assert_eq!(average_of(&r, 5), Some(Some(11_000)));
        let r2 = [None, None, Some(11_000), Some(12_000), Some(9_000)];
        assert_eq!(average_of(&r2, 5), Some(None));
        assert_eq!(average_of(&r[..4], 5), None);
    }

    #[test]
    fn scramble_has_no_repeated_faces() {
        let mut seed = 0x2545_f491_u64;
        let mut random = |n: usize| {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed % n as u64) as usize
        };
        for _ in 0..200 {
            let s = scramble(&mut random);
            let faces: Vec<char> = s.split(' ').map(|m| m.chars().next().unwrap()).collect();
            assert_eq!(faces.len(), 20);
            assert!(faces.windows(2).all(|w| w[0] != w[1]));
        }
    }
}
