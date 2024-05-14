// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1.to_string()
    } else if animal == "gopher" {
        2.0.to_string()
    } else if animal == "snake" {
        3.to_string()
    } else {
        "Unknown".to_owned()
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier.parse::<i32>().unwrap_or(-1) == 1 {
        "Beach"
    } else if identifier.parse::<i32>().unwrap_or(-1) == 2 {
        "Burrow"
    } else if identifier.parse::<i32>().unwrap_or(-1) == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

// No test changes needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
