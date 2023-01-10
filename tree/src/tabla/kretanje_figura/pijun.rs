use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};




pub fn prirodno_kretanje_pijuna(
    polje_na_kom_se_nalazim: u8,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: Option<u8>) -> Vec<u8>{
        Vec::new()
    }

pub fn pijun_napada_kralja<T>(tabla: &T, polje_pijunice: u8) -> bool 
where T:Ima_podatke_o_tabli{
    false
}



#[cfg(test)]
pub mod test_pijun{
    use crate::tabla::{Tabla, E_FILE,A_FILE, B_FILE, C_FILE, D_FILE, F_FILE, G_FILE, H_FILE};


    fn test_pijun_napada_kralja(file_belog_pijuna: u8, rank_belog_pijuna: u8,
         file_kralja: u8, rank_kralja: u8, 
        lovac_napada_pijuna: bool){
            let tabla: Tabla = Tabla::pocetna_pozicija()
            .odigraj_validan_potez_bez_promocije(E_FILE, 2, file_belog_pijuna, rank_belog_pijuna)
            .odigraj_validan_potez_bez_promocije(E_FILE, 8, file_kralja, rank_kralja);
            
            let polje: u8 = Tabla::file_rank_to_broj(file_belog_pijuna, rank_kralja);
            assert_eq!(
                lovac_napada_pijuna,
                 crate::tabla::kretanje_figura::pijun::pijun_napada_kralja(&tabla, polje));
        }
        
}