use std::io;

#[derive(Debug)]
struct Bingo {
    rows: [[i32; 6]; 5],
    cols: [i32; 5],
    marked: [[bool; 5]; 5],
    bingo: bool,
}

impl Bingo {
    pub fn new() -> Self {
        return Bingo {
            rows: [[0; 6]; 5],
            cols: [0; 5],
            marked: [[false; 5]; 5],
            bingo: false,
        };
    }
    
    pub fn add_row(&mut self, s: &str, r: usize) {
        let numbers: Vec<i32> = s
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect(format!("not a row: {}", s).as_str()))
            .collect();

        for c in 0..5 {
            let n = numbers[c];
            self.rows[r][c] = n;
        }
    }

    pub fn mark(&mut self, n: i32) -> bool {
        for r in 0..5 {
            for c in 0..5 {
                if self.rows[r][c] == n {
                    self.rows[r][5] += 1;
                    self.cols[c] += 1;
                    self.marked[r][c] = true;
                    if self.rows[r][5] == 5 || self.cols[c] == 5 {
                        self.bingo = true;
                    }
                }
            }
        }

        return self.bingo;
    }

    pub fn sum_unmarked(&self) -> i32 {
        let mut sum = 0;
        for r in 0..5 {
            for c in 0..5 {
                if !self.marked[r][c] {
                    sum += self.rows[r][c];
                }
            }
        }
        return sum;
    }
}

pub fn run() {
    // read numbers, then boards from input
    let mut numbers_str = String::new();
    io::stdin()
        .read_line(&mut numbers_str)
        .expect("bro do u even line");
    let numbers: Vec<i32> = numbers_str
        .trim()
        .split(",")
        .map(|x| x.parse().expect("oops"))
        .collect();

    let mut boards = Vec::<Bingo>::new();
    let mut r: usize = 0;
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                break;
            }
            Ok(1) => {
                boards.push(Bingo::new());
                r = 0;
            }
            Ok(_) => {
                boards.last_mut().unwrap().add_row(&line, r);
                r += 1;
            }
            Err(_) => {
                panic!("not a line");
            }
        }
    }

    for n in numbers {
        for bingo in &mut boards {
            if bingo.bingo {
                continue;
            }
            if bingo.mark(n) {
                let sum = bingo.sum_unmarked();
                println!("bingo! {:?} x {} = {}", sum, n, sum * n);
            }
        }
    }
}
