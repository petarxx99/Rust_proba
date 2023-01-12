use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};

use super::{lovac::{prirodno_kretanje_lovca, lovac_napada_polje}, top::{polja_na_koja_ide_top, top_napada_polje}};

pub fn prirodno_kretanje_kraljice<T>(
    tabla: &T,
    polje_na_kom_se_nalazim: u8,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: Option<u8>, ja_sam_beli: bool) -> Vec<u8>
    where T:Ima_podatke_o_tabli{
        let dijagonale: Vec<u8> = prirodno_kretanje_lovca(tabla, polje_na_kom_se_nalazim, rokada, fajl_pijuna_2_polja, ja_sam_beli);
        let mut kao_top: Vec<u8> = polja_na_koja_ide_top(tabla, polje_na_kom_se_nalazim, rokada, fajl_pijuna_2_polja, ja_sam_beli);
        
        for polje in dijagonale {
            kao_top.push(polje);
        }
        kao_top
    }



pub fn kraljica_napada_polje<T>(tabla: &T, polje_meta: u8, polje_kraljice: u8, ja_sam_beli: bool) -> bool 
where T:Ima_podatke_o_tabli{
    lovac_napada_polje(tabla, polje_meta, polje_kraljice, ja_sam_beli)
    ||
    top_napada_polje(tabla, polje_meta, polje_kraljice, ja_sam_beli)
}

pub fn kraljica_moze_doci_do_polja<T>(tabla: &T, polje_na_koje_dolazim: u8, moje_polje: u8, ja_sam_beli: bool) -> bool
    where T:Ima_podatke_o_tabli
    {
        kraljica_napada_polje(tabla, polje_na_koje_dolazim, moje_polje, ja_sam_beli)   
    }

#[cfg(test)]
pub mod test_kraljica{
    use crate::tabla::{Tabla, E_FILE, D_FILE, H_FILE, C_FILE, B_FILE, F_FILE, A_FILE, kretanje_figura::kraljica::kraljica_napada_polje, G_FILE};


    fn testiraj_kraljica_na_kralj_na(
        file_kraljice: u8, rank_kraljice: u8, file_kralja: u8, rank_kralja: u8, 
        kraljica_treba_da_napada_kralja: bool
    ){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(D_FILE, 1, file_kraljice, rank_kraljice)
        .odigraj_validan_potez_bez_promocije(E_FILE, 8, file_kralja, rank_kralja);
        
        let polje_kraljice: u8 = crate::file_rank_to_broj(file_kraljice, rank_kraljice);
        let polje_napada: u8 = crate::file_rank_to_broj(file_kralja, rank_kralja);
        assert_eq!(kraljica_treba_da_napada_kralja, kraljica_napada_polje(&tabla, polje_napada, polje_kraljice, true));
    }

    #[test]
    fn kraljica_sa_b3_napada_kralja_na_e6(){
        testiraj_kraljica_na_kralj_na(B_FILE, 3, E_FILE, 6, true);
    }

    #[test]
    fn kraljica_sa_h4_napada_kralja_na_f6(){
        testiraj_kraljica_na_kralj_na(H_FILE, 4, F_FILE, 6, true);
    }

    #[test]
    fn kraljica_sa_f5_napada_kralja_na_d3(){
        testiraj_kraljica_na_kralj_na(F_FILE, 5, D_FILE, 3, true);
    }

    #[test]
    fn kraljica_sa_a7_napada_kralja_na_e3(){
        testiraj_kraljica_na_kralj_na(A_FILE, 7, E_FILE, 3, true);
    }

    #[test]
    fn kraljica_sa_c3_napada_kralja_na_c6(){
        testiraj_kraljica_na_kralj_na(C_FILE, 3, C_FILE, 6, true);
    }

    #[test]
    fn kraljica_sa_h7_napada_kralja_na_h3(){
        testiraj_kraljica_na_kralj_na(H_FILE, 7, H_FILE, 3, true);
    }

    #[test]
    fn kraljica_sa_h5_napada_kralja_na_a5(){
        testiraj_kraljica_na_kralj_na(H_FILE, 5, A_FILE, 5, true);
    }

    #[test]
    fn kraljica_sa_a4_napada_kralja_na_h4(){
        testiraj_kraljica_na_kralj_na(A_FILE, 4, H_FILE, 4, true);
    }

    #[test]
    fn kraljica_sa_g5_ne_napada_kralja_na_b4(){
        testiraj_kraljica_na_kralj_na(G_FILE, 5, B_FILE, 4, false);
    }

    #[test]
    fn kraljica_sa_f4_ne_napada_kralja_na_d5(){
        testiraj_kraljica_na_kralj_na(F_FILE, 4, D_FILE, 5, false);
    }

    #[test]
    fn kraljica_sa_b8_ne_napada_kralja_na_g8_jer_ima_figura_izmedju(){
        testiraj_kraljica_na_kralj_na(B_FILE, 8, G_FILE, 8, false);
    }

    #[test]
    fn kraljica_sa_b1_ne_napada_kralja_na_d3_jer_ima_figura_izmedju(){
        testiraj_kraljica_na_kralj_na(B_FILE, 1, D_FILE, 3, false);
    }
}


