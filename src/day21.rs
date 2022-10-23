use crate::aoc_io::Solver;
use crate::parse;

use log::debug;
use permutator::LargeCombinationIterator;

#[derive(Debug)]
pub struct Day21 {}

#[allow(clippy::new_without_default)]
impl Day21 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
struct Player {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl Player {
    pub fn consume(&mut self, item: &Item) -> i32 {
        self.damage += item.damage;
        self.armor += item.armor;
        item.cost
    }

    pub fn attack(&self, opponent: &mut Player) {
        // Damage dealt by an attacker each turn is equal to the attacker's damage score minus the
        // defender's armor score. An attacker always does at least `1` damage.
        let score = std::cmp::max(1, self.damage - opponent.armor);
        opponent.hp -= score;
    }

    // Equip player and return cost of equipment.
    fn equip(&mut self, weapon: &Item, armor: &Option<Item>, rings: &Vec<&Item>) -> i32 {
        let mut cost = 0;
        cost += self.consume(weapon);
        if let Some(armor) = armor {
            cost += self.consume(armor);
        }
        for r in rings {
            cost += self.consume(r);
        }
        cost
    }
}

#[derive(Debug)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Solver for Day21 {
    fn day(&self) -> i32 {
        21
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let buf = input;
        let (buf, hp) = parse::positive(buf, true).unwrap();
        let (buf, damage) = parse::positive(buf, true).unwrap();
        let (_, armor) = parse::positive(buf, true).unwrap();
        let boss = Player {
            hp: hp as i32,
            damage: damage as i32,
            armor: armor as i32,
        };
        let me = Player {
            hp: 100,
            armor: 0,
            damage: 0,
        };

        let weapons = [
            Item {
                // name: "Dagger",
                cost: 8,
                damage: 4,
                armor: 0,
            },
            Item {
                // name: "Shortsword",
                cost: 10,
                damage: 5,
                armor: 0,
            },
            Item {
                // name: "Warhammer",
                cost: 25,
                damage: 6,
                armor: 0,
            },
            Item {
                // name: "Longsword",
                cost: 40,
                damage: 7,
                armor: 0,
            },
            Item {
                // name: "Greataxe",
                cost: 74,
                damage: 8,
                armor: 0,
            },
        ];

        let armors = [
            None,
            Some(Item {
                // name: "Leather",
                cost: 13,
                damage: 0,
                armor: 1,
            }),
            Some(Item {
                // name: "Chainmail",
                cost: 31,
                damage: 0,
                armor: 2,
            }),
            Some(Item {
                // name: "Splintmail",
                cost: 53,
                damage: 0,
                armor: 3,
            }),
            Some(Item {
                // name: "Bandedmail",
                cost: 75,
                damage: 0,
                armor: 4,
            }),
            Some(Item {
                // name: "Platemail",
                cost: 102,
                damage: 0,
                armor: 5,
            }),
        ];

        let rings = [
            Item {
                // name: "Damage +1",
                cost: 25,
                damage: 1,
                armor: 0,
            },
            Item {
                // name: "Damage +2",
                cost: 50,
                damage: 2,
                armor: 0,
            },
            Item {
                // name: "Damage +3",
                cost: 100,
                damage: 3,
                armor: 0,
            },
            Item {
                // name: "Defense +1",
                cost: 20,
                damage: 0,
                armor: 1,
            },
            Item {
                // name: "Defense +2",
                cost: 40,
                damage: 0,
                armor: 2,
            },
            Item {
                // name: "Defense +3",
                cost: 80,
                damage: 0,
                armor: 3,
            },
        ];

        // Rules:
        // - You must use any items you buy
        // - The shop only has one of each item

        let mut min_cost = i32::MAX;
        let mut max_cost = i32::MIN;

        let empty: Vec<&Item> = Vec::new();
        // You must buy exactly one weapon
        for weapon in &weapons {
            // Armor is optional, but you can't use more than one
            for armor in &armors {
                // You can buy 0-2 rings
                for ring_count in 0..=2 {
                    if ring_count > 0 {
                        let ringiter = LargeCombinationIterator::new(&rings, ring_count);
                        for combination in ringiter {
                            let mut me = me.clone();
                            let mut boss = boss.clone();
                            let cost = me.equip(weapon, armor, &combination);
                            if cost < min_cost || cost > max_cost {
                                let winner = simulate(&mut me, &mut boss);
                                if winner == 0 {
                                    min_cost = std::cmp::min(min_cost, cost);
                                } else {
                                    max_cost = std::cmp::max(max_cost, cost);
                                }
                            }
                        }
                    } else {
                        let mut me = me.clone();
                        let mut boss = boss.clone();
                        let cost = me.equip(weapon, armor, &empty);
                        if cost < min_cost || cost > max_cost {
                            let winner = simulate(&mut me, &mut boss);
                            if winner == 0 {
                                min_cost = std::cmp::min(min_cost, cost);
                            } else {
                                max_cost = std::cmp::max(max_cost, cost);
                            }
                        }
                    }
                }
            }
        }

        (min_cost.to_string(), max_cost.to_string())
    }
}

// Simulate boss battle. Returns 0 if player wins, 1 if boss wins.
fn simulate(me: &mut Player, boss: &mut Player) -> u32 {
    debug!("simulating: {:?} vs {:?}", me, boss);
    let mut idx: u32 = 0;
    loop {
        if idx == 0 {
            me.attack(boss);
            if boss.hp <= 0 {
                return idx;
            }
        } else {
            boss.attack(me);
            if me.hp <= 0 {
                return idx;
            }
        }
        idx = 1 - idx;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn part1_and_part2() {
        let mut d = Day21::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("121", answer.0);
        assert_eq!("201", answer.1);
    }
}
