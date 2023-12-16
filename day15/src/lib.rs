use core::array::from_fn;

// label + focal length
type Lens<'a> = (&'a str, u8);

// label + focal lenght (0 means "remove the lens")
type Step<'a> = (&'a str, u8);

fn hash(input: &str) -> u32 {
    input
        .as_bytes()
        .iter()
        .fold(0, |acc, curr| ((acc + *curr as u32) * 17) % 256)
}

fn parse_steps(input: &str) -> Vec<Step> {
    input
        .split(',')
        .map(|step| {
            if step.contains('=') {
                let (label, focal) = step.split_once('=').unwrap();
                return (label, focal.parse().expect("should be between 1-9"));
            }
            (step.strip_suffix('-').expect("should end with -"), 0)
        })
        .collect()
}

pub fn part1(input: &str) -> String {
    input.split(',').map(hash).sum::<u32>().to_string()
}

pub fn part2(input: &str) -> String {
    let mut boxes = from_fn::<Vec<Lens>, 256, _>(|_| Vec::new());
    let steps = parse_steps(input);

    for (label, focal) in steps {
        let b = hash(label) as usize;
        let b = boxes
            .get_mut(b)
            .expect("calculated hash should be between 0-255");
        if focal == 0 {
            // remove lens
            b.retain(|&(l, _)| l != label);
        } else {
            // add or replace lens
            if let Some(index) = b
                .iter()
                .enumerate()
                .find_map(|(index, l)| (l.0 == label).then_some(index))
            {
                b[index] = (label, focal);
            } else {
                b.push((label, focal));
            }
        }
    }

    // calculate the focusing power
    let power = boxes
        .iter()
        .enumerate()
        .map(|(bi, b)| {
            b.iter()
                .enumerate()
                .map(|(li, l)| (bi + 1) * (li + 1) * (l.1 as usize))
                .sum::<usize>()
        })
        .sum::<usize>();

    power.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "1320");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "145");
    }
}
