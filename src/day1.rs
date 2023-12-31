use std::cmp::max;
use crate::topk::TopK;

pub fn day1_part_a(contents: &str) -> i32 {
    // track max and current calories in accumulator
    let foldresult = contents
        .lines()
        .fold((0, 0), |acc, x| {
            match x {
                "" => (max(acc.0, acc.1), 0),
                y => (acc.0, acc.1 + y.parse::<i32>().unwrap())
            }
        });
    
        foldresult.0
}

fn contents_to_calories(contents: &str) -> Vec<i32> {
    // return the total calories for each elf
    let mut iter = contents.lines();

    let mut result = Vec::<i32>::new();
    loop {
        let total_calories = iter
            .by_ref()
            .take_while(|line| line != &"")
            .map(|cal_str| cal_str.parse::<i32>().unwrap())
            .sum();
        
        if total_calories > 0 {
        result.push(total_calories);
        } else {
            break
        };
    }
    return result;

}

pub fn day1_part_b(contents: &str) -> i32 {
    // find the 3 elves with the most calories
    let topk_calories = contents_to_calories(contents)
        .iter()
        .fold(TopK::new(3), |mut acc, x| {
            acc.push(*x);
            acc
        });

    // sum their total values
    topk_calories
        .values()
        .iter()
        .sum()
}

#[cfg(test)]

mod tests {
    use super::*; 

    #[test]
    fn test_contents_to_calories() {
        let contents = "\
10
20

10
10";
        let result = contents_to_calories(&contents);
        assert_eq!(vec![30, 20], result);
    }
}