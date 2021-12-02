fn main() {
    let mut x = 0;
    let mut y = 0;

    include_str!("../input.txt")
        .lines()
        .map(|i| i.split(" ").collect::<Vec<&str>>())
        .for_each(|parts| {
            let amount = parts[1].parse::<i32>().unwrap();

            match parts[0] {
                "forward" => x = x + amount,
                "down" => y = y + amount,
                "up" => y = y - amount,
                _ => println!("{}", parts[0]),
            }

            return;
        });

    println!("{}", x * y);
}
