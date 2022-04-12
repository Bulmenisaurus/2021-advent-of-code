fn update_fish(fish_ages: [i128; 9]) -> [i128; 9] {
    // decrease each fish's internal timer
    let mut ages: [i128; 9] = [0; 9];

    let new_fishes_born = fish_ages[0];

    // all fish with age 1 or older become younger
    // 8 -> 7, 6 -> 5, ... 1 -> 0

    // clippy suggestion is very confusing...
    #[allow(clippy::manual_memcpy)]
    for i in 1..=8 {
        ages[i - 1] = fish_ages[i]
    }

    // all old fish create a new fish of age 8 ...
    ages[8] += new_fishes_born;
    // ... and make themselves have an age of 6
    ages[6] += new_fishes_born;

    ages
}

fn main() {
    let mut amount_of_each_fish_ages: [i128; 9] = [0; 9];
    let input = include_str!("../input.txt");

    let fish_ages: Vec<i128> = input
        .split(',')
        .map(|age| age.parse::<i128>().unwrap())
        .collect();

    // initial condition
    for i in 0..8i128 {
        amount_of_each_fish_ages[i as usize] =
            fish_ages.iter().filter(|&age| *age == i).count() as i128;
    }

    let simulation_steps = 256;

    for _ in 0..simulation_steps {
        amount_of_each_fish_ages = update_fish(amount_of_each_fish_ages);
    }

    println!("{}", amount_of_each_fish_ages.iter().sum::<i128>());
}
