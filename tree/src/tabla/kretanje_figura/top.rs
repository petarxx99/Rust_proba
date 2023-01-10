use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};



pub fn polja_na_koja_ide_top(
    polje_na_kom_se_nalazim: u8,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: Option<u8>) -> Vec<u8>{
         Vec::new()
    }

fn top_napada_protivnickog_kralja<T>(tabla: &T, polje_na_kom_se_nalazim: u8) -> bool
where T: Ima_podatke_o_tabli
{
    false
}

#[cfg(test)]
mod top_test{
    use crate::tabla::{Tabla, E_FILE, A_FILE, G_FILE, Rokada, H_FILE, B_FILE};

    use super::{polja_na_koja_ide_top, top_napada_protivnickog_kralja};

    fn top_na_polje_kralj_na_polje(file_topa: u8, rank_topa: u8, file_kralja: u8, rank_kralja: u8)->Tabla{
        let tabla0 : Tabla = Tabla::pocetna_pozicija();
        let tabla1: Tabla = tabla0.odigraj_validan_potez_bez_promocije(A_FILE, 1, file_topa, rank_topa);
        tabla1.odigraj_validan_potez_bez_promocije(E_FILE, 8, file_kralja, rank_kralja)
    }

    #[test]
    fn top_sa_a4_vidi_15_polja(){
        let tabla: Tabla = top_na_polje_kralj_na_polje(A_FILE, 4, G_FILE, 8);
        assert_eq!(15, polja_na_koja_ide_top(Tabla::file_rank_to_broj(A_FILE, 4), &Rokada::new_sve_rokade_moguce(), None).len());

    }

    #[test]
    fn top_sa_h3_vidi_kralja_na_h3_kad_nema_nista_izmedju(){
        let tabla: Tabla = top_na_polje_kralj_na_polje(H_FILE, 3, H_FILE, 3);
        assert_eq!(true, top_napada_protivnickog_kralja(&tabla, Tabla::file_rank_to_broj(H_FILE, 3)));
    }

    #[test]
    fn top_sa_b8_ne_vidi_kralja_na_e8_jer_ima_figura_izmedju(){
        let tabla: Tabla = top_na_polje_kralj_na_polje(B_FILE, 8, E_FILE, 8);
        assert_eq!(false, top_napada_protivnickog_kralja(&tabla, Tabla::file_rank_to_broj(B_FILE, 8)));
    }

    #[test]
    fn top_sa_a3_napada_kralja_na_h3(){
        let tabla: Tabla = top_na_polje_kralj_na_polje(A_FILE, 3, H_FILE, 3);
        assert_eq!(true, top_napada_protivnickog_kralja(&tabla, Tabla::file_rank_to_broj(A_FILE, 3)));
    }

    #[test]
    fn top_sa_b4_ne_napada_kralja_na_g6(){
        let polje: u8 = Tabla::file_rank_to_broj(B_FILE, 4);
        let tabla: Tabla = top_na_polje_kralj_na_polje(B_FILE, 4, G_FILE, 6);
        assert_eq!(false, top_napada_protivnickog_kralja(&tabla, polje));
    }

}

