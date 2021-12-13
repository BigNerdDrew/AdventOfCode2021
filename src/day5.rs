use std::fs;

struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize
}

impl From<&str> for Line {
    fn from(input: &str) -> Line {

        let mut split1 = input.split(" -> ");

        let p1_str = split1.next().unwrap();
        let p2_str = split1.next().unwrap();

        let mut p1 = p1_str.split(",").map(|s| str::parse::<usize>(s).unwrap());
        let mut p2 = p2_str.split(",").map(|s| str::parse::<usize>(s).unwrap());

        Line {
            x1: p1.next().unwrap(), y1: p1.next().unwrap(),
            x2: p2.next().unwrap(), y2: p2.next().unwrap()
        }
    }
}

impl Line {
    fn is_straight(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn all_points(&self) -> Vec<(usize, usize)> {
        if self.is_straight() {
            self.all_straight_points()
        } else {
            self.all_diagonal_points()
        }
    }

    fn all_diagonal_points(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::<(usize, usize)>::new();
 
        let start = (self.x1, self.y1);
        let distance = if self.x2 > self.x1 { self.x2 - self.x1 } else { self.x1 - self.x2 };

        let goingRight = self.x2 > self.x1;
        let goingUp = self.y2 > self.y1;

        for offset in 0..(distance+1) {
            let mut x = start.0;
            let mut y = start.1;

            if goingRight { x += offset } else { x -= offset }
            if goingUp { y += offset } else { y -= offset }

            v.push((x, y))
        }

        v
    }

    fn all_straight_points(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::<(usize, usize)>::new();

        let lesser_x = usize::min(self.x1, self.x2);
        let greater_x = usize::max(self.x1, self.x2);

        let lesser_y = usize::min(self.y1, self.y2);
        let greater_y = usize::max(self.y1, self.y2);

        for x in lesser_x..(greater_x+1) {
            for y in lesser_y..(greater_y+1) {
                v.push((x, y))
            }
        }

        v
    }
}

fn main() {
    let file = fs::read_to_string("day5in.txt").unwrap();
    let input = file.lines();

    let width = 1000;
    let height = 1000;

    let mut grid: Vec<Vec::<u32>> = vec![vec![0; height]; width];

    for line in input {
        let this_line: Line = line.into();

        // if !this_line.is_straight() { continue }

        let points = this_line.all_points();

        for point in points {
            grid[point.0][point.1] += 1;
        }
    }

    let mut count = 0;

    for row in grid {
        for column in row {
            if column >= 2 { count += 1 }
        }
    }

    println!("Safe count: {}", count);
}