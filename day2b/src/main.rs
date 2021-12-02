fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    include_str!("../input.txt")
        .lines()
        .map(|i| i.split(" ").collect::<Vec<&str>>())
        .for_each(|parts| {
            let amount = parts[1].parse::<i32>().unwrap();

            match parts[0] {
                "forward" => {
                    x = x + amount;
                    y = y + (aim * amount)
                }
                "down" => aim = aim + amount,
                "up" => aim = aim - amount,
                _ => unreachable!(),
            }

            return;
        });

    println!("({}, {}) -> {}", x, y, x * y);
}
