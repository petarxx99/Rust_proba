use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};


pub fn prirodno_kretanje_kralja(
    polje_na_kom_se_nalazim: u8,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: Option<u8>) -> Vec<u8>{
        Vec::new()
    }

pub fn kralj_napada_kralja<T>(tabla: &T, polje_kraljice: u8) -> bool 
where T:Ima_podatke_o_tabli{
    false
}



#[cfg(test)]
pub mod test_kralj{
    use crate::tabla::{Tabla, E_FILE,A_FILE, B_FILE, C_FILE, D_FILE, F_FILE, G_FILE, H_FILE};


    fn test_kralj_napada_kralja(file_belog_kralja: u8, rank_belog_kralja: u8,
         file_crnog_kralja: u8, rank_crnog_kralja: u8, 
        lovac_napada_kralja: bool){
            let tabla: Tabla = Tabla::pocetna_pozicija()
            .odigraj_validan_potez_bez_promocije(E_FILE, 1, file_belog_kralja, rank_belog_kralja)
            .odigraj_validan_potez_bez_promocije(E_FILE, 8, file_crnog_kralja, rank_crnog_kralja);
            
            let polje: u8 = Tabla::file_rank_to_broj(file_belog_kralja, rank_crnog_kralja);
            assert_eq!(
                lovac_napada_kralja,
                 crate::tabla::kretanje_figura::kralj::kralj_napada_kralja(&tabla, polje));
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
}