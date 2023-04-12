mod tabel;
use crate::tabel::lijn::TabelLijn;
use crate::tabel::opmaak::Opmaak;
use crate::tabel::spaties::Spaties;

struct BacklogExampleSubjects {
    // Message reads:
    // New ideas from source: https://rust-lang.zulipchat.com/#narrow/stream/346005-t-style to be implemented.
    // happy fun times: https://rust-lang.zulipchat.com/#narrow/stream/268952-edition-2021
    aim: String,
    description: String,
    parent_goal: String,
}
impl BacklogExampleSubjects{
    fn drie_kollomen(){
        // to setup
        todo!()
    }
}
fn main() {
    let titels = BacklogExampleSubjects {
        aim: "Aim".to_string(),
        description: "Description".to_string(),
        parent_goal: "Parent Goal".to_string(),
    };
    // Limited version, of a lot more content, a table.
    let some_table_content_qeastions = "line-tables-only";
    let some_subject_content_no_idea = "-C debuginfo=1";
    let some_table_content_qeastions_year = 2024;
    let empty_spaces = Spaties::nummer_naar_spatie(4);
    let teken_spaties_style_one = Spaties {
        er_voor: 1,
        er_na: 1,
    };
    let teken_spaties_style_two = Spaties {
        er_voor: 1,
        er_na: 3,
    };
    let teken_opmaak = Opmaak::Streep;
    let lijn_kunst_boven_kolom_1 = TabelLijn::regel_met_plus_laag_streep_plus(18, 1);
    let lijn_kunst_boven_kolom_2 = TabelLijn::regel_met_plus_laag_streep_plus(6, 0);


    let lijn_kunst_boven_binnen_kolom_1_onder =
        tabel::Tabel::tabel_cel_staande_streep_bijde_zijkanten_woord(
            &teken_spaties_style_one,
            some_table_content_qeastions,
            &teken_opmaak,
        );
    let lijn_kunst_boven_binnen_kolom_2_onder =
        tabel::Tabel::tabel_cel_staande_streep_rechter_zijkant_nummer(
            &teken_spaties_style_one,
            &some_table_content_qeastions_year,
            &teken_opmaak,
        );
    let lijn_kunst_boven_binnen_kolom_3_onder =
        tabel::Tabel::tabel_cel_staande_streep_bijde_zijkanten_woord(
            &teken_spaties_style_two,
            some_subject_content_no_idea,
            &teken_opmaak,
        );
    let lijn_kunst_boven_binnen_kolom_4_onder =
        tabel::Tabel::tabel_cel_staande_streep_rechter_zijkant_woord(
            &teken_spaties_style_one,
            &empty_spaces,
            &teken_opmaak,
        );
    let lijn_kunst_beneden_kolom_1 = TabelLijn::regel_met_plus_min_plus(18, 1);
    let lijn_kunst_beneden_kolom_2 = TabelLijn::regel_met_plus_min_plus(6, 0);
    println!(
        "{}{}\n{}{}\n{}\n{}{}\n{}{}",
        lijn_kunst_boven_kolom_1,
        lijn_kunst_boven_kolom_2,
        lijn_kunst_boven_binnen_kolom_1_onder,
        lijn_kunst_boven_binnen_kolom_2_onder,
        lijn_kunst_beneden_kolom_1,
        lijn_kunst_boven_binnen_kolom_3_onder,
        lijn_kunst_boven_binnen_kolom_4_onder,
        lijn_kunst_beneden_kolom_1,
        lijn_kunst_beneden_kolom_2
    );
}
