use std::num::NonZeroU32;

#[derive(Clone, Copy, Debug)]
pub struct Level(pub NonZeroU32);

impl Level {
    /// # Panics
    ///
    /// Panics if the provided level value is 0.
    pub fn new(level: u32) -> Self {
        Self(NonZeroU32::new(level).expect("level cannot be 0"))
    }

    pub fn one() -> Self {
        Self::new(1)
    }

    pub fn max_health(&self) -> u32 {
        if self.0.get() >= 6 {
            1500
        } else {
            1000
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::one()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Character {
    /// Invariant: must not be higher than max health allowed by the level.
    health: u32,
    pub level: Level,
}

impl Character {
    pub fn new() -> Self {
        let level = Level::one();
        Self {
            health: level.max_health(),
            level,
        }
    }

    pub fn new_dead() -> Self {
        Self {
            health: 0,
            level: Level::one(),
        }
    }

    pub fn health(&self) -> u32 {
        self.health
    }

    /// Sets health to the given value, capped at maximum health.
    ///
    /// Returns the new health value.
    pub fn set_health(&mut self, health: u32) -> u32 {
        self.health = u32::min(self.level.max_health(), health);
        self.health
    }

    pub fn alive(&self) -> bool {
        self.health > 0
    }

    /// Returns `true` if the character is still alive.
    pub fn take_damage(&mut self, damage: u32) -> bool {
        self.health = self.health.saturating_sub(damage);
        self.alive()
    }
}

impl Default for Character {
    fn default() -> Self {
        Self::new()
    }
}

/// Heal self by the given amount up to maximum.
///
/// Cannot be used when dead.
pub fn heal_self(c: &mut Character, amount: u32) -> Result<(), DeadCannotHealSelf> {
    if c.alive() {
        c.set_health(c.health + amount);
        Ok(())
    } else {
        Err(DeadCannotHealSelf)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeadCannotHealSelf;

/// Attacker deals damage to defender.
///
/// Mutable references prevent characters from attacking themselves.
pub fn deal_damage(attacker: &mut Character, defender: &mut Character, mut damage: u32) {
    let level_difference = attacker.level.0.get() as i32 - defender.level.0.get() as i32;
    if level_difference >= 5 {
        damage += damage / 2;
    } else if level_difference <= -5 {
        damage /= 2;
    }

    defender.take_damage(damage);
}

#[cfg(test)]

mod damage_and_health {
    use super::*;

    #[test]
    fn new_character_has_1000_health() {
        let c = Character::new();
        assert_eq!(c.health, 1000);
    }

    #[test]
    fn new_character_is_alive() {
        let c = Character::new();
        assert!(c.alive());
    }

    #[test]
    fn character_with_no_health_is_dead() {
        let c = Character {
            health: 0,
            ..Default::default()
        };
        assert!(!c.alive());
    }

    #[test]
    fn dealt_damage_is_subtracted_from_health() {
        let mut attacker = Character::new();
        let mut defender = Character {
            health: 500,
            ..Default::default()
        };
        deal_damage(&mut attacker, &mut defender, 100);
        assert_eq!(defender.health, 400);
    }

    #[test]
    fn when_received_damage_exceeds_health_character_dies() {
        let mut c = Character {
            health: 100,
            ..Default::default()
        };
        c.take_damage(200);
        assert!(!c.alive());
        assert_eq!(c.health, 0);
    }

    #[test]
    fn new_character_dies_after_max_health_damage() {
        let mut character = Character::new();
        character.take_damage(character.level.max_health());
        assert!(!character.alive());
    }

    #[test]
    fn alive_character_can_self_heal() {
        let mut character = Character {
            health: 100,
            ..Default::default()
        };
        assert!(character.health < character.level.max_health());

        heal_self(&mut character, 10).expect("should succeed for non-dead");
        assert_eq!(character.health, 110);
    }

    #[test]
    fn dead_cannot_self_heal() {
        let mut character = Character::new_dead();
        assert_eq!(heal_self(&mut character, 1), Err(DeadCannotHealSelf));
    }
}

#[cfg(test)]
mod levels {
    use super::*;

    #[test]
    fn new_character_is_level_1() {
        let c = Character::new();
        assert_eq!(c.level.0.get(), 1);
    }

    #[test]
    fn level_1_to_5_characters_cannot_heal_over_1000() {
        let mut c1 = Character::new();
        let mut c5 = Character {
            level: Level::new(5),
            ..Default::default()
        };
        c1.set_health(2000);
        c5.set_health(2000);
        assert!(!c1.health() > 1000);
        assert!(!c5.health() > 1000);
    }

    #[test]
    fn level_6_plus_characters_cannot_heal_over_1500() {
        let mut c6 = Character {
            level: Level::new(6),
            ..Default::default()
        };
        let mut c100 = Character {
            level: Level::new(100),
            ..Default::default()
        };
        c6.set_health(2000);
        c100.set_health(2000);
        assert!(!c6.health() > 1500);
        assert!(!c100.health() > 1500);
    }

    #[test]
    fn attacking_plus_5_level_deals_half_damage() {
        let mut attacker = Character {
            level: Level::one(),
            ..Default::default()
        };
        let mut defender = Character {
            level: Level::new(6),
            health: 500,
        };
        assert!(attacker.level.0.get() + 5 <= defender.level.0.get());

        deal_damage(&mut attacker, &mut defender, 20);
        assert_eq!(defender.health, 490);
    }

    #[test]
    fn attacking_minus_5_level_deals_extra_damage() {
        let mut attacker = Character {
            level: Level::new(6),
            ..Default::default()
        };
        let mut defender = Character {
            level: Level::one(),
            health: 500,
        };
        assert!(attacker.level.0.get() >= 5 + defender.level.0.get());

        deal_damage(&mut attacker, &mut defender, 20);
        assert_eq!(defender.health, 470);
    }
}
