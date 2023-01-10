use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};

pub fn prirodno_kretanje_kraljice(
    polje_na_kom_se_nalazim: u8,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: Option<u8>, ja_sam_beli: bool) -> Vec<u8>{
        Vec::new()
    }

pub fn kraljica_napada_kralja<T>(tabla: &T, polje_kraljice: u8, kralj_je_beli: bool) -> bool 
where T:Ima_podatke_o_tabli{
    false
}

pub fn kraljica_napada_polje<T>(polje: u8, tabla: &T, polje_kraljice: u8, ja_sam_beli: bool) -> bool 
where T:Ima_podatke_o_tabli{
    false
}



#[cfg(test)]
pub mod test_kraljica{
    use crate::tabla::{Tabla, E_FILE, D_FILE, H_FILE, C_FILE, B_FILE, F_FILE, A_FILE, kretanje_figura::kraljica::kraljica_napada_kralja, G_FILE};


    fn testiraj_kraljica_na_kralj_na(
        file_kraljice: u8, rank_kraljice: u8, file_kralja: u8, rank_kralja: u8, 
        kraljica_treba_da_napada_kralja: bool
    ){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(D_FILE, 1, file_kraljice, rank_kraljice)
        .odigraj_validan_potez_bez_promocije(E_FILE, 8, file_kralja, rank_kralja);
        
        let polje: u8 = Tabla::file_rank_to_broj(file_kraljice, rank_kraljice);
        assert_eq!(kraljica_treba_da_napada_kralja, kraljica_napada_kralja(&tabla, polje, false));
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


