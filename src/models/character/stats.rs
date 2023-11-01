#[derive(Debug)]
pub struct Stats {
    pub resources: Resources,
    pub body: Body,
    pub reflexes: Reflexes,
    pub soul: Soul,
    pub mind: Mind,
    pub social: Social,
    pub other: Other,
}

#[derive(Debug)]
pub struct Resources {
    pub health: i32,
    pub mana: i32,
    pub stamina: i32,
}

#[derive(Debug)]
pub struct Body {
    pub strength: i32,
    pub toughness: i32,
    pub endurance: i32,
}

#[derive(Debug)]
pub struct Reflexes {
    pub dexterity: i32,
    pub agility: i32,
    pub speed: i32,
}

#[derive(Debug)]
pub struct Soul {
    pub will: i32,
    pub perception: i32,
    pub spirit: i32,
}

#[derive(Debug)]
pub struct Mind {
    pub wit: i32,
    pub knowledge: i32,
    pub logic: i32,
    pub sanity: i32,
    pub faith: i32,
    pub morality: i32,
}

#[derive(Debug)]
pub struct Social {
    pub charisma: i32,
    pub empathy: i32,
    pub charm: i32,
    pub honor: i32,
    pub reputation: i32,
    pub fame: i32,
    pub infamy: i32,
}

#[derive(Debug)]
pub struct Other {
    pub luck: i32,
    pub fate: i32,
    pub destiny: i32,
}
