const WIN_SCORES: [i8; 9] = [3, 6, 0, 0, 3, 6, 6, 0, 3];

pub fn solve(s: &str) -> (isize, isize) {
    let moves = parse(s);
    moves.iter().fold((0, 0), |(p1, p2), (other, x)| {
        (p1 + score_p1(*other, *x), p2 + score_p2(*other, *x))
    })
}

fn score_p1(other: i8, you: i8) -> isize {
    unsafe { (you + 1 + WIN_SCORES.get_unchecked((other * 3 + you) as usize)) as isize }
}

fn score_p2(other: i8, outcome: i8) -> isize {
    let you = (2 + other + outcome) % 3;
    (you + 1 + 3 * outcome) as isize
}

fn parse(s: &str) -> Vec<(i8, i8)> {
    s.lines()
        .map(str::trim_end)
        .filter(|x| !x.is_empty())
        .map(|line| {
            parse_line(line).unwrap_or_else(|| panic!("Error when parsing line \"{}\"", line))
        })
        .collect()
}

fn parse_line(s: &str) -> Option<(i8, i8)> {
    let bytes = s.as_bytes();
    if bytes.len() != 3 {
        return None;
    }
    unsafe {
        if *bytes.get_unchecked(1) != b' ' {
            return None;
        }
        let left_move = match *bytes.get_unchecked(0) {
            b'A' => 0,
            b'B' => 1,
            b'C' => 2,
            _ => return None,
        };
        let right_move = match *bytes.get_unchecked(2) {
            b'X' => 0,
            b'Y' => 1,
            b'Z' => 2,
            _ => return None,
        };
        Some((left_move, right_move))
    }
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "A Y
B X
C Z";

    #[test]
    fn test() {
        assert_eq!(super::solve(&TEST_STR), (15, 12))
    }
}
