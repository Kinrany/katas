pub struct Character {
  pub health: u32,
}

pub const START_HEALTH: u32 = 1000;

impl Character {
  pub fn new() -> Self {
    Self {
      health: START_HEALTH,
    }
  }

  pub fn new_dead() -> Self {
    Self { health: 0 }
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

/// Heal self to at least the initial amount.
///
/// Cannot be used when dead.
pub fn heal_self(c: &mut Character) -> Result<(), DeadCannotHealSelf> {
  if c.alive() {
    c.health = u32::max(START_HEALTH, c.health);
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
pub fn deal_damage(_attacker: &mut Character, defender: &mut Character, damage: u32) {
  defender.take_damage(damage);
}

#[cfg(test)]
mod tests {
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
    let c = Character { health: 0 };
    assert!(!c.alive());
  }

  #[test]
  fn dealt_damage_is_subtracted_from_health() {
    let mut attacker = Character::new();
    let mut defender = Character { health: 500 };
    deal_damage(&mut attacker, &mut defender, 100);
    assert_eq!(defender.health, 400);
  }

  #[test]
  fn when_received_damage_exceeds_health_character_dies() {
    let mut c = Character { health: 100 };
    c.take_damage(200);
    assert!(!c.alive());
    assert_eq!(c.health, 0);
  }

  #[test]
  fn new_character_dies_after_max_health_damage() {
    let mut character = Character::new();
    character.take_damage(START_HEALTH);
    assert!(!character.alive());
  }

  #[test]
  fn alive_character_can_self_heal() {
    let mut character = Character { health: 100 };
    assert!(character.health < START_HEALTH);
    heal_self(&mut character).expect("should succeed for non-dead");
    assert!(character.health > 100);
  }

  #[test]
  fn dead_cannot_self_heal() {
    let mut character = Character::new_dead();
    assert_eq!(heal_self(&mut character), Err(DeadCannotHealSelf));
  }
}
