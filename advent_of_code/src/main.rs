use day_1;

fn main() {
    process_day_1();
    process_day_2();
}

fn process_day_1() {
    if let Some(n) = day_1::part_1::run("./day_1/data/input.txt") {
        println!("Part 1: Frequency {}", n);
    } else {
        println!("Part 1: Frequency not found");
    }

    if let Some(n) = day_1::part_2::run("./day_1/data/input.txt") {
        println!("Part 2: Callibration {}", n);
    } else {
        println!("Part 2: Callibration not found");
    }
}

fn process_day_2() {
    if let Some(n) = day_2::part_1::run("./day_2/data/input.txt") {
        println!("Part 1: Checksum {}", n);
    } else {
        println!("Part 1: Checksum not found");
    }

    if let Some(n) = day_2::part_2::run("./day_2/data/input.txt") {
        println!("Part 2: Matching Id {}", n);
    } else {
        println!("Part 2: Matching Id not found");
    }
}