use std::thread;
use std::{collections::VecDeque, fmt};

use ahash::AHashSet;
use arrayvec::ArrayVec;

use crate::aoc_io::Solver;
use crate::parse;

use log::{debug, trace};

#[derive(Debug, Clone, Copy, Eq, Ord, Hash, PartialEq, PartialOrd)]
enum Effect {
    MagicMissile,
    Drain,
    Shield(i16),
    Poison(i16),
    Recharge(i16),
}

const MAGIC_MISSILE: u16 = 0;
const DRAIN: u16 = 1;
const SHIELD: u16 = 2;
const POISON: u16 = 3;
const RECHARGE: u16 = 4;

const SHIELD_INITIAL_COUNT: i16 = 6;
const POISON_INITIAL_COUNT: i16 = 6;
const RECHARGE_INITIAL_COUNT: i16 = 5;

const EFF_RECHARGE: Effect = Effect::Recharge(RECHARGE_INITIAL_COUNT);
const EFF_SHIELD: Effect = Effect::Shield(SHIELD_INITIAL_COUNT);
const EFF_DRAIN: Effect = Effect::Drain;
const EFF_POISON: Effect = Effect::Poison(POISON_INITIAL_COUNT);
const EFF_MAGIC_MISSILE: Effect = Effect::MagicMissile;

const ALL_EFFECTS: [Effect; 5] = [
    EFF_MAGIC_MISSILE,
    EFF_DRAIN,
    EFF_SHIELD,
    EFF_POISON,
    EFF_RECHARGE,
];

impl Effect {
    pub fn cost(&self) -> i32 {
        match self {
            Effect::MagicMissile => 53,
            Effect::Drain => 73,
            Effect::Shield(_) => 113,
            Effect::Poison(_) => 173,
            Effect::Recharge(_) => 229,
        }
    }

    pub fn ordinal(&self) -> u16 {
        match self {
            Effect::MagicMissile => MAGIC_MISSILE,
            Effect::Drain => DRAIN,
            Effect::Shield(_) => SHIELD,
            Effect::Poison(_) => POISON,
            Effect::Recharge(_) => RECHARGE,
        }
    }

    pub fn apply(&self, player: &mut Player, boss: &mut Boss) -> Option<Effect> {
        use Effect::*;
        match self {
            MagicMissile => {
                trace!("Player casts Magic Missile, dealing 4 damage.");
                boss.hp -= 4;
                None
            }
            Drain => {
                trace!("Drain deals 2 damage and heals 2 hit points.");
                boss.hp -= 2;
                player.hp += 2;
                None
            }
            Shield(n) => {
                let n = *n;
                if n == SHIELD_INITIAL_COUNT {
                    trace!("Player casts Shield, increasing armor by 7.");
                    player.armor += 7;
                } else if n == 0 {
                    trace!("Shield wears off, decreasing armor by 7.");
                    player.armor -= 7;
                    return None;
                }
                Some(Shield(n - 1))
            }
            Poison(n) => {
                let n = *n;
                if n == POISON_INITIAL_COUNT {
                    trace!("Player casts Poison.");
                } else {
                    trace!("Poison deals 3 damage");
                    boss.hp -= 3;
                }
                if n == 0 {
                    trace!("Poison wears off.");
                    return None;
                }
                Some(Poison(n - 1))
            }
            Recharge(n) => {
                let n = *n;
                if n == RECHARGE_INITIAL_COUNT {
                    trace!("Player casts Recharge.");
                } else {
                    trace!("Recharge provides 101 mana");
                    player.mana += 101;
                }
                if n == 0 {
                    trace!("Recharge wears off.");
                    return None;
                }
                Some(Recharge(n - 1))
            }
        }
    }
}

#[derive(Debug, Clone, Eq, Ord, Hash, PartialEq, PartialOrd)]
struct Player {
    hp: i32,
    armor: i32,
    mana: i32,
}

impl Player {
    pub fn new(hp: i32, mana: i32) -> Self {
        Self { hp, mana, armor: 0 }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Player has {} hit points, {} armor, {} mana",
            self.hp, self.armor, self.mana
        )
    }
}

#[derive(Debug, Clone, Eq, Ord, Hash, PartialEq, PartialOrd)]
struct Boss {
    hp: i32,
}

