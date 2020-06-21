use std::fmt::Display;

fn main() {
    process_day_1();
    process_day_2();
    process_day_3();
}

fn process_day_1() {
    println!("Day 1");
    process_day("Frequency", "./day_1/data/input.txt", day_1::part_1::run);
    process_day("Callibration", "./day_1/data/input.txt", day_1::part_2::run);
    println!("");
}

fn process_day_2() {
    println!("Day 2");
    process_day("Checksum", "./day_2/data/input.txt", day_2::part_1::run);
    process_day("Matching Id", "./day_2/data/input.txt", day_2::part_2::run);
    println!("");
}

fn process_day_3() {
    println!("Day 3");
    process_day("Overlapping Claims", "./day_3/data/input.txt", day_2::part_1::run);
    println!("");
}

fn process_day<T>(label: &str, input: &str, f: fn (&str) -> Option<T>)
where T: Display
{
    match f(input) {
        Some(val) => println!("{}: {}", label, val),
        None => println!("{} not found", label)
    }
}