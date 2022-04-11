fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    include_str!("../input.txt")
        .lines()
        .map(|i| i.split(' ').collect::<Vec<&str>>())
        .for_each(|parts| {
            let amount = parts[1].parse::<i32>().unwrap();

            match parts[0] {
                "forward" => {
                    x += amount;
                    y += aim * amount
                }
                "down" => aim += amount,
                "up" => aim -= amount,
                _ => unreachable!(),
            }
        });

    println!("({}, {}) -> {}", x, y, x * y);
}
