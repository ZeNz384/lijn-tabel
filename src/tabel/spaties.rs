pub struct Spaties {
    pub er_voor: u32,
    pub er_na: u32,
}
impl Spaties {
    pub fn nummer_naar_spatie(getal: u32) -> String {
        let mut spaties = "".to_string();
        let waarde = " ".to_string();
        for _spaties in 0..getal {
            spaties.push_str(&waarde)
        }
        spaties
    }
    pub fn woord(&self, spaties_er_voor: &str, woord: &str, spaties_er_na: &str) -> String {
        let waarde = format!("{spaties_er_voor}{woord}{spaties_er_na}");
        waarde
    }
    pub fn nummer(&self, spaties_er_voor: &str, getal: &u32, spaties_er_na: &str) -> String {
        let waarde = format!("{spaties_er_voor}{getal}{spaties_er_na}");
        waarde
    }
}
