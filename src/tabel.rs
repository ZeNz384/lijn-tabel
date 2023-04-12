pub mod lijn;
pub mod opmaak;
pub mod spaties;

use crate::tabel::opmaak::Opmaak;
use crate::tabel::spaties::Spaties;

pub struct Tabel;
impl Tabel {
    pub fn tabel_cel_staande_streep_bijde_zijkanten_woord(
        spaties: &Spaties,
        woord: &str,
        opmaak: &Opmaak,
    ) -> String {
        let er_voor = Spaties::nummer_naar_spatie(spaties.er_voor);
        let er_na = Spaties::nummer_naar_spatie(spaties.er_na);
        let inhoud_eerste_tabel_cel = spaties.woord(&er_voor, woord, &er_na);
        let begin = opmaak.streep();
        let einde = opmaak.streep();
        format!("{begin}{inhoud_eerste_tabel_cel}{einde}")
    }
    pub fn tabel_cel_staande_streep_rechter_zijkant_woord(
        spaties: &Spaties,
        woord: &str,
        opmaak: &Opmaak,
    ) -> String {
        let er_voor = Spaties::nummer_naar_spatie(spaties.er_voor);
        let er_na = Spaties::nummer_naar_spatie(spaties.er_na);
        let na_eerste_tabel_cel = spaties.woord(&er_voor, woord, &er_na);
        format!("{na_eerste_tabel_cel}{}", opmaak.streep())
    }
    pub fn tabel_cel_staande_streep_rechter_zijkant_nummer(
        spaties: &Spaties,
        nummer: &u32,
        opmaak: &Opmaak,
    ) -> String {
        let er_voor = Spaties::nummer_naar_spatie(spaties.er_voor);
        let er_na = Spaties::nummer_naar_spatie(spaties.er_na);
        let na_eerste_tabel_cel = spaties.nummer(&er_voor, nummer, &er_na);
        format!("{na_eerste_tabel_cel}{}", opmaak.streep())
    }
}
