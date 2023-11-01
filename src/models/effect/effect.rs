#[derive(Debug)]
pub struct Effect {
    pub name: String,
    pub description: String,
    pub effect_type: EffectType,
    pub value: f32,
}

#[derive(Debug)]
pub struct EffectType {
    pub name: String,
    pub description: String,
    pub value: f32,
}
