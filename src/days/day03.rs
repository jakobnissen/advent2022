pub fn solve(s: &str) -> (usize, usize) {
    let trios = parse(s);
    let p1 = trios.iter().flatten().fold(0, |sum, rucksack| {
        sum + rucksack.only_intersection() as usize
    });
    let p2 = trios
        .iter()
        .map(threeway_intersection)
        .fold(0, |x, r| x + r as usize);
    (p1, p2)
}

fn parse(s: &str) -> Vec<[RuckSack; 3]> {
    let lines: Vec<_> = s
        .lines()
        .map(str::trim)
        .filter(|s| !str::is_empty(s))
        .map(str::as_bytes)
        .collect();
    if lines.len() % 3 != 0 {
        println!("Number of lines for RuckSack not divisible by 3");
        std::process::exit(1);
    }
    lines
        .chunks_exact(3)
        .map(|s| {
            [
                RuckSack::new(s[0]),
                RuckSack::new(s[1]),
                RuckSack::new(s[2]),
            ]
        })
        .collect()
}

struct RuckSack(u64, u64);

impl RuckSack {
    fn new(bytes: &[u8]) -> Self {
        if bytes.len() % 2 == 1 {
            println!("Rucksack does not contain en even number of elements");
            std::process::exit(1);
        }
        let (left, right) = bytes.split_at(bytes.len() / 2);
        let (first, last) = (encode(left), encode(right));
        let y = RuckSack(first, last);
        y.only_intersection();
        y
    }

    fn only_intersection(&self) -> u8 {
        let x = self.0 & self.1;
        if x.count_ones() != 1 {
            println!("Rucksack does not have exactly one element shared between both halves");
            std::process::exit(1);
        }
        x.trailing_zeros() as u8
    }
}

fn encode(v: &[u8]) -> u64 {
    let mut x = 0;
    for priorirty in v.iter().map(|&b| match b {
        b'a'..=b'z' => b - b'a' + 1,
        b'A'..=b'Z' => b - b'A' + 27,
        _ => {
            println!("Rucksack contains byte value {} which is invalid", b);
            std::process::exit(1);
        }
    }) {
        x |= 1 << (priorirty & 63);
    }
    x
}

fn threeway_intersection(x: &[RuckSack; 3]) -> u8 {
    let u = x.iter().map(|r| r.0 | r.1).fold(u64::MAX, |x, r| x & r);
    if u.count_ones() != 1 {
        println!("Rucksack trio does not have exactly one element in common");
        std::process::exit(1);
    }
    u.trailing_zeros() as u8
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test() {
        assert_eq!(super::solve(&TEST_STR), (157, 70))
    }
}
