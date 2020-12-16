#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_string_rows;

const RANGE_COUNT: usize = 20;
//const RANGE_COUNT: usize = 3;
type Range = (usize, usize);
type Field = (String, Range, Range);

pub fn main() {
    let input: Vec<String> = get_row_input!("16");
    let parse_ticket = |ticket: &String| -> Vec<usize> {
        ticket
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect()
    };
    let ranges: Vec<Field> = input
        .iter()
        .take(RANGE_COUNT)
        .map(|l| {
            let parse_range = |s: &str| -> Range {
                let r_vec: Vec<usize> = s.split("-").map(|n| n.parse::<usize>().unwrap()).collect();
                (r_vec[0], r_vec[1])
            };

            let splitted: Vec<&str> = l.split(" ").collect();
            let mut field_name = splitted[0].to_string();
            let mut range_offset = 0;
            if !field_name.contains(":") {
                field_name += " ";
                field_name += splitted[1];
                range_offset = 1;
            }
            field_name = field_name.replace(":", "");
            let lower_range = parse_range(&splitted[1 + range_offset]);
            let higher_range = parse_range(&splitted[3 + range_offset]);

            (field_name, lower_range, higher_range)
        })
        .collect();
    let your = input
        .iter()
        .skip(RANGE_COUNT + 2)
        .next()
        .map(|t| parse_ticket(t))
        .unwrap();

    let nearby: Vec<Vec<usize>> = input
        .iter()
        .skip(RANGE_COUNT + 5)
        .map(|t| parse_ticket(t))
        .collect();

    let in_range = |r: Range, tn| -> bool { r.0 <= tn && r.1 >= tn };
    let test_range = |r: &Field, tn| -> bool { in_range(r.1, tn) || in_range(r.2, tn) };

    let solution: usize = nearby
        .iter()
        .flatten()
        .filter(|tn| !ranges.iter().any(|r| test_range(r, **tn)))
        .sum();
    println!("day 16 part 1: {}", solution);

    let valid: Vec<Vec<usize>> = nearby
        .into_iter()
        .filter(|t| t.iter().all(|tn| ranges.iter().any(|r| test_range(r, *tn))))
        .collect();
    let mut possibilities = vec![ranges.clone(); RANGE_COUNT];

    loop {
        for t in &valid {
            for (i, n) in t.iter().enumerate() {
                let mut removes = vec![];
                for r in 0..possibilities[i].len() {
                    if !test_range(&possibilities[i][r], *n) {
                        removes.push((i, r));
                    }
                }
                for r in removes {
                    possibilities[r.0].remove(r.1);
                }
            }
        }
        let mut single = vec![];
        for (i, p) in possibilities.iter().enumerate() {
            if p.len() == 1 {
                let ref poss = p[0];
                single.push((i, poss.0.clone()));
            }
        }

        possibilities = possibilities
            .into_iter()
            .enumerate()
            .filter_map(|(i, p)| {
                if p.len() == 1 {
                    return Some(p);
                }

                let res = p
                    .into_iter()
                    .filter_map(|f| {
                        if single.iter().any(|(j, n)| i != *j && f.0 == *n) {
                            None
                        } else {
                            Some(f)
                        }
                    })
                    .collect();
                Some(res)
            })
            .collect();

        if possibilities.iter().flatten().count() == RANGE_COUNT {
            break;
        }
    }
    for p in &possibilities {
        println!("{:?}", p);
    }

    let departure_index: Vec<usize> = possibilities
        .iter()
        .flatten()
        .enumerate()
        .filter_map(|(i, p)| {
            if p.0.contains("departure") {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    println!("{:?}", departure_index);
    let solution: usize = your
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if departure_index.iter().any(|j| *j == i) {
                Some(v)
            } else {
                None
            }
        })
        .product();
    println!("day 16 part 2: {}", solution);
}
