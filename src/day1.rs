use std::fs;

pub fn day1() {
    println!("Test: {}", 100 / 100);
    let directions = read_direction_file("data/day1");
    part_one(&directions);
    part_two(&directions);
}

fn part_one(directions: &Vec<i64>) {
    let mut code = 50;
    let mut counter = 0;
    for dir in directions {
        code += dir;
        code = code % 100;
        if code == 0 {
            counter += 1;
        }
    }
    println!("Number of zero-passings inbetween turns: {}", counter);
}


fn part_two(directions: &Vec<i64>) {
    let mut code = 50;
    let mut counter = 0;
    for dir in directions {
        code += dir;
        println!("{} step {} -> {}", code-dir, dir, code);
        if code > 99 {
            println!("Counting {} more", code/100);
            counter += code/100;
        } else if code <= 0 {
            println!("Counting {} more", (-code+100)/100);
            counter += (-code+100)/100;
            if code-dir == 0 { counter -= 1; }
        }
        code = code % 100;
        if code < 0 {
            code += 100;
        }
    }
    println!("Number of zero-passings in total: {}", counter);
}


fn read_direction_file(filename: &str) -> Vec<i64> {
    fs::read_to_string(filename)
        .expect("Could not open file!")
        .lines()
        .filter( |s: &&str| !s.is_empty() )
        .map( |s: &str| {
            // Call next once on the iterator to get first element
            let letter = s.chars().next()
                .expect("String should not be empty");
            let number = s.get(1..)
                .expect("String is unexpectedly short")
                .parse::<i64>()
                .expect("String end is not a number!");
            if letter == 'R' {number} else {-number}
        })
        .collect()
}
