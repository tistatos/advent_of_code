#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_map;
use std::collections::HashMap;

type Cube = (i32, i32, i32);
type Grid = HashMap<Cube, bool>;
const ACTIVE: char = '#';



fn eval_cube(grid: &Grid, cube: &Cube) -> (bool, Vec<(Cube, bool)>) {
    let mut active_neighbours = 0;
    let mut new_cubes = vec![];
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                let coord = (cube.0 + x,cube.1 + y, cube.2 + z);
                match grid.get(&coord) {
                    Some(state) => { if *state { active_neighbours += 1;}}
                    None => {
                        new_cubes.push((coord, false));
                    }
                }
            }
        }
    }
    let active = match grid.get(&cube) {
                    Some(state) =>  *state,
                   None => false
    };
    if active {
        (active_neighbours == 2 || active_neighbours == 3, new_cubes)
    }
    else {
        (active_neighbours == 3, new_cubes)
    }
}
fn print_grid(grid: &Grid) {
    let grid_limits = |g: &Grid| -> (Cube, Cube) {
        (
        (
            g.keys().map(|c| c.0).min().unwrap(),
            g.keys().map(|c| c.1).min().unwrap(),
            g.keys().map(|c| c.2).min().unwrap()
        ),
        (
            g.keys().map(|c| c.0).max().unwrap(),
            g.keys().map(|c| c.1).max().unwrap(),
            g.keys().map(|c| c.2).max().unwrap()
        )
        )
    };

    let (min, max) = grid_limits(&grid);

    for z in min.2..=max.2 {
        println!("Z = {}", z);
        for y in min.0..=max.0 {
            for x in min.1..=max.1 {
                if *grid.get(&(x,y,z)).unwrap() { print!("#") } else { print!(".") }
            }
            println!("");
        }
        println!("");
    }
}

fn main() {
    let input = get_map_input!("17");
    let mut grid: Grid = input.iter().map(|c| ((c.0.0 as i32, c.0.1 as i32, 0), *c.1 == ACTIVE)).collect();


    println!("Start:");
    print_grid(&grid);

    for i in 0..6 {
        println!("running cycle: {}", i);

        grid = grid.iter().map(|c| {
            let (new_state, mut new_cubes) = eval_cube(&grid, c.0);
            for nc in &mut new_cubes {
                let (new_state, _) = eval_cube(&grid, &nc.0);
                nc.1 = new_state;
            }
            new_cubes.push( (*c.0, new_state));
            new_cubes}).flatten().collect();

            print_grid(&grid);
    }

    let count = grid.iter().filter(|c| *c.1).count();
    println!("Day 17 part 1: {}", count);
}
