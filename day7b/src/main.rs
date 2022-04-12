fn calculate_crab_distance(num: i32, positions: Vec<i32>) -> i32 {
    positions
        .iter()
        .map(|position| {
            let distance = (num - position).abs();
            let fuel_used = (distance * (1 + distance)) / 2;

            fuel_used
        })
        .sum::<i32>()
}

fn main() {
    let input = include_str!("../input.txt");

    let mut crab_positions: Vec<i32> = input
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    crab_positions.sort_unstable();

    let min = *crab_positions.first().expect("no first number");
    let max = *crab_positions.last().expect("no last number");

    let mut laser_posssible_position: Vec<i32> = (min..=max).collect();
    println!("{} -> {}", min, max);

    laser_posssible_position.sort_by(|a, b| {
        let a_sum = calculate_crab_distance(*a, crab_positions.clone());
        let b_sum = calculate_crab_distance(*b, crab_positions.clone());

        a_sum.cmp(&b_sum)
    });

    println!("{:?}", laser_posssible_position[0]);
    println!(
        "{:?}",
        calculate_crab_distance(laser_posssible_position[0], crab_positions)
    )
}
