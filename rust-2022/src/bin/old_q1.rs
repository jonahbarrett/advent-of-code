use std::{fs, io::{self, BufRead}};

// Note: input path is wrong because I moved this file and I am lazy

fn part_one() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    let content_iter = contents.split("\n\n");
    let mut max = 0;

    for elf in content_iter {
        let mut calories = 0;
        for line in elf.lines() {
            calories += line.parse::<i32>().unwrap();
        }
        if calories > max {
            max = calories;
        }
    }

    println!("Part one: {}", max);
}

fn part_two() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    let content_iter = contents.split("\n\n");
    let mut calories_list: Vec<i32> = vec![];

    for elf in content_iter {
        let mut calories = 0;
        for line in elf.lines() {
            calories += line.parse::<i32>().unwrap();
        }

        calories_list.push(calories)
    }

    calories_list.sort_by(|a, b| b.cmp(a));
    calories_list.resize(3, 0);
    let sum: i32 = calories_list.iter().sum();
    println!("Part Two: {}", sum);
}

// Testing the BufReader and some error handling stuff
fn part_two_reader() -> io::Result<()> {
    let file = fs::File::open("input.txt").expect("Error reading file");

    let reader = io::BufReader::new(file);

    let mut elves: Vec<i32> = vec![];
    let mut calories = 0;

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            elves.push(calories);
            calories = 0;
            continue;
        }

        match line.parse::<i32>() {
            Ok(n) => {
                calories += n;
            },
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }
    }

    elves.sort_by(|a, b| b.cmp(a));

    let sum: i32 = elves[0..3].iter().sum();

    println!("Sum of top three: {}", sum);

    Ok(())
}

fn main() {
    part_one();
    part_two();
    part_two_reader().expect("Reader failed");
}