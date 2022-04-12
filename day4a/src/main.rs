use regex::Regex;

#[derive(Debug, Clone)]
struct BingoBoard {
    items: Vec<Vec<i32>>,
    numbers_played: Vec<i32>,
}

impl From<&str> for BingoBoard {
    fn from(board: &str) -> BingoBoard {
        let number_regex = Regex::new(r"[0-9]+").unwrap();

        BingoBoard {
            items: board
                .strip_prefix('\n')
                .unwrap_or(board)
                .split('\n')
                .map(|row| {
                    number_regex
                        .find_iter(row)
                        .map(|item| item.as_str().parse::<i32>().unwrap())
                        .collect()
                })
                .collect(),
            numbers_played: vec![],
        }
    }
}

impl BingoBoard {
    fn winning_numbers(&self) -> Option<Vec<i32>> {
        let rows = self.items.to_vec();
        let columns: Vec<Vec<i32>> = (0..5)
            .map(|column| self.items.iter().map(|row| row[column]).collect())
            .collect();

        let rows_and_columns: Vec<Vec<i32>> = vec![rows, columns].concat();

        let res = rows_and_columns
            .iter()
            .find(|nums| nums.iter().all(|i| self.numbers_played.contains(i)));

        res.map(|i| i.to_owned())
    }

    fn is_winning_board(&self) -> bool {
        self.winning_numbers().is_some()
    }

    fn unmarked_sum(&self) -> i32 {
        self.items
            .iter()
            .flatten()
            .filter(|i| !self.numbers_played.contains(i))
            .sum::<i32>()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let lines = input.lines();

    let numbers = input.lines().next().unwrap();
    let numbers: Vec<i32> = numbers
        .split(',')
        .map(|number| number.parse::<i32>().unwrap())
        .collect();

    let bingo_boards: &str = &lines.skip(1).collect::<Vec<&str>>().join("\n");
    let mut bingo_boards: Vec<BingoBoard> =
        bingo_boards.split("\n\n").map(BingoBoard::from).collect();

    for i in numbers {
        for board in &mut bingo_boards {
            board.numbers_played.push(i);

            if board.is_winning_board() {
                println!("{} * {}", board.unmarked_sum(), i);
                println!("{:?}", board.unmarked_sum() * i);
                return;
            }
        }
    }
}