impl fmt::Display for Boss {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Boss has {} hit points", self.hp)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Day22 {}

#[allow(clippy::new_without_default)]
impl Day22 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solver for Day22 {
    fn day(&self) -> i32 {
        22
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let buf = input;
        let (buf, hp) = parse::positive(buf, true).unwrap();
        let (_, damage) = parse::positive(buf, true).unwrap();

        let boss_hp = hp as i32;
        let boss_dmg = damage as i32;
        let boss = Boss { hp: boss_hp };

        let player = Player::new(50, 500);
        let game = GameState::new(player, boss);

        let game1 = game.clone();
        let thread1 = thread::spawn(move || bfs(game1, boss_dmg, false));

        let part2 = bfs(game, boss_dmg, true);
        let part1 = thread1.join().unwrap();

        (part1.to_string(), part2.to_string())
    }
}

fn bfs(game: GameState, boss_dmg: i32, hard_mode: bool) -> i32 {
    let mut queue: VecDeque<GameState> = VecDeque::with_capacity(1024);
    let mut visited: AHashSet<GameState> = AHashSet::with_capacity(1024);
    queue.push_front(game.clone());
    visited.insert(game);

    let mut min_mana: i32 = i32::MAX;

    while let Some(game) = queue.pop_front() {
        debug!("[bfs] Processing {}", game);
        let mut push_count: u32 = 0;
        for eff in ALL_EFFECTS {
            let mut game = game.clone();
            let status = game.play(eff, boss_dmg, hard_mode);
            if game.mana_spent >= min_mana {
                continue;
            }
            debug!("Playing {:?} results in {:?}", eff, status);
            let is_new = visited.insert(game.clone());
            match status {
                GameResult::Pending => {
                    if is_new {
                        queue.push_front(game);
                        push_count += 1;
                    }
                }
                GameResult::PlayerWins => {
                    debug!("Player wins! Mana spent: {}", game.mana_spent);
                    if game.mana_spent < min_mana {
                        min_mana = game.mana_spent;
                    }
                }
                _ => {}
            }
        }
        debug!(
            "[bfs] added {} new game states. queue len: {}",
            push_count,
            queue.len()
        );
    }
    min_mana
}

#[derive(Debug, PartialEq, Eq)]
enum GameResult {
    PlayerWins,
    BossWins,
    Pending,
    /// e.g. negative mana
    Invalid,
}

#[derive(Debug, Clone, Eq, PartialOrd, Ord, Hash, PartialEq)]
struct GameState {
    pub player: Player,
    pub boss: Boss,
    effects: ArrayVec<Effect, 3>,
    effect_kinds: u16, // bitset
    mana_spent: i32,   // keep track of mana spent
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GameState:
- {}
- {}
- effects: {:?}",
            self.player, self.boss, self.effects,
        )
    }
}

impl GameState {
    pub fn new(player: Player, boss: Boss) -> Self {
        Self {
            player,
            boss,
            effects: ArrayVec::new(),
            effect_kinds: 0,
            mana_spent: 0,
        }
    }

    fn apply_effects(&mut self) {
        let mut effects: ArrayVec<Effect, 3> = ArrayVec::new();

        for eff in self.effects.iter() {
            if let Some(new_eff) = eff.apply(&mut self.player, &mut self.boss) {
                unsafe {
                    effects.push_unchecked(new_eff);
                }
            } else {
                let i: u16 = eff.ordinal();
                self.effect_kinds &= !(1 << i); // unset i-th bit
            }
        }
        self.effects = effects;
    }

    pub fn play(&mut self, player_eff: Effect, boss_dmg: i32, hard_mode: bool) -> GameResult {
        let i = player_eff.ordinal();
        let cost = player_eff.cost();

        if hard_mode {
            self.player.hp -= 1;
            if self.player.hp <= 0 {
                return GameResult::BossWins;
            }
        }

        if cost > self.player.mana {
            debug!("not enough mana left to play {:?}", player_eff);
            return GameResult::Invalid;
        }

        trace!("-- Player turn --");
        trace!("- {}", self.player);
        trace!("- {}", self.boss);
        // Effects apply at the start of both the player's turns and the boss' turns.
        self.apply_effects();
        if self.boss.hp <= 0 {
            return GameResult::PlayerWins;
        }

        // You cannot cast a spell that would start an effect which is already active.
        // However, effects can be started on the same turn they end.
        if (self.effect_kinds & (1 << i)) != 0 {
            debug!(
                "You cannot cast {:?} that would start an effect which is already active.",
                player_eff
            );
            return GameResult::Invalid;
        }

        self.player.mana -= cost;
        self.mana_spent += cost;

        if let Some(new_eff) = player_eff.apply(&mut self.player, &mut self.boss) {
            self.effect_kinds |= 1 << i; // set i-th bit
            unsafe {
                self.effects.push_unchecked(new_eff);
            }
        } else {
            self.effect_kinds &= !(1 << i); // unset i-th bit
        }
        if self.boss.hp <= 0 {
            return GameResult::PlayerWins;
        }

        trace!("-- Boss turn --");
        trace!("- {}", self.player);
        trace!("- {}", self.boss);
        self.apply_effects();
        if self.boss.hp <= 0 {
            return GameResult::PlayerWins;
        }
        let damage = std::cmp::max(1, boss_dmg - self.player.armor);
        trace!("Boss attacks for {damage} damage!");
        self.player.hp -= damage;
        if self.player.hp <= 0 {
            return GameResult::BossWins;
        }

        GameResult::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn example1() {
        let mut game = GameState::new(Player::new(10, 250), Boss { hp: 13 });
        let effects = vec![EFF_POISON, EFF_MAGIC_MISSILE];
        let mut result = GameResult::Pending;
        for eff in effects {
            result = game.play(eff, 8, false);
        }
        assert_eq!(GameResult::PlayerWins, result);
        assert_eq!(2, game.player.hp);
        assert_eq!(24, game.player.mana);
    }

    #[test]
    fn example2() {
        let mut game = GameState::new(Player::new(10, 250), Boss { hp: 14 });
        let effects = vec![
            EFF_RECHARGE,
            EFF_RECHARGE,
            EFF_SHIELD,
            EFF_DRAIN,
            EFF_POISON,
            EFF_MAGIC_MISSILE,
        ];
        let mut result = GameResult::Pending;
        for eff in effects {
            result = game.play(eff, 8, false);
        }
        assert_eq!(GameResult::PlayerWins, result);
        assert_eq!(1, game.player.hp);
        assert_eq!(114, game.player.mana);
    }

    #[test]
    fn duplicate() {
        let mut game = GameState::new(Player::new(10, 250), Boss { hp: 14 });
        let result = game.play(EFF_SHIELD, 8, false);
        assert_eq!(GameResult::Pending, result);
        let result = game.play(EFF_SHIELD, 8, false);
        assert_eq!(GameResult::Invalid, result);
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day22::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("1824", answer.0);
        assert_eq!("1937", answer.1);
    }
}
