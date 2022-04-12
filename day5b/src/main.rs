fn sorted(a: i32, b: i32) -> (i32, i32) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

#[derive(Debug)]
struct Grid {
    items: Vec<Vec<i32>>,
}

impl Grid {
    fn from_size(n: usize) -> Grid {
        let mut grid: Vec<Vec<i32>> = vec![];

        for _ in 0..n {
            grid.push(vec![0; n]);
        }

        Grid { items: grid }
    }

    fn apply_straight_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        // vertical line
        if x1 == x2 {
            let (y_min, y_max) = sorted(y1, y2);
            for y in y_min..=y_max {
                self.items[x1 as usize][y as usize] += 1;
            }
        }

        // horizontal line
        if y1 == y2 {
            let (x_min, x_max) = sorted(x1, x2);
            for x in x_min..=x_max {
                self.items[x as usize][y1 as usize] += 1;
            }
        }

        // diagonal
        if (x1 - x2).abs() == (y1 - y2).abs() {
            let mut x = x1;
            let mut y = y1;

            let line_length = (x1 - x2).abs();

            self.items[x as usize][y as usize] += 1;

            for _ in 0..line_length {
                // if x2 is bigger than x1, that means that the x increases
                if x2 >= x1 {
                    x += 1;
                } else {
                    x -= 1;
                }

                if y2 >= y1 {
                    y += 1;
                } else {
                    y -= 1;
                }

                self.items[x as usize][y as usize] += 1;
            }
        }
    }

    fn count_overlaps(&self) -> usize {
        self.items.iter().flatten().filter(|i| i >= &&2).count()
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.replace(" -> ", ",")
                .split(',')
                .map(|i| i.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let straight_lines: Vec<&Vec<i32>> = lines
        .iter()
        .filter(|line| {
            (line[0] == line[2])
                || (line[1] == line[3])
                || (line[0] - line[2]).abs() == (line[1] - line[3]).abs()
        })
        .collect();

    // max value in dataset is 989
    let mut grid = Grid::from_size(990);

    straight_lines
        .iter()
        .for_each(|line| grid.apply_straight_line(line[0], line[1], line[2], line[3]));

    println!("{}", grid.count_overlaps());
}
