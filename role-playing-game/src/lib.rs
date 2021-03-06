pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player {
                health: 100,
                mana: if self.level < 10 { None } else { Some(100) },
                level: self.level,
            }),
            _ => None,
        }
    }

    pub fn cast_spell1(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
            Some(mana) if mana_cost > mana => 0,
            Some(mana) => {
                self.mana = Some(mana - mana_cost);
                mana_cost.checked_mul(2).expect("Damages are overflowing!")
            }
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana.as_mut() {
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
            Some(mana) => mana.checked_sub(mana_cost).map_or(0, |remaining| {
                *mana = remaining;
                mana_cost * 2
            }),
        }
    }
}
