use regex::Regex;

fn main() {
    let assignments = read_cleanup_assignments_from_file().unwrap();
    println!("clean up assignment at index 0 is {:?}", assignments[0]);

    let overlapping_assignments = check_which_assignments_fully_contains_the_other(&assignments);
    println!(
        "Number of overlapping assignments is {}",
        overlapping_assignments
    );

    let part2_overlapping_assignments = check_which_assignments_overlap_at_all(&assignments);
    println!(
        "Number of assignments that overlap at all {}",
        part2_overlapping_assignments
    );
}

// part 2
fn check_which_assignments_overlap_at_all(assignments: &Vec<Assignment>) -> u32 {
    let mut overlapping_assignments: u32 = 0;

    for assignment in assignments {
        if (assignment.first_elf.start..=assignment.first_elf.end)
            .contains(&assignment.second_elf.start)
            || (assignment.first_elf.start..=assignment.first_elf.end)
                .contains(&assignment.second_elf.end)
            || (assignment.second_elf.start..=assignment.second_elf.end)
                .contains(&assignment.first_elf.end)
            || (assignment.second_elf.start..=assignment.second_elf.end)
                .contains(&assignment.first_elf.start)
        {
            overlapping_assignments += 1;
        }
    }

    return overlapping_assignments;
}

// part 1
fn check_which_assignments_fully_contains_the_other(assignments: &Vec<Assignment>) -> u32 {
    let mut overlapping_assignments: u32 = 0;

    for assignment in assignments {
        if assignment.first_elf.start <= assignment.second_elf.start
            && assignment.first_elf.end >= assignment.second_elf.end
        {
            overlapping_assignments += 1;
        } else if assignment.first_elf.start >= assignment.second_elf.start
            && assignment.first_elf.end <= assignment.second_elf.end
        {
            overlapping_assignments += 1;
        }
    }

    return overlapping_assignments;
}

fn read_cleanup_assignments_from_file() -> Result<Vec<Assignment>, std::io::Error> {
    lazy_static::lazy_static! {
        static ref REGX: Regex = Regex::new(r"(?P<first_elf_start>\d{1,})-(?P<first_elf_end>\d{1,}),(?P<second_elf_start>\d{1,})-(?P<second_elf_end>\d{1,})").unwrap();
    }

    let contents = match std::fs::read_to_string("cleanup_assignments.txt") {
        Ok(str) => str,
        Err(err) => panic!("failed to read from file with an error of: {}", err),
    };

    let mut assignments: Vec<Assignment> = vec![];

    for line in contents.lines() {
        for caps in REGX.captures_iter(line) {
            assignments.push(Assignment {
                first_elf: AssignmentRange {
                    start: caps["first_elf_start"].parse::<u32>().unwrap(),
                    end: caps["first_elf_end"].parse::<u32>().unwrap(),
                },
                second_elf: AssignmentRange {
                    start: caps["second_elf_start"].parse::<u32>().unwrap(),
                    end: caps["second_elf_end"].parse::<u32>().unwrap(),
                },
            })
        }
    }

    return Ok(assignments);
}

#[derive(Debug)]
pub struct Assignment {
    pub first_elf: AssignmentRange,
    pub second_elf: AssignmentRange,
}

#[derive(Debug)]
pub struct AssignmentRange {
    pub start: u32,
    pub end: u32,
}
