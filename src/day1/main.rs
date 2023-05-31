use std::cmp::Ordering;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let elves = read_elf_calories()?;

    most_calories(&elves);

    top_three_elves_with_most_calories(elves);
    Ok(())
}

fn most_calories(elfs: &Vec<Elf>) {
    // method 1
    let big_elf = &elfs
        .iter()
        .max_by(|a, b| {
            if a.total_calories < b.total_calories {
                Ordering::Less
            } else if a.total_calories == b.total_calories {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        })
        .unwrap();

    println!("elf carrying most calories is {:?}", big_elf);

    // method 2
    let mut largest: i32 = 0;
    let mut i: usize = 0;

    for (index, elem) in elfs.iter().enumerate() {
        if elem.total_calories > largest {
            largest = elem.total_calories;
            i = index;
        }
    }

    println!(
        "elf carrying most calories is at index {} with {} calories",
        i, largest
    )
}

fn top_three_elves_with_most_calories(mut elves: Vec<Elf>) {
    elves.sort_by(|a, b| {
        if a.total_calories < b.total_calories {
            Ordering::Less
        } else if a.total_calories == b.total_calories {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    println!(
        "top three elves are carrying a total of {} calories",
        elves[elves.len() - 1].total_calories
            + elves[elves.len() - 2].total_calories
            + elves[elves.len() - 3].total_calories
    )
}

fn read_elf_calories() -> Result<Vec<Elf>, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("elfs.txt")?;

    let mut elfs = Vec::<Elf>::new();

    let mut total_cal: i32 = 0;

    let mut tmp_calories = Vec::<i32>::new();

    for line in contents.lines() {
        let calorie = match line.parse::<i32>() {
            Ok(int) => int,
            Err(_) => 0,
        };

        if calorie != 0 {
            tmp_calories.push(calorie);
        }

        total_cal += calorie;

        if line == "" {
            elfs.push(Elf {
                total_calories: total_cal,
                calories: tmp_calories.clone(),
            });

            total_cal = 0;
            tmp_calories.clear();
        }
    }

    Ok(elfs)
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Elf {
    pub total_calories: i32,
    pub calories: Vec<i32>,
}
