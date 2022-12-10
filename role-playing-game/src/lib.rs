pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            None
        } else if self.level < 10 {
            Some(Player {
                health: 100,
                mana: None,
                level: self.level,
            })
        } else {
            Some(Player {
                health: 100,
                mana: Some(100),
                level: self.level,
            })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = if self.health > mana_cost {
                    self.health - mana_cost
                } else {
                    0
                };
                0
            }
            Some(mana) => {
                if mana >= mana_cost {
                    self.mana = Some(mana - mana_cost);
                    2 * mana_cost
                } else {
                    0
                }
            }
        }
    }
}
