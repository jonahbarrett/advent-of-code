pub fn part_one(input: &str) -> Option<u32> {
    let content_iter = input.split("\n\n");
    let mut max: u32 = 0;

    for elf in content_iter {
        let mut calories = 0;
        for line in elf.lines() {
            calories += line.parse::<u32>().unwrap();
        }
        if calories > max {
            max = calories;
        }
    }

    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let content_iter = input.split("\n\n");
    let mut calories_list: Vec<u32> = vec![];

    for elf in content_iter {
        let mut calories = 0;
        for line in elf.lines() {
            calories += line.parse::<u32>().unwrap();
        }

        calories_list.push(calories)
    }

    calories_list.sort_by(|a, b| b.cmp(a));
    calories_list.resize(3, 0);
    let sum: u32 = calories_list.iter().sum();

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
