fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input missing").replace('\r', "");

    let split_input = input.split('\n');

    let mut elves: Vec<u32> = vec![];

    let mut current_elf = 0;
    for line in split_input {
        if line.is_empty() {
            current_elf += 1;
            continue;
        }

        if let None = elves.get(current_elf) {
            elves.push(0);
        }

        elves[current_elf] += line.parse::<u32>().unwrap();
    }

    elves.sort();
    elves.reverse();
    println!("Elf with most calories: {}", elves[0]);

    let total_calories = elves[0] + elves[1] + elves[2];
    println!("Top 3: {}", total_calories);
}
