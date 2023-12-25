use std::io;

struct Sharky {
    name: String,
    toothbrush_power: u32,
}

impl Sharky {
    fn new(name: &str) -> Sharky {
        Sharky {
            name: String::from(name),
            toothbrush_power: 10,
        }
    }

    fn brush_teeth(&self) {
        println!("{} brushes teeth!", self.name);
    }

    fn swim(&self) {
        println!("{} swims deeper into the ocean.", self.name);
    }

    fn fight_plaque_monster(&self) {
        println!("{} fights the plaque monster!", self.name);
    }

    fn promote_oral_hygiene(&self, target: &str) {
        println!("{} promotes oral hygiene among {}.", self.name, target);
    }

    fn secret_hideout(&self) {
        println!("{}'s secret hideout is equipped for the mission!", self.name);
    }

    // Additional exploration actions
    fn find_treasure(&self) {
        println!("{} discovers a hidden treasure!", self.name);
    }

    fn encounter_mermaid(&self) {
        println!("{} meets a friendly mermaid!", self.name);
    }

    fn discover_shipwreck(&self) {
        println!("{} finds an ancient shipwreck!", self.name);
    }

    fn explore_deep_trench(&self) {
        println!("{} explores a deep ocean trench!", self.name);
    }

    // Introduction actions
    fn introduction(&self) {
        println!("{} welcomes you to the underwater world!", self.name);
        println!("This is a place of wonders and adventures.");
    }

    fn welcome_message(&self) {
        println!("Welcome to Sharky's adventure game!");
        println!("Help {} save the underwater world!", self.name);
    }
}

fn main() {
    let sharky = Sharky::new("Sharky");

    sharky.fight_plaque_monster();
    sharky.promote_oral_hygiene("fish, whales, and pirates");
    sharky.secret_hideout();

    sharky.introduction();
    sharky.welcome_message();

    let mut game_active = true;
    let mut player_score = 0;

    while game_active {
        println!("Choose an action:");
        println!("1. Brush teeth");
        println!("2. Explore");
        println!("3. Quit");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                sharky.brush_teeth();
                player_score += 10;
                println!("Teeth brushed! Score +10");
            }
            2 => {
                println!("You explore the depths.");
                sharky.swim();

                let random_event: u32 = rand::random::<u32>() % 100; // Large random range for branching

                match random_event {
                    0..=24 => {
                        // ... Branching for sunken ship event (as previously implemented)
                        // ...
                    }
                    25..=49 => {
                        // ... Branching for coral reef event (as previously implemented)
                        // ...
                    }
                    50..=74 => {
                        // ... Branching for hidden treasure event (as previously implemented)
                        // ...
                    }
                    75..=84 => {
                        // ... Branching for mysterious underwater cave event (as previously implemented)
                        // ...
                    }
                    85..=94 => {
                        // Another event occurs here with its own branching choices
                        // ...
                    }
                    _ => {
                        // Another event occurs here with its own branching choices
                        // ...
                    }
                }
            }
            3 => {
                println!("Thanks for playing!");
                println!("Final Score: {}", player_score);
                game_active = false;
            }
            _ => {
                println!("Invalid choice. Please choose a valid option.");
            }
        }
    }
}
