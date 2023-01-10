use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};

use super::figure::abs;
use super::figure::ako_su_validni_dodaj_u_vektor;

pub fn prirodno_kretanje_kralja<T>(
    tabla: &T,
    polje_na_kom_se_nalazim: u8,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: Option<u8>, ja_sam_beli: bool) -> Vec<u8>
    where T:Ima_podatke_o_tabli{
        let (rank_u8, file_u8) = Tabla::broj_to_rank_file(polje_na_kom_se_nalazim);
        let rank: i32 = rank_u8 as i32;
        let file: i32 = file_u8 as i32;

        let mut polja: Vec<u8> = Vec::new();
        ako_su_validni_dodaj_u_vektor(&mut polja, rank, file-1);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank-1, file-1);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank-1, file);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank-1, file+1);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank, file+1);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank+1, file+1);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank, file+1);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank-1, file+1);
        polja
    }

pub fn kralj_napada_kralja<T>(tabla: &T, polje_kralja: u8, kralj_je_beli: bool) -> bool 
where T:Ima_podatke_o_tabli{
    kralj_napada_polje(tabla.pozicija_kralja(kralj_je_beli), tabla, polje_kralja, !kralj_je_beli)
}



pub fn kralj_napada_polje<T>(polje: u8, tabla: &T, polje_kralja: u8, ja_sam_beli: bool) -> bool 
where T:Ima_podatke_o_tabli{
    let (rank, file) = Tabla::broj_to_rank_file(polje);
    let (moj_rank, moj_file) = Tabla::broj_to_rank_file(polje_kralja);

    if moj_rank == rank && abs(file as i32 - moj_file as i32) == 1 {
        return true;
    }
    if abs(moj_rank as i32 - rank as i32) == 1 && abs(moj_file as i32 - file as i32) == 1{
        return true;
    }
    moj_file == file && abs(rank as i32 - moj_rank as i32) == 1
}



#[cfg(test)]
pub mod test_kralj{
    use crate::tabla::{Tabla, E_FILE,A_FILE, B_FILE, C_FILE, D_FILE, F_FILE, G_FILE, H_FILE, Rokada};

    use super::prirodno_kretanje_kralja;


    fn test_kralj_napada_kralja(file_belog_kralja: u8, rank_belog_kralja: u8,
         file_crnog_kralja: u8, rank_crnog_kralja: u8, 
        lovac_napada_kralja: bool){
            let tabla: Tabla = Tabla::pocetna_pozicija()
            .odigraj_validan_potez_bez_promocije(E_FILE, 1, file_belog_kralja, rank_belog_kralja)
            .odigraj_validan_potez_bez_promocije(E_FILE, 8, file_crnog_kralja, rank_crnog_kralja);
            
            let polje: u8 = Tabla::file_rank_to_broj(file_belog_kralja, rank_crnog_kralja);
            assert_eq!(
                lovac_napada_kralja,
                 crate::tabla::kretanje_figura::kralj::kralj_napada_kralja(&tabla, polje, false));
        }

        #[test]
        fn na_e4_e5(){
            test_kralj_napada_kralja(E_FILE, 4, E_FILE, 5, true);
        }

        #[test]
        fn na_b4_c4(){
            test_kralj_napada_kralja(B_FILE, 4, C_FILE, 4, true);
        }

        #[test]
        fn na_g6_g5(){
            test_kralj_napada_kralja(G_FILE, 6, G_FILE, 5, true);
        }

        #[test]
        fn na_c6_b5(){
            test_kralj_napada_kralja(C_FILE, 6, B_FILE, 5, true);
        }

        #[test]
        fn na_d3_f4(){
            test_kralj_napada_kralja(D_FILE, 3, F_FILE, 4, true);
        }

        #[test]
        fn na_f4_e5(){
            test_kralj_napada_kralja(F_FILE, 4, E_FILE, 5, true);
        }

        #[test]
        fn na_c5_e4(){
            test_kralj_napada_kralja(C_FILE, 5, E_FILE, 4, true);
        }

        #[test]
        fn na_b3_b5_ne_napada(){
            test_kralj_napada_kralja(B_FILE, 3, B_FILE, 5, false);
        }

        #[test]
        fn na_c5_e6_ne_napada(){
            test_kralj_napada_kralja(C_FILE, 5, E_FILE, 6, false);
        }

        fn obe_rokade_mogu() -> Tabla{
             Tabla::pocetna_pozicija().odigraj_validan_potez_bez_promocije (F_FILE, 1, F_FILE, 3)
            .odigraj_validan_potez_bez_promocije(F_FILE, 8, F_FILE, 6)
            .odigraj_validan_potez_bez_promocije (G_FILE, 1, G_FILE, 8)
            .odigraj_validan_potez_bez_promocije(G_FILE, 8, G_FILE, 6)
        }

        fn broj_kraljevih_poteza(file: u8, rank: u8, rokada: &Rokada, ja_sam_beli: bool) -> usize {
            prirodno_kretanje_kralja(&Tabla::pocetna_pozicija(),Tabla::file_rank_to_broj(file, rank), rokada, None, ja_sam_beli).len()
        }

        #[test]
        fn oba_kralja_mogu_na_po_4_polja_zbog_rokada(){
            let tabla: Tabla = obe_rokade_mogu();
            assert_eq!(4, broj_kraljevih_poteza(E_FILE, 1, &tabla.rokada(), true));
            let tabla_2: Tabla = tabla.odigraj_validan_potez_bez_promocije(E_FILE, 1,G_FILE, 1);
            assert_eq!(true, tabla_2.rokada().bela_kraljeva_rokada_vise_nije_moguca);
            assert_eq!(4, broj_kraljevih_poteza(E_FILE, 8, &tabla_2.rokada(), tabla_2.beli_je_na_potezu()));
            let tabla_nakon_obe_rokade: Tabla = tabla_2.odigraj_validan_potez_bez_promocije(E_FILE, 8, D_FILE, 8);
            assert_eq!(true, tabla_nakon_obe_rokade.rokada().crna_kraljicina_rokada_vise_nije_moguca);
        }
}