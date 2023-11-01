mod models;
use models::character::*;
fn main() {
    let stats = stats::Stats {
        resources: stats::Resources {
            health: 10,
            mana: 10,
            stamina: 10,
        },
        body: stats::Body {
            strength: 10,
            toughness: 10,
            endurance: 10,
        },
        reflexes: stats::Reflexes {
            dexterity: 10,
            agility: 10,
            speed: 10,
        },
        soul: stats::Soul {
            will: 10,
            perception: 10,
            spirit: 10,
        },
        mind: stats::Mind {
            wit: 10,
            knowledge: 10,
            logic: 10,
            sanity: 10,
            faith: 10,
            morality: 10,
        },
        social: stats::Social {
            charisma: 10,
            empathy: 10,
            charm: 10,
            honor: 10,
            reputation: 10,
            fame: 10,
            infamy: 10,
        },
        other: stats::Other {
            luck: 10,
            fate: 10,
            destiny: 10,
        },
    };
    let character = character::Character {
        name: "Bilbo Baggins".to_string(),
        race: race::Race::Human,
        stats,
        virtues: vec![],
        skills: vec![],
        inventory: vec![],
    };
    println!("{:?}", character);
}
