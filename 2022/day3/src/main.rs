fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input missing");

    let rucksacks = input.lines();

    let mut overall_priority = 0;

    for rucksack in rucksacks.clone() {
        let compartment_a = rucksack[0..rucksack.len() / 2].to_string();
        let compartment_b = rucksack[rucksack.len() / 2..].to_string();

        let mut duplicate: Option<char> = None;
        for c in compartment_a.chars() {
            if compartment_b.contains(c) {
                duplicate = Some(c);
                break;
            }
        }

        if let None = duplicate {
            unreachable!();
        }

        let dup_char = duplicate.unwrap();

        let mut priority = dup_char.to_ascii_lowercase() as i32 - 96;
        if dup_char.is_ascii_uppercase() {
            priority += 26;
        }

        overall_priority += priority;
    }

    println!("Priority: {}", overall_priority);

    let rucksack_vec = rucksacks.collect::<Vec<&str>>();
    let chunked_rucksacks = rucksack_vec.chunks(3);

    overall_priority = 0;

    for group in chunked_rucksacks {
        let rucksack_a = group[0];
        let rucksack_b = group[1];
        let rucksack_c = group[2];

        let mut duplicate: Option<char> = None;
        for c in rucksack_a.chars() {
            if rucksack_b.contains(c) && rucksack_c.contains(c) {
                duplicate = Some(c);
                break;
            }
        }

        if let None = duplicate {
            unreachable!();
        }

        let dup_char = duplicate.unwrap();

        let mut priority = dup_char.to_ascii_lowercase() as i32 - 96;
        if dup_char.is_ascii_uppercase() {
            priority += 26;
        }

        overall_priority += priority;
    }

    println!("Priority (part 2): {}", overall_priority);
}
