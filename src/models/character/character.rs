use super::super::item::item::Item;
use super::race::Race;
use super::skill::Skill;
use super::stats::Stats;
use super::virtue::Virtue;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub race: Race,
    pub stats: Stats,
    pub virtues: Vec<Virtue>,
    pub skills: Vec<Skill>,
    pub inventory: Vec<Item>,
}

#[allow(dead_code)]
impl Character {
    pub fn new(
        name: String,
        race: Race,
        stats: Stats,
        virtues: Vec<Virtue>,
        skills: Vec<Skill>,
        inventory: Vec<Item>,
    ) -> Self {
        Self {
            name,
            race,
            stats,
            virtues,
            skills,
            inventory,
        }
    }
}
