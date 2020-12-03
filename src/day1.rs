use std::collections::HashMap;

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
