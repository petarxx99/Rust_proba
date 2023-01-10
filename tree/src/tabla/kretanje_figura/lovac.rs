use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};

pub fn prirodno_kretanje_lovca(
    polje_na_kom_se_nalazim: u8,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: Option<u8>) -> Vec<u8>{
        Vec::new()
    }

pub fn lovac_napada_kralja<T>(tabla: &T, polje_lovca: u8) -> bool 
where T:Ima_podatke_o_tabli{
    false
}

#[cfg(test)]
pub mod test_lovac{
    use crate::tabla::{Tabla, self, A_FILE, B_FILE, C_FILE, D_FILE, E_FILE, F_FILE, G_FILE, H_FILE,
        kretanje_figura::lovac::lovac_napada_kralja, Rokada};
        
        fn test_kretanje_lovca(broj_polja: usize, file_lovca: u8, rank_lovca: u8){
            let polje: u8 = Tabla::file_rank_to_broj(file_lovca, rank_lovca);
            assert_eq!(broj_polja, 
                crate::tabla::kretanje_figura::lovac::prirodno_kretanje_lovca(polje, &Rokada::new_sve_rokade_moguce(), None).len());
        }

        #[test]
        fn lovac_vidi_12_polja_sa_d3(){
            test_kretanje_lovca(12, D_FILE, 3);
        }

        #[test]
        fn lovac_vidi_9_polja_sa_g2(){
            test_kretanje_lovca(9, G_FILE, 2);
        }

        #[test]
        fn lovac_vidi_7_polja_sa_c8(){
            test_kretanje_lovca(7, C_FILE, 8);
        }

    fn test_lovac_na_kralj_na(file_lovca: u8, rank_lovca: u8, file_kralja: u8, rank_kralja: u8, 
    lovac_napada_kralja: bool){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(C_FILE, 1, file_lovca, rank_lovca)
        .odigraj_validan_potez_bez_promocije(E_FILE, 8, file_kralja, rank_kralja);
        
        let polje: u8 = Tabla::file_rank_to_broj(file_lovca, rank_lovca);
        assert_eq!(
            lovac_napada_kralja,
             crate::tabla::kretanje_figura::lovac::lovac_napada_kralja(&tabla, polje));
    }

    #[test]
    fn test_lovac_sa_a3_napada_kralja_na_d6(){
        test_lovac_na_kralj_na(A_FILE, 3, D_FILE, 6, true);
    }

    #[test]
    fn test_lovac_sa_h6_napada_kralja_na_e3(){
        test_lovac_na_kralj_na(H_FILE, 6, E_FILE, 3, true);
    }

    #[test]
    fn test_lovac_sa_g3_napada_kralja_na_d6(){
        test_lovac_na_kralj_na(G_FILE, 3, D_FILE, 6, true);
    }

    #[test]
    fn test_lovac_sa_c6_napada_kralja_na_f3(){
        test_lovac_na_kralj_na(C_FILE, 6, F_FILE, 3,true);
    }

    #[test]
    fn test_lovac_sa_c1_ne_napada_kralja_na_e3_jer_ima_figura_izmedju(){
        test_lovac_na_kralj_na(C_FILE, 1, E_FILE, 3, false);
    }

    #[test]
    fn test_lovac_sa_f1_ne_napada_kralja_na_d3_jer_ima_figura_izmedju(){
        test_lovac_na_kralj_na(F_FILE, 1, D_FILE, 3, false);
    }

    #[test]
    fn test_lovac_sa_h8_ne_napada_kralja_na_f6_jer_ima_figura_izmedju(){
        test_lovac_na_kralj_na(H_FILE, 8, F_FILE, 6, false);
    }

    #[test]
    fn test_lovac_sa_b8_ne_napada_kralja_na_d6_jer_ima_figura_izmedju(){
        test_lovac_na_kralj_na(B_FILE, 8, D_FILE, 6, false);
    }

    #[test]
    fn test_lovac_sa_f3_ne_napada_kralja_na_d6(){
        test_lovac_na_kralj_na(F_FILE, 3, D_FILE, 6, false);
    }

    #[test]
    fn test_lovac_na_b4_ne_napada_kralja_na_c4(){
        test_lovac_na_kralj_na(B_FILE, 4, C_FILE, 4, false);
    }

    #[test]
    fn test_lovac_sa_c3_ne_napada_kralja_na_c4(){
        test_lovac_na_kralj_na(C_FILE, 3, C_FILE, 4, false);
    }
}
