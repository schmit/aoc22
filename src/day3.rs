use std::collections::HashSet;

fn find_duplicate_in_second_half(input: &str) -> Option<char> {
    let mut seen = HashSet::new();
    let midpoint = input.len() / 2;

    let (first, second) = input.split_at(midpoint);

    for ch in first.chars() {
        seen.insert(ch);
    }
    for ch in second.chars() {
        if seen.contains(&ch) {
            return Some(ch)
        }
    }
    None
}

fn char_to_score(ch: char) -> u32 {
    if ch.is_ascii_lowercase() {
        return ch as u32 - 96
    } 
    ch as u32 - 65 + 27
}

pub fn day3_part_a(contents: &str) -> i32 {
    contents
        .lines()
        .map(|line| {
            let duplicate = find_duplicate_in_second_half(line);
            match duplicate {
                Some(ch) => char_to_score(ch),
                None => 0
            }
        })
        .sum::<u32>()
        .try_into()
        .unwrap()
}

mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate_in_second_half() {
        assert_eq!(Some('b'), find_duplicate_in_second_half("ababcd"))
    }

    #[test]
    fn test_char_to_score() {
        assert_eq!(char_to_score('a'), 1);
        assert_eq!(char_to_score('z'), 26);
        assert_eq!(char_to_score('A'), 27);
        assert_eq!(char_to_score('Z'), 52);
    }

    #[test]
    fn test_day3_part_a() {
        let contents = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(day3_part_a(contents), 157);
    }
}