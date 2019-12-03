mod aoc;
use self::aoc::get_string_rows;

type Point=[i32; 2];
type Vector=[i32; 2];

#[derive(Debug)]
struct Line {
    start: Point,
    dir: Vector
}

impl Line {
    fn new(start: Point, end: Point) -> Self{
        let dir = [end[0]-start[0], end[1]-start[1]];
        Line { start, dir }
    }

    fn parallel(&self, other: &Self) -> bool {
        (self.dir[0] == 0 && other.dir[0] == 0) ||
        (self.dir[1] == 0 && other.dir[1] == 0)
    }

    fn point(&self, s: f32) -> Point {
        [self.start[0]+ (self.dir[0] as f32 * s) as i32 ,
        self.start[1]+ (self.dir[1] as f32 * s) as i32]
    }
}

type Wire= Vec<Line>;

fn to_wire(desc: &str) -> Wire {
    let mut lines = vec!();
    let mut start: Point = [0, 0];
    for p in desc.split(",") {
        let mut end = start;
        let delta = p[1..].parse::<i32>().unwrap();
        if p.starts_with("U") {
            end[1] += delta;
        }
        if p.starts_with("D") {
            end[1] -= delta;
        }
        if p.starts_with("L") {
            end[0] -= delta;
        }
        if p.starts_with("R") {
            end[0] += delta;
        }
        let line = Line::new(start, end);
        start[0] += line.dir[0];
        start[1] += line.dir[1];

        lines.push(line);
    }
    lines
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (b[0] - a[0]).abs() + (b[1] - a[1]).abs()
}

fn find_intersection(a: &Line, b: &Line) -> Option<i32>{
    if a.start != b.start {
        let u = &a.dir;
        let v = &b.dir;
        let w = Line::new(b.start, a.start).dir;
        let s =
            (v[1]*w[0] - v[0]*w[1]) as f32 /
            (v[0]*u[1] - v[1]*u[0]) as f32;
        let t =
            (u[0]*w[1] - u[1]*w[0]) as f32 /
            (u[0]*v[1] - u[1]*v[0]) as f32;

        if s > 0.0 && s < 1.0 && t > 0.0 && t < 1.0 {
            let i = a.point(s);
            println!("intersection at {:?}", i);
            return Some(manhattan_distance(&i, &[0, 0]))
        }
    }
    None
}

fn find_intersections(a: &Wire, b: &Wire) -> Option<i32>{
    let mut closest_intersection_distance = std::i32::MAX;
    for i in 0..a.len() {
        let a_line = &a[i];
        for j in 0..b.len() {
            let b_line = &b[j];
            if !a_line.parallel(b_line) {
                if let Some(d) = find_intersection(a_line, b_line) {
                    if d < closest_intersection_distance {
                        closest_intersection_distance = d;
                    }
                }
            }
        }
    }

    if closest_intersection_distance < std::i32::MAX {
        Some(closest_intersection_distance)
    }
    else {
        None
    }
}

fn main() {
    let input = get_string_rows("input_data/day_3");
    //let input = vec!("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    let wires: Vec<Wire> = input.iter().map(|w| to_wire(w)).collect();

    if let Some(i) = find_intersections(&wires[0], &wires[1]) {
        println!("day 3 part 1: {}", i);
    }


}
