use day_1;

fn main() {
    process_day_1();
}

fn process_day_1() {
    if let Some(n) = day_1::run("./day_1/data/input.txt") {
        println!("Frequency: {}", n);
    } else {
        println!("Frequency not found");
    }
}
