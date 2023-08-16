enum Move {
    Rock,
    Paper,
    Scissors
}

fn parse(s: &str) -> Vec<(Move, Move)> {
    s.lines().map(str::trim_end).filter(|x| !x.is_empty()).map(|line| {
        parse_line(line).expect(&format!("Error when parsing line \"{}\"", line))
    }).collect()
}

fn parse_line(s: &str) -> Option<(Move, Move)> {
    let (left, right) = s.split_once(' ')?;
    let left_move = match left {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => return None
    };
    let right_move = match right {
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => return None
    };
    Some((left_move, right_move))
}