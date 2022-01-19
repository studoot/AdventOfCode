use std::path::{Component, Path};

mod part1 {
    pub fn run() -> isize {
        // The simpler solution (cartesian product of iterating 1..97 for
        // sprinkles, butterscotch and candy is a bit slower than this one, as
        // it iterates through more illegal states (where the sum of ingredients
        // > 100)
        (1isize..97)
            .map(|sprinkles| {
                (1..98 - sprinkles)
                    .map(|butterscotch| {
                        (1..99 - sprinkles - butterscotch)
                            .map(|chocolate| {
                                // Sprinkles: capacity 2, durability 0, flavor -2, texture 0, calories 3
                                // Butterscotch: capacity 0, durability 5, flavor -3, texture 0, calories 3
                                // Chocolate: capacity 0, durability 0, flavor 5, texture -1, calories 8
                                // Candy: capacity 0, durability -1, flavor 0, texture 5, calories 8
                                let candy = 100 - (sprinkles + butterscotch + chocolate);
                                let capacity = 2 * sprinkles;
                                let durability = (5 * butterscotch - candy).max(0);
                                let flavor = (5 * chocolate - 3 * butterscotch - 2 * sprinkles).max(0);
                                let texture = (5 * candy - chocolate).max(0);
                                capacity * durability * flavor * texture
                            })
                            .max()
                            .unwrap()
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }

    #[test]
    fn test_run() {
        let max_score = (1isize..99)
            .map(|butterscotch| {
                let cinnamon = 100 - butterscotch;
                // Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
                // Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
                let capacity = (2 * cinnamon - butterscotch).max(0);
                let durability = (3 * cinnamon - 2 * butterscotch).max(0);
                let flavor = (6 * butterscotch - 2 * cinnamon).max(0);
                let texture = (3 * butterscotch - cinnamon).max(0);
                capacity * durability * flavor * texture
            })
            .max()
            .unwrap();
        assert_eq!(62842880, max_score)
    }
}

mod part2 {
    pub fn run() -> isize {
        (1isize..97)
            .filter_map(|sprinkles| {
                (1..98 - sprinkles)
                    .filter_map(|butterscotch| {
                        (1..99 - sprinkles - butterscotch)
                            .filter_map(|chocolate| {
                                // Sprinkles: capacity 2, durability 0, flavor -2, texture 0, calories 3
                                // Butterscotch: capacity 0, durability 5, flavor -3, texture 0, calories 3
                                // Chocolate: capacity 0, durability 0, flavor 5, texture -1, calories 8
                                // Candy: capacity 0, durability -1, flavor 0, texture 5, calories 8
                                let candy = 100 - (sprinkles + butterscotch + chocolate);
                                let calories = 3 * sprinkles + 3 * butterscotch + 8 * chocolate + 8 * candy;
                                if calories != 500 {
                                    None
                                } else {
                                    let capacity = 2 * sprinkles;
                                    let durability = (5 * butterscotch - candy).max(0);
                                    let flavor = (5 * chocolate - 3 * butterscotch - 2 * sprinkles).max(0);
                                    let texture = (5 * candy - chocolate).max(0);
                                    Some(capacity * durability * flavor * texture)
                                }
                            })
                            .max()
                    })
                    .max()
            })
            .max()
            .unwrap()
    }

    #[test]
    fn test_run() {
        let max_score = (1isize..99)
            .filter_map(|butterscotch| {
                let cinnamon = 100 - butterscotch;
                // Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
                // Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
                let capacity = (2 * cinnamon - butterscotch).max(0);
                let durability = (3 * cinnamon - 2 * butterscotch).max(0);
                let flavor = (6 * butterscotch - 2 * cinnamon).max(0);
                let texture = (3 * butterscotch - cinnamon).max(0);
                if 8 * butterscotch + 3 * cinnamon == 500 {
                    Some(capacity * durability * flavor * texture)
                } else {
                    None
                }
            })
            .max()
            .unwrap();
        assert_eq!(57600000, max_score)
    }
}

fn main() {
    let day_number = Path::new(file!())
        .components()
        .find_map(|bit| {
            if let Component::Normal(os_name) = bit {
                if let Some(dir_name) = os_name.to_str() {
                    return dir_name.strip_prefix("day-");
                }
            };
            None
        })
        .unwrap()
        .to_lowercase()
        .replace("-", " ")
        .replace("_", " ");
    let now = std::time::Instant::now();
    let part1_ans = part1::run();
    println!("Day {} part 1 - {} - took {} milliseconds.", day_number, part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 21367368);

    let now = std::time::Instant::now();
    let part2_ans = part2::run();
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 1766400);
}
