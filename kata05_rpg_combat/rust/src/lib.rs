use std::{collections::BTreeSet, num::NonZeroU32};

use uuid::Uuid;

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

#[derive(Clone, Debug)]
pub struct Character {
    id: Uuid,
    /// Invariant: must not be higher than max health allowed by the level.
    health: u32,
    pub level: Level,
    pub factions: BTreeSet<&'static str>,
}

impl Character {
    pub fn new() -> Self {
        let level = Level::one();
        Self {
            id: Uuid::new_v4(),
            health: level.max_health(),
            level,
            factions: BTreeSet::new(),
        }
    }

    pub fn same_character(&self, other: &Self) -> bool {
        self.id == other.id
    }

    pub fn is_ally(&self, other: &Self) -> bool {
        self.factions.intersection(&other.factions).next().is_some()
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

    pub fn take_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage);
    }

    /// Gain the given amount of health, up to the maximum.
    pub fn gain_health(&mut self, amount: u32) {
        self.set_health(self.health + amount);
    }
}

impl Default for Character {
    fn default() -> Self {
        Self::new()
    }
}

/// Heal target by the given amount up to maximum.
///
/// Cannot be used by dead characters.
///
/// Can only target self and allies.
pub fn heal(c: &Character, target: &mut Character, amount: u32) -> Result<(), CannotHeal> {
    if !c.alive() {
        return Err(CannotHeal::Dead);
    } else if !(c.same_character(target) || c.is_ally(target)) {
        return Err(CannotHeal::NotSelfOrAlly);
    }

    target.gain_health(amount);
    Ok(())
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CannotHeal {
    #[error("cannot heal while dead")]
    Dead,
    #[error("can only heal self or allies")]
    NotSelfOrAlly,
}

/// Attacker deals damage to defender.
///
/// Cannot damage self or members of the same faction.
///
/// Returns the amount of damage dealt.
pub fn deal_damage(
    atk: &Character,
    def: &mut Character,
    mut damage: u32,
) -> Result<(), CannotDamage> {
    if atk.same_character(def) {
        return Err(CannotDamage::SameCharacter);
    } else if atk.is_ally(def) {
        return Err(CannotDamage::SameFaction);
    }

    let level_difference = atk.level.0.get() as i32 - def.level.0.get() as i32;
    if level_difference >= 5 {
        damage += damage / 2;
    } else if level_difference <= -5 {
        damage /= 2;
    }

    def.take_damage(damage);
    Ok(())
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CannotDamage {
    #[error("cannot damage self")]
    SameCharacter,
    #[error("cannot damage members of shared faction")]
    SameFaction,
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
            ..Character::new()
        };
        assert!(!c.alive());
    }

    #[test]
    fn dealt_damage_is_subtracted_from_health() {
        let atk = Character::new();
        let mut def = Character {
            health: 500,
            ..Character::new()
        };
        deal_damage(&atk, &mut def, 100).expect("can deal damage");
        assert_eq!(def.health, 400);
    }

    #[test]
    fn when_received_damage_exceeds_health_then_character_dies() {
        let mut c = Character {
            health: 100,
            ..Character::new()
        };
        c.take_damage(200);
        assert!(!c.alive());
        assert_eq!(c.health, 0);
    }

    #[test]
    fn cannot_deal_damage_to_self() {
        let atk = Character::new();
        let mut def = atk.clone();
        assert_eq!(
            deal_damage(&atk, &mut def, 1),
            Err(CannotDamage::SameCharacter)
        );
    }

    #[test]
    fn new_character_dies_after_max_health_damage() {
        let mut c = Character::new();
        c.take_damage(c.level.max_health());
        assert!(!c.alive());
    }

    #[test]
    fn alive_character_can_self_heal() {
        let mut c = Character {
            health: 100,
            ..Character::new()
        };
        assert!(c.health < c.level.max_health());

        heal(&c.clone(), &mut c, 10).expect("should succeed for non-dead");
        assert_eq!(c.health, 110);
    }

    #[test]
    fn dead_cannot_heal_self() {
        let mut c = Character::new();
        c.set_health(0);
        assert!(!c.alive());
        assert_eq!(heal(&c.clone(), &mut c, 1), Err(CannotHeal::Dead));
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
    fn level_1_to_5_cannot_heal_over_1000() {
        let mut c1 = Character::new();
        let mut c5 = Character {
            level: Level::new(5),
            ..Character::new()
        };
        c1.set_health(2000);
        c5.set_health(2000);
        assert!(!c1.health() > 1000);
        assert!(!c5.health() > 1000);
    }

    #[test]
    fn level_6_plus_cannot_heal_over_1500() {
        let mut c6 = Character {
            level: Level::new(6),
            ..Character::new()
        };
        let mut c100 = Character {
            level: Level::new(100),
            ..Character::new()
        };
        c6.set_health(2000);
        c100.set_health(2000);
        assert!(!c6.health() > 1500);
        assert!(!c100.health() > 1500);
    }

    #[test]
    fn attacking_plus_5_level_deals_half_damage() {
        let atk = Character::new();
        let mut def = Character {
            level: Level::new(6),
            ..Character::new()
        };
        assert!(atk.level.0.get() + 5 <= def.level.0.get());

        def.set_health(500);
        deal_damage(&atk, &mut def, 20).expect("can deal damage");
        assert_eq!(def.health, 490);
    }

    #[test]
    fn attacking_minus_5_level_deals_extra_damage() {
        let atk = Character {
            level: Level::new(6),
            ..Character::new()
        };
        let mut def = Character {
            level: Level::one(),
            ..Character::new()
        };
        assert!(atk.level.0.get() >= 5 + def.level.0.get());

        def.set_health(500);
        deal_damage(&atk, &mut def, 20).expect("can deal damage");
        assert_eq!(def.health, 470);
    }
}

#[cfg(test)]
mod factions {
    use super::*;

    fn red_pandas() -> (Character, Character) {
        let c1 = Character {
            factions: ["red pandas", "blue chinchillas"].into(),
            ..Character::new()
        };
        let c2 = Character {
            factions: ["red pandas", "turquoise leprechauns"].into(),
            ..Character::new()
        };
        (c1, c2)
    }

    #[test]
    fn new_character_has_no_faction() {
        let c = Character::new();
        assert!(c.factions.is_empty());
    }

    #[test]
    fn can_join_and_leave_factions() {
        let mut c = Character::new();
        c.factions.insert("red pandas");
        c.factions.remove("red pandas");
    }

    #[test]
    fn same_faction_means_are_allies() {
        let (c1, c2) = red_pandas();
        assert!(c1.is_ally(&c2));
    }

    #[test]
    fn cannot_deal_damage_to_allies() {
        let (c1, mut c2) = red_pandas();
        assert!(c1.is_ally(&c2));
        assert_eq!(deal_damage(&c1, &mut c2, 1), Err(CannotDamage::SameFaction));
    }

    #[test]
    fn can_heal_allies() {
        let (c1, mut c2) = red_pandas();
        assert_eq!(heal(&c1, &mut c2, 10), Ok(()));
    }

    #[test]
    fn dead_cannot_heal_allies() {
        let (mut c1, mut c2) = red_pandas();
        c1.set_health(0);
        assert_eq!(heal(&c1, &mut c2, 10), Err(CannotHeal::Dead));
    }

    #[test]
    fn can_only_heal_allies() {
        let (c1, mut c2) = red_pandas();
        c2.factions.remove("red pandas");
        assert_eq!(heal(&c1, &mut c2, 10), Err(CannotHeal::NotSelfOrAlly));
    }
}
