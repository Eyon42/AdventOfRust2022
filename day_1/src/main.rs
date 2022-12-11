use std::fs;
fn main() {
    let input = fs::read_to_string("input").expect("Error reading input.txt");

    let mut elves: Vec<i32> = Vec::new();
    elves.push(0);
    let mut current_elf = 0;

    for line in input.lines() {
        if line == "" {
            current_elf += 1;
            elves.push(0);
        } else {
            elves[current_elf] += line.parse::<i32>().unwrap();
        }
    }
    elves.sort();

    println!("Carry: {:?}", elves);

    let max_calories_carried_by_elf = elves[elves.len() - 1];
    println!(
        "Max calories carried by elf: {}",
        max_calories_carried_by_elf
    );

    let calories_carried_by_top_3_elves: i32 =
        elves[elves.len() - 1] + elves[elves.len() - 2] + elves[elves.len() - 3];
    println!(
        "Calories carried by top 3 elves: {}",
        calories_carried_by_top_3_elves
    );
}
