use itertools::Itertools;
use static_init::dynamic;
use std::path::{Component, Path};

#[derive(PartialEq, Eq)]
pub struct Item {
    cost: usize,
    damage: usize,
    armour: usize,
}
impl Item {
    const fn new(cost: usize, damage: usize, armour: usize) -> Self {
        Item { cost, damage, armour }
    }
}

#[dynamic]
static WEAPONS: Vec<Item> = vec![
    // Weapons:    Cost  Damage  Armour
    /* Dagger */ Item::new(8, 4, 0),
    /* Shortsword */ Item::new(10, 5, 0),
    /* Warhammer */ Item::new(25, 6, 0),
    /* Longsword */ Item::new(40, 7, 0),
    /* Greataxe */ Item::new(74, 8, 0),
];

#[dynamic]
static ARMOUR: Vec<Item> = vec![
    // Armour:      Cost  Damage  Armour
    /* Dummy */ Item::new(0, 0, 0),
    /* Leather */ Item::new(13, 0, 1),
    /* Chainmail */ Item::new(31, 0, 2),
    /* Splintmail */ Item::new(53, 0, 3),
    /* Bandedmail */ Item::new(75, 0, 4),
    /* Platemail */ Item::new(102, 0, 5),
];

#[dynamic]
static RINGS: Vec<Item> = vec![
    // Rings:      Cost  Damage  Armour
    /* Dummy */ Item::new(0, 0, 0),
    /* Damage +1 */ Item::new(25, 1, 0),
    /* Damage +2 */ Item::new(50, 2, 0),
    /* Damage +3 */ Item::new(100, 3, 0),
    /* Defense +1 */ Item::new(20, 0, 1),
    /* Defense +2 */ Item::new(40, 0, 2),
    /* Defense +3 */ Item::new(80, 0, 3),
];

pub struct Player {
    hit_points: usize,
    damage: usize,
    armour: usize,
}

impl Player {
    fn new_player(damage: usize, armour: usize) -> Player {
        Player { hit_points: 100, damage, armour }
    }
}

fn play(player: &Player, opponent: &Player) -> bool {
    let player_damage = player.damage.saturating_sub(opponent.armour).max(1);
    let oppo_damage = opponent.damage.saturating_sub(player.armour).max(1);

    (opponent.hit_points / player_damage) <= (player.hit_points / oppo_damage)
}

mod part1 {
    use super::*;

    pub fn run(boss: &Player) -> usize {
        WEAPONS
            .iter()
            .cartesian_product(ARMOUR.iter())
            .cartesian_product(RINGS.iter())
            .cartesian_product(RINGS.iter())
            .filter_map(|(((w, a), r1), r2)| {
                let cost = w.cost + a.cost + r1.cost + r2.cost;
                let player = Player::new_player(
                    w.damage + a.damage + r1.damage + r2.damage,
                    w.armour + a.armour + r1.armour + r2.armour,
                );
                if (r1 != r2 || r1 == &RINGS[0]) && play(&player, boss) {
                    Some(cost)
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    }

    #[test]
    fn test_run() {
        let p = Player { hit_points: 8, damage: 5, armour: 5 };
        let b = Player { hit_points: 12, damage: 7, armour: 2 };
        assert!(play(&p, &b));
    }
}

mod part2 {
    use super::*;

    pub fn run(boss: &Player) -> usize {
        WEAPONS
            .iter()
            .cartesian_product(ARMOUR.iter())
            .cartesian_product(RINGS.iter())
            .cartesian_product(RINGS.iter())
            .filter_map(|(((w, a), r1), r2)| {
                let cost = w.cost + a.cost + r1.cost + r2.cost;
                let player = Player::new_player(
                    w.damage + a.damage + r1.damage + r2.damage,
                    w.armour + a.armour + r1.armour + r2.armour,
                );
                if (r1 != r2 || r1 == &RINGS[0]) && !play(&player, boss) {
                    Some(cost)
                } else {
                    None
                }
            })
            .max()
            .unwrap()
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
    let boss = Player { hit_points: 109, damage: 8, armour: 2 };
    let part1_ans = part1::run(&boss);
    println!("Day {} part 1 - {} - took {} milliseconds.", day_number, part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 111);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(&boss);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 188);
}
