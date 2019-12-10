mod aoc;
use self::aoc::get_string;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;



fn main() {
    let input = get_string("input_data/day_8");
    let mut layers: Vec<Vec<char>> = input.chars().collect::<Vec<char>>().chunks(WIDTH * HEIGHT).clone().map(|l| l.to_vec()).collect();

    let image = layers.iter_mut()
        .enumerate()
        .fold(vec!(), |mut acc, (i, l)| {
            if i == 0 {
                acc.append(l);
            }
            else {
                for (i, p) in l.iter().enumerate() {
                    if acc[i] == '2' {
                        acc[i] = *p;
                    }
                }
            }
            acc
        });

    for (i, p) in image.iter().enumerate() {

        if (*p == '1') {
            print!("{}", p );
        }
        else {
            print!(" ");
        }
        if (i + 1) % 25 == 0 {
            println!("");
        }
    }





    layers.sort_by(|a, b| {
        let count_zeros = |x: &Vec<char>| x.iter().filter(|&p| *p == '0').count();
        count_zeros(a).partial_cmp(&count_zeros(b)).unwrap()
    });

    let (ones, twos) = layers[0].iter()
        .fold((0, 0), |mut acc, p| {
            if *p == '1' {
                acc.0 += 1;
            }
            if *p == '2' {
                acc.1 += 1;
            }
            acc
        });
    println!("day 8 part 1:{}", ones * twos );
}
