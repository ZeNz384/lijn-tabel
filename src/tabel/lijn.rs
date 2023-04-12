pub struct TabelLijn;
impl TabelLijn {
    /// Een teken reeks opmaak weer te geven aan een `boven` of `onder` zijde kant van `tabel` inhoud.
    pub fn regel_met_plus_min_plus(cel_midden_min: usize, cel_plus: u8) -> String {
        let eerste_plus = Self::plus_toevoegen_als_string(cel_plus);
        let tweede_min =
            Self::extra_min_streepjes_als_string(cel_midden_min, Self::min_toevoegen_als_lijn());
        let derde_plus = Self::plus_toevoegen_als_string(1);
        format!("{eerste_plus}{tweede_min}{derde_plus}")
    }
    /// Een teken reeks opmaak weer te geven aan een `boven` of `onder` zijde kant van `tabel` inhoud.
    pub fn regel_met_plus_laag_streep_plus(cel_midden_min: usize, cel_plus: u8) -> String {
        let eerste_plus = Self::plus_toevoegen_als_string(cel_plus);
        let tweede_laag_streep =
            Self::extra_min_streepjes_als_string(cel_midden_min, Self::laag_streep_als_lijn());
        let derde_plus = Self::plus_toevoegen_als_string(1);
        format!("{eerste_plus}{tweede_laag_streep}{derde_plus}")
    }
    fn plus_toevoegen_als_string(cel_plus: u8) -> String {
        let _even_of_on_even = true;
        if cel_plus == 1 {
            String::from("+")
        } else {
            String::from("")
        }
    }
    fn laag_streep_als_lijn() -> String {
        String::from("_")
    }
    fn min_toevoegen_als_lijn() -> String {
        String::from("-")
    }
    fn extra_min_streepjes_als_string(aantal_strepen: usize, lijn_soort: String) -> String {
        let mut s = "".to_string();
        let waarde = lijn_soort;
        for _nummer in 0..aantal_strepen {
            s.push_str(&waarde);
        }
        s
    }
}
