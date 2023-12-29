use crate::{part1::hash, structs::*};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut insertion_order = 0;

    let hmap = input
        .sequence
        .iter()
        .fold(HashMap::default(), |mut hm, instr| {
            if instr.ends_with('-') {
                let label = instr.trim_end_matches('-').to_string();
                let r#box = hash(&label);
                hm.remove(&(r#box, label));

                hm
            } else {
                let split = instr.split('=').collect::<Vec<_>>();
                let label = split[0];
                let focal_length: u8 = split[1].parse().unwrap();
                let r#box = hash(label);

                hm.entry((r#box, label.to_string()))
                    .and_modify(|e: &mut (u8, usize)| e.0 = focal_length)
                    .or_insert((focal_length, insertion_order));

                insertion_order += 1;

                hm
            }
        });

    let mut focusing_power = 0;

    for r#box in 0..=255 {
        let mut box_contents = hmap
            .iter()
            .filter(|(k, _)| k.0 == r#box)
            .collect::<Vec<_>>();

        box_contents.sort_by_key(|(_, v)| v.1);

        let mut box_power = 0;

        for (slot, lens) in box_contents.iter().enumerate() {
            box_power += (1 + r#box as usize) * (slot + 1) * (lens.1 .0) as usize;
        }

        focusing_power += box_power;
    }

    focusing_power.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "145");
    }
}
