use std::num::NonZeroUsize;

pub fn solve(s: &str) -> (usize, usize) {
    let v = parse(s);
    let mut buffer = Vec::new();
    let mut sum: usize = 0;
    for maybe_number in v.iter() {
        if let Some(n) = maybe_number {
            sum += usize::from(*n)
        } else {
            buffer.push(sum);
            sum = 0
        }
    }
    if sum > 0 {
        buffer.push(sum)
    }
    buffer.sort_unstable();
    let &p1 = buffer.last().unwrap();
    let p2 = buffer[buffer.len() - 3..].iter().sum();
    (p1, p2)
}

// Returns nonempty Vec. First and last elements are always Some.
// No two None are consecutive.
fn parse(s: &str) -> Vec<Option<NonZeroUsize>> {
    let mut result = Vec::new();
    let mut last_element = None;
    for line in s.lines().map(str::trim_end) {
        let new_element = if line.is_empty() {
            None
        } else {
            Some(line.parse::<NonZeroUsize>().unwrap())
        };
        if last_element.is_some() || new_element.is_some() {
            result.push(new_element)
        }
        last_element = new_element;
    }
    // We know here that first element must be Some,
    // so popping the last None element cannot possibly leave an empty vector
    match result.last() {
        None => panic!("Input file of Day1 contains no nonempty lines"),
        Some(None) => {result.pop();},
        Some(Some(_)) => ()
    }
    result
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test() {
        assert_eq!(super::solve(&super::parse(TEST_STR)), (24000, 45000))
    }
}
