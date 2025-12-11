use std::fs;

struct IDRange {
    start: u64,
    end: u64
}

pub fn day2() {
    part_one();
    part_two();
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

fn part_two() {
    let ranges = parse_ids("data/day2");
    let mut cumsum = 0;
    let mut n = 0;
    for r in ranges {
        'idloop: for id in r.start..r.end {
            let s = id.to_string();
            for i in 1..s.len() {
                match  find_repeating_string(&s, i) {
                    Some(x) => {
                        cumsum += id;
                        n += 1;
                        println!("{s}");
                        println!("{}", x);
                        continue 'idloop;
                    },
                    None => ()
                };
            }
        }
    }
    println!("Found {n} invalid IDs");
    println!("Cumulative sum is {cumsum}");
}

fn find_repeating_string(str: &str, freq: usize) -> Option<&str> {
    if str.len() % freq != 0 {
        return None;
    }
    let first = &str[..freq];
    // println!("{first}");
    for i in 1..str.len()/freq {
        // println!("{}", &str[i*freq..(i+1)*freq]);
        if &str[i*freq..(i+1)*freq] != first {
            return None;
        }
    }
    Some(first)
}

fn parse_ids(filename: &str) -> Vec<IDRange> {
    fs::read_to_string(filename)
        .expect("File not found!")
        .split(",")
        .map( |s| {
            // println!("{s}");
            let spl: Vec<&str> = s.split("-").collect();
            // println!("{}", spl[0]);
            // println!("{}", spl[1]);
            IDRange {
                start: spl[0].trim().parse::<u64>()
                    .expect("Not a number"),
                end: spl[1].trim().parse::<u64>()
                    .expect("Not a number")
            }
        }).collect()
}


#[test]
fn repeating_string_test() {
    let val = "121212";
    assert_eq!(find_repeating_string(val, 2), Some("12"));
    let val = "222222222";
    assert_eq!(find_repeating_string(val, 1), Some("2"));
    let val = "123456789123456789";
    assert_eq!(find_repeating_string(val, 9), Some("123456789"));
    let val = "12345";
    for i in 1..3 {
        assert_eq!(find_repeating_string(val, i), None);
    }
}
