use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    pub x: u32,
    pub y: u32,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [x, y] = s.split(",").collect::<Vec<&str>>().as_slice() {
            return Ok(Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            });
        }
        panic!("could not parse {}", s);
    }
}

#[derive(Debug)]
struct LineSegment {
    pub start: Point,
    pub end: Point,
}

impl LineSegment {
    pub fn is_vertical(&self) -> bool {
        return self.start.x == self.end.x;
    }

    pub fn is_horizontal(&self) -> bool {
        return self.start.y == self.end.y;
    }
}

impl FromStr for LineSegment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [p1_str, p2_str] = s.trim().split(" -> ").collect::<Vec<&str>>().as_slice() {
            let p1: Point = p1_str.parse().unwrap();
            let p2: Point = p2_str.parse().unwrap();
            if p1.x < p2.x || (p1.x == p2.x && p1.y < p2.y) {
                return Ok(LineSegment { start: p1, end: p2 });
            } else {
                return Ok(LineSegment { start: p2, end: p1 });
            }
        }
        panic!("could not parse {}", s);
    }
}

struct Grid {
    grid: HashMap<Point, u32>,
    danger: HashSet<Point>,
}

impl Grid {
    pub fn new() -> Self {
        return Grid {
            grid: HashMap::<Point, u32>::new(),
            danger: HashSet::<Point>::new(),
        };
    }

    pub fn add(&mut self, segment: &LineSegment) {
        if segment.is_horizontal() {
            let y = segment.start.y;
            for x in segment.start.x..=segment.end.x {
                let point = Point { x: x, y: y };
                if self.grid.contains_key(&point) {
                    self.danger.insert(point);
                    *(self.grid.get_mut(&point).unwrap()) += 1;
                } else {
                    self.grid.insert(point, 1);
                }
            }
        } else if segment.is_vertical() {
            let x = segment.start.x;
            for y in segment.start.y..=segment.end.y {
                let point = Point { x: x, y: y };
                if self.grid.contains_key(&point) {
                    self.danger.insert(point);
                    *(self.grid.get_mut(&point).unwrap()) += 1;
                } else {
                    self.grid.insert(point, 1);
                }
            }
        } else {
            let x_range = segment.start.x..=segment.end.x;
            let y_range: Vec<u32> = if segment.start.y < segment.end.y {
                (segment.start.y..=segment.end.y).collect()
            } else {
                (segment.end.y..=segment.start.y).rev().collect()
            };
            for (x, y) in x_range.zip(y_range) {
                let point = Point { x: x, y: y };
                if self.grid.contains_key(&point) {
                    self.danger.insert(point);
                    *(self.grid.get_mut(&point).unwrap()) += 1;
                } else {
                    self.grid.insert(point, 1);
                }
            }
        }
    }

    pub fn danger(&self) -> usize {
        return self.danger.len();
    }
}

pub fn run() {
    let mut segments = Vec::<LineSegment>::new();

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                let segment: LineSegment = line.parse().unwrap();
                segments.push(segment);
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }

    let mut grid = Grid::new();
    for segment in segments {
        grid.add(&segment);
    }

    println!("danger: {}", grid.danger());
}
