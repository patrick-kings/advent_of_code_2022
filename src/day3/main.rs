fn main() {
    let racksacks = read_racksacks_from_file().unwrap();
    println!("first item in the racksack is {:?}", racksacks[0]);

    let sum_of_priorities = item_in_both_compartments(&racksacks).unwrap();
    println!("sum of priorities is {}", sum_of_priorities);

    let sum_of_priorities_of_badges = item_type_that_corresponds_to_the_badges(&racksacks).unwrap();
    println!(
        "sum of priorities of badges is {}",
        sum_of_priorities_of_badges
    );
}

fn item_type_that_corresponds_to_the_badges(
    racksacks: &Vec<Racksack>,
) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum_of_priorities_of_badges: u32 = 0;

    let mut index: usize = 0;

    while index < racksacks.len() {
        // concatenate the first and second compartments for each racksack
        let first =
            racksacks[index].first_compartment.clone() + &racksacks[index].second_compartment;
        let second = racksacks[index + 1].first_compartment.clone()
            + &racksacks[index + 1].second_compartment;
        let third = racksacks[index + 2].first_compartment.clone()
            + &racksacks[index + 2].second_compartment;

        let mut priority: u32 = 0;

        for first_char in first.chars() {
            for second_char in second.chars() {
                for thrd_char in third.chars() {
                    if first_char == second_char && first_char == thrd_char {
                        if first_char >= 'a' {
                            priority = first_char as u32 - 96;
                        };

                        if first_char <= 'Z' {
                            priority = first_char as u32 - 38;
                        }
                    }
                }
            }
        }

        // increment by 3 to get the next group
        index += 3;

        sum_of_priorities_of_badges += priority;
    }

    Ok(sum_of_priorities_of_badges)
}

fn item_in_both_compartments(racksacks: &Vec<Racksack>) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum_of_priorities: u32 = 0;

    for racksack in racksacks {
        // only one item is misplaced.
        let mut priority: u32 = 0;

        for first_char in racksack.first_compartment.chars() {
            for second_char in racksack.second_compartment.chars() {
                // a-z has priority 1..26
                // A-Z has priority 27..52
                // convert character to ascii then subtract to get the priority
                // if 'a' is 97, subtract 96 to get a priority of 1
                if first_char == second_char {
                    if first_char >= 'a' {
                        priority = first_char as u32 - 96;
                    };

                    if first_char <= 'Z' {
                        priority = first_char as u32 - 38;
                    }
                }
            }
        }
        sum_of_priorities += priority;
    }

    return Ok(sum_of_priorities);
}

fn read_racksacks_from_file() -> Result<Vec<Racksack>, Box<dyn std::error::Error>> {
    let mut racksacks: Vec<Racksack> = Vec::<Racksack>::new();

    let contents = std::fs::read_to_string("racksacks.txt")?;

    for line in contents.lines() {
        let (first, second) = line.split_at(line.len() / 2);

        racksacks.push(Racksack {
            first_compartment: first.to_string(),
            second_compartment: second.to_string(),
        });
    }

    Ok(racksacks)
}

#[derive(Debug)]
pub struct Racksack {
    pub first_compartment: String,
    pub second_compartment: String,
}
