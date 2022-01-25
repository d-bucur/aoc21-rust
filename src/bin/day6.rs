use aoc::read_lines;

fn solve(days: u32) {
    let mut fish: Vec<u32> = read_lines()
        .next()
        .unwrap()
        .split(",")
        .map(|e| e.parse::<u32>().unwrap())
        .collect();

    for day in 0..days {
        simulate(&mut fish);
        println!("Day {day}");
    }
    println!("{}", fish.len());
}

fn simulate(fish: &mut Vec<u32>) {
    let mut new_fish = 0u32;
    for f in fish.iter_mut() {
        if *f == 0 {
            *f = 6;
            new_fish += 1;
        } else {
            *f -= 1;
        }
    }
    for _ in 0..new_fish {
        fish.push(8);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        "1" => solve(80),
        "2" => solve(256),
        _ => println!("Invalid option"),
    }
}
