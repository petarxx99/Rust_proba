use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};




pub fn prirodno_kretanje_pijuna(
    polje_na_kom_se_nalazim: u8,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: Option<u8>) -> Vec<u8>{
        Vec::new()
    }

pub fn pijun_napada_kralja<T>(tabla: &T, polje_pijuna: u8) -> bool 
where T:Ima_podatke_o_tabli{
    false
}



#[cfg(test)]
pub mod test_pijun{
    use crate::tabla::{Tabla, E_FILE,A_FILE, B_FILE, C_FILE, D_FILE, F_FILE, G_FILE, H_FILE};


    fn beli_pijun_napada_kralja(file_belog_pijuna: u8, rank_belog_pijuna: u8,
         file_kralja: u8, rank_kralja: u8, 
        pijun_napada_kralja: bool){
            let tabla: Tabla = Tabla::pocetna_pozicija()
            .odigraj_validan_potez_bez_promocije(E_FILE, 2, file_belog_pijuna, rank_belog_pijuna)
            .odigraj_validan_potez_bez_promocije(E_FILE, 8, file_kralja, rank_kralja);
            
            let polje: u8 = Tabla::file_rank_to_broj(file_belog_pijuna, rank_kralja);
            assert_eq!(
                pijun_napada_kralja,
                 crate::tabla::kretanje_figura::pijun::pijun_napada_kralja(&tabla, polje));
        }

        fn crni_pijun_napada_kralja(file_belog_pijuna: u8, rank_belog_pijuna: u8,
            file_kralja: u8, rank_kralja: u8, 
           pijun_napada_kralja: bool){
               let tabla: Tabla = Tabla::pocetna_pozicija()
               .odigraj_validan_potez_bez_promocije(E_FILE, 1, file_kralja, rank_kralja)
               .odigraj_validan_potez_bez_promocije(E_FILE, 7, file_belog_pijuna, rank_belog_pijuna)
               ;
               
               let polje: u8 = Tabla::file_rank_to_broj(file_belog_pijuna, rank_kralja);
               assert_eq!(
                   pijun_napada_kralja,
                    crate::tabla::kretanje_figura::pijun::pijun_napada_kralja(&tabla, polje));
        }


       #[test]
       fn beli_pijun_sa_c5_napada_kralja_na_b6(){
            beli_pijun_napada_kralja(C_FILE, 5, B_FILE, 6, true);
       }

       #[test]
       fn beli_pijun_sa_f7_napada_kralja_na_h8(){
            beli_pijun_napada_kralja(F_FILE, 7, H_FILE, 8, true);
       }

       #[test]
       fn beli_pijun_sa_e4_ne_napada_kralja_na_e5(){
            beli_pijun_napada_kralja(E_FILE, 4, E_FILE, 5, false);
       }

       #[test]
       fn beli_pijun_sa_c5_ne_napada_kralja_na_e7(){
            beli_pijun_napada_kralja(C_FILE, 5, E_FILE, 7, false);
       }

       #[test]
       fn crni_pijun_sa_e3_napada_kralja_na_d2(){
            crni_pijun_napada_kralja(E_FILE, 3, D_FILE, 2, true);
       }

       #[test]
       fn crni_pijun_sa_f2_napada_kralja_na_g1(){
            crni_pijun_napada_kralja(F_FILE, 2, G_FILE, 1, true);
       }

       #[test]
       fn crni_pijun_sa_b5_ne_napada_kralja_na_b4(){
            crni_pijun_napada_kralja(B_FILE, 5, B_FILE, 4, false);
       }

       #[test]
       fn crni_pijun_sa_f4_ne_napada_kralja_na_d2(){
            crni_pijun_napada_kralja(F_FILE, 4, D_FILE, 2, false);
       }

}