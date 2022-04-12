#[derive(Debug)]
struct Fish {
    current_age: i32,
}

impl Fish {
    fn _new_fish(&self) -> Fish {
        Fish { current_age: 8 }
    }

    fn age(&mut self) -> Option<Fish> {
        if self.current_age == 0 {
            self.current_age = 6;

            Some(self._new_fish())
        } else {
            self.current_age -= 1;
            None
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let fish_ages: Vec<i32> = input
        .split(',')
        .map(|age| age.parse::<i32>().unwrap())
        .collect();

    let mut fish: Vec<Fish> = fish_ages
        .iter()
        .map(|age| Fish { current_age: *age })
        .collect();

    let simulation_length = 80;

    for _ in 0..simulation_length {
        let mut new_fish: Vec<Fish> = vec![];

        for f in &mut fish {
            let new_f = f.age();

            if let Some(octopus) = new_f {
                new_fish.push(octopus);
            }
        }

        fish.append(&mut new_fish);
    }

    println!("{}", fish.len());
}
