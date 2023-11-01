use super::super::effect::effect::Effect;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub weight: f32,
    pub value: f32,
    pub quantity: i32,
    pub effect: Vec<Effect>,
}
