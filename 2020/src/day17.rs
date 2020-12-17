#[macro_use]
extern crate advent_of_code;

use advent_of_code::get_map;
use std::collections::HashMap;

type Cube = (i32, i32, i32);
type HyperCube = (i32, i32, i32, i32);
type Grid = HashMap<Cube, bool>;
type HyperGrid = HashMap<HyperCube, bool>;
const ACTIVE: char = '#';

fn eval_cube_state<T: std::cmp::Eq + std::hash::Hash>(
    grid: &HashMap<T, bool>,
    cube: &T,
    active_neighbours_count: usize,
) -> bool {
    match grid.get(cube) {
        Some(state) => {
            (*state && active_neighbours_count == 2 || active_neighbours_count == 3)
                || active_neighbours_count == 3
        }
        None => active_neighbours_count == 3,
    }
}

fn eval_hyper_cube(grid: &HyperGrid, cube: &HyperCube) -> (bool, Vec<(HyperCube, bool)>) {
    let mut active_neighbours = 0;
    let mut new_cubes = vec![];
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }
                    let coord = (cube.0 + x, cube.1 + y, cube.2 + z, cube.3 + w);
                    match grid.get(&coord) {
                        Some(state) => {
                            if *state {
                                active_neighbours += 1;
                            }
                        }
                        None => {
                            new_cubes.push((coord, false));
                        }
                    }
                }
            }
        }
    }
    (eval_cube_state(&grid, &cube, active_neighbours), new_cubes)
}

fn eval_cube(grid: &Grid, cube: &Cube) -> (bool, Vec<(Cube, bool)>) {
    let mut active_neighbours = 0;
    let mut new_cubes = vec![];
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                let coord = (cube.0 + x, cube.1 + y, cube.2 + z);
                match grid.get(&coord) {
                    Some(state) => {
                        if *state {
                            active_neighbours += 1;
                        }
                    }
                    None => {
                        new_cubes.push((coord, false));
                    }
                }
            }
        }
    }
    (eval_cube_state(&grid, &cube, active_neighbours), new_cubes)
}

fn eval_grid<T: std::cmp::Eq + std::hash::Hash + std::marker::Copy>(
    mut grid: HashMap<T, bool>,
    cycles: usize,
    eval_cube: fn(&HashMap<T, bool>, &T) -> (bool, Vec<(T, bool)>),
) -> HashMap<T, bool> {
    for _ in 0..cycles {
        grid = grid
            .iter()
            .map(|c| {
                let (new_state, mut new_cubes) = eval_cube(&grid, c.0);
                for nc in &mut new_cubes {
                    let (new_state, _) = eval_cube(&grid, &nc.0);
                    nc.1 = new_state;
                }
                new_cubes.push((*c.0, new_state));
                new_cubes
            })
            .flatten()
            .collect();
    }
    grid
}

fn main() {
    let input = get_map_input!("17");
    let grid: Grid = input
        .iter()
        .map(|(c, s)| ((c.0 as i32, c.1 as i32, 0), *s == ACTIVE))
        .collect();
    let cycles = 6;

    let final_grid = eval_grid(grid, cycles, eval_cube);
    let count = final_grid.iter().filter(|c| *c.1).count();
    println!("Day 17 part 1: {}", count);

    let hyper_grid: HyperGrid = input
        .iter()
        .map(|(c, s)| ((c.0 as i32, c.1 as i32, 0, 0), *s == ACTIVE))
        .collect();
    let final_hyper_grid = eval_grid(hyper_grid, cycles, eval_hyper_cube);
    let count = final_hyper_grid.iter().filter(|c| *c.1).count();
    println!("Day 17 part 2: {}", count);
}
