pub fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut total = 0;

    // the hardest part is finding these functions lol
    for pair in lines.windows(2) {
        if pair[1] > pair[0] {
            total = total + 1;
        }
    }

    println!("{}", total)
}
