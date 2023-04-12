pub enum Opmaak {
    // PositieSpaties(String, u32, u32, u32, u32, String),
    // BTWvarianten(String, String),
    Streep,
}
impl Opmaak {
    pub fn streep(&self) -> String {
        "|".to_string()
    }
}
