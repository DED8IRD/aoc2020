use std::collections::HashMap;

/// 2sum
#[aoc(day1, part1)]
pub fn day1_part1(input: &str) -> i32 {
    let mut difference: HashMap<i32, i32> = HashMap::new();
    let lines = input.lines();
    for line in lines {
        let num: i32 = line.parse().unwrap();
        let dif = 2020 - num;
        if difference.get(&num) == Some(&dif) {
            return num * dif;
        }
        difference.insert(dif, num);
    }
    0_i32
}

/// 3sum
#[aoc(day1, part2)]
pub fn day1_part2(input: &str) -> i32 {
    let mut nums = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    nums.sort();
    for i in 0..nums.len() - 1 {
        let mut difference: HashMap<i32, i32> = HashMap::new();
        for j in 1..nums.len() {
            let dif = 2020 - &nums[i] - &nums[j];
            if difference.get(&nums[j]) == Some(&dif) {
                return dif * &nums[i] * &nums[j];
            }
            difference.insert(dif, nums[j]);
        }
    }
    0_i32
}
