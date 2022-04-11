fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .windows(3)
        .map(|i| i.iter().sum())
        .collect::<Vec<usize>>();

    let mut total = 0;

    for pair in lines.windows(2) {
        if pair[1] > pair[0] {
            total += 1;
        }
    }

    println!("{}", total)
}
