use std::fs;

struct IDRange {
    start: u64,
    end: u64
}

pub fn day2() {
    part_one();
}

fn part_one() {
    let ranges = parse_ids("data/day2");
    let mut cumsum = 0;
    let mut n = 0;
    for r in ranges {
        for id in r.start..r.end {
            let s = id.to_string();
            let l = s.len();
            if l % 2 == 0 && s[..l/2] == s[l/2..] {
                // println!("{s}");
                // println!("{}, {}", &s[..l/2], &s[l/2..]);
                cumsum += id;
                n += 1;
            }
        }
    }
    println!("Found {n} invalid IDs");
    println!("Cumulative sum is {cumsum}");
}

fn parse_ids(filename: &str) -> Vec<IDRange> {
    fs::read_to_string(filename)
        .expect("File not found!")
        .split(",")
        .map( |s| {
            println!("{s}");
            let spl: Vec<&str> = s.split("-").collect();
            println!("{}", spl[0]);
            println!("{}", spl[1]);
            IDRange {
                start: spl[0].trim().parse::<u64>()
                    .expect("Not a number"),
                end: spl[1].trim().parse::<u64>()
                    .expect("Not a number")
            }
        }).collect()
}
