fn main() {
    let mut x = 0;
    let mut y = 0;

    include_str!("../input.txt")
        .lines()
        .map(|i| i.split(' ').collect::<Vec<&str>>())
        .for_each(|parts| {
            let amount = parts[1].parse::<i32>().unwrap();

            match parts[0] {
                "forward" => x += amount,
                "down" => y += amount,
                "up" => y -= amount,
                _ => {
                    println!("{}", parts[0]);
                }
            }
        });

    println!("{}", x * y);
}
