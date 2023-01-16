use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};
use std::boxed::Box;

use super::figure::ako_su_validni_dodaj_u_vektor;


pub fn prirodno_kretanje_konja<T>(
    tabla: &T,
    polje_na_kom_se_nalazim: &File_rank,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: &Option<u8>, ja_sam_beli: bool) -> Vec<File_rank>
    where T:Ima_podatke_o_tabli{
 
        let rank: i32 = polje_na_kom_se_nalazim.rank as i32;
        let file: i32 = polje_na_kom_se_nalazim.file as i32;
        let mut polja: Vec<File_rank> = Vec::new();
        ako_su_validni_dodaj_u_vektor(&mut polja, rank+1, file+2);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank+1, file-2);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank-1, file+2);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank-1, file-2);

        ako_su_validni_dodaj_u_vektor(&mut polja, rank + 2, file+1);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank-2, file+1);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank+2, file-1);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank-2, file-1);
        polja
    }

    

    pub fn konj_napada_polje<T>(tabla: &T, polje_meta: &File_rank, moje_polje: &File_rank, ja_sam_beli: bool)->bool
    where T:Ima_podatke_o_tabli
    {
        let moj_rank: u8 = moje_polje.rank;
        let moj_file: u8 = moje_polje.file;
        let rank: u8 = polje_meta.rank;
        let file: u8 = polje_meta.file;


        if abs(rank as i32 - moj_rank as i32) == 2 &&
        abs(file as i32 - moj_file as i32) == 1{
            return true;
        }

        if abs(rank as i32 - moj_rank as i32) == 1 &&
        abs(file as i32 - moj_file as i32) == 2 {
            return true;
        }
        false
    }

    pub fn konj_moze_doci_na_polje<T>(tabla: &T, polje_na_koje_dolazim: &File_rank, moje_polje: &File_rank, ja_sam_beli: bool) -> bool
    where T:Ima_podatke_o_tabli
    {
        
        konj_napada_polje(tabla, polje_na_koje_dolazim, moje_polje, ja_sam_beli)   
        &&
        !tabla.da_li_je_figura_boje_na_polju(ja_sam_beli, polje_na_koje_dolazim.rank, polje_na_koje_dolazim.file)
    }

    fn abs(broj: i32) -> u32 {
        if broj<0 {
            return (-broj) as u32
        }
        broj as u32
    }

pub fn potezi_konja<T>  (
    tabla: &T,
    polje_na_kom_se_nalazim: &File_rank,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: &Option<u8>, ja_sam_beli: bool) -> Vec<File_rank>
    where T:Ima_podatke_o_tabli{
        let polja_prirodnog_kretanja: Vec<File_rank> = prirodno_kretanje_konja(tabla, polje_na_kom_se_nalazim, rokada, fajl_pijuna_2_polja, ja_sam_beli);
        let mut potezi_konja: Vec<File_rank> = Vec::new();
        for polje in polja_prirodnog_kretanja {
            if konj_moze_doci_na_polje(tabla, &polje, polje_na_kom_se_nalazim, ja_sam_beli){
                potezi_konja.push(polje);
            }
        }
        potezi_konja
}  

    #[cfg(test)]
    mod konj_test{
        use crate::tabla::{Tabla, Rokada, A_FILE, G_FILE, E_FILE, B_FILE, C_FILE, F_FILE, File_rank, kretanje_figura::konj::potezi_konja};

        use super::{prirodno_kretanje_konja, konj_napada_polje};

        fn gde_moze_konj(file: u8, rank: u8) -> Vec<File_rank> {
            prirodno_kretanje_konja(&Tabla::pocetna_pozicija(), &File_rank{file, rank}, &Rokada::new_sve_rokade_moguce(), &None, true)
        }

        #[test]
        fn konj_sa_A1_moze_na_2_polja(){
            let polja = gde_moze_konj(A_FILE, 1);
            assert_eq!(2, polja.len());
        }

        #[test]
        fn konj_sa_G7_moze_na_4_polja(){
            let polja = gde_moze_konj(G_FILE, 7);
            assert_eq!(4, polja.len());
        }

        #[test]
        fn konj_sa_E4_moze_na_8_polja(){
            let polja = gde_moze_konj(E_FILE, 4);
            assert_eq!(8, polja.len());
        }

        #[test]
        fn konj_sa_B4_moze_na_6_polja(){
            let polja = gde_moze_konj(B_FILE, 6);
            assert_eq!(6, polja.len());
        }



        /* Da li napada kralja. */
        fn beli_konj(file_konja: u8, rank_konja:u8, file_kralja: u8, rank_kralja: u8)->Tabla{
            let tabla0: Tabla = Tabla::pocetna_pozicija();
            let tabla: Tabla = tabla0.odigraj_validan_potez_bez_promocije(B_FILE, 1, file_konja, rank_konja);
            tabla.odigraj_validan_potez_bez_promocije(E_FILE, 8, file_kralja, rank_kralja)
        }
        fn crni_konj(file_konja: u8, rank_konja:u8, file_kralja: u8, rank_kralja: u8)->Tabla{
            let tabla0: Tabla = Tabla::pocetna_pozicija();
            let tabla1: Tabla = tabla0.odigraj_validan_potez_bez_promocije(A_FILE, 2, A_FILE, 2);
            let tabla2: Tabla = tabla1.odigraj_validan_potez_bez_promocije(G_FILE, 8, file_konja, rank_konja);
            tabla2.odigraj_validan_potez_bez_promocije(E_FILE, 1, file_kralja, rank_kralja)
        }

        fn test_konj_napada_polje(file_belog_konja: u8, rank_belog_konja: u8,
            file_crnog_kralja: u8, rank_crnog_kralja: u8, 
           napadam_polje: bool){
               let tabla: Tabla = Tabla::pocetna_pozicija()
               .odigraj_validan_potez_bez_promocije(E_FILE, 1, file_belog_konja, rank_belog_konja)
               .odigraj_validan_potez_bez_promocije(E_FILE, 8, file_crnog_kralja, rank_crnog_kralja);
               
               let polje_konja: File_rank = File_rank::new(file_belog_konja, rank_belog_konja);
               let polje_koje_napadam: File_rank = File_rank::new(file_crnog_kralja, rank_crnog_kralja);
               assert_eq!(
                   napadam_polje,
                    crate::tabla::kretanje_figura::konj::konj_napada_polje(&tabla, &polje_koje_napadam, &polje_konja, true));
           }

        #[test]
        fn konj_sa_B4_napada_kralja_na_C6(){
            test_konj_napada_polje(B_FILE, 4, C_FILE, 6, true);
        }

        #[test]
        fn konj_sa_F4_napada_kralja_na_E2(){
            test_konj_napada_polje(F_FILE, 4, E_FILE, 2, true);
        }

        #[test]
        fn beli_konj_sa_e5_ne_napada_kralja_na_g7(){
            test_konj_napada_polje(E_FILE, 5, G_FILE, 7, false);
        }

        #[test]
        fn test_nakon_e4_e5_Nf3_konj_ima_5_poteza(){
            let tabla: Tabla = Tabla::pocetna_pozicija()
            .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
            .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
            .odigraj_validan_potez_bez_promocije(G_FILE, 1, F_FILE, 3)
            .odigraj_validan_potez_bez_promocije(A_FILE, 7, A_FILE, 6);
            
            let potezi_konja: Vec<File_rank> = potezi_konja(&tabla, &File_rank::new(F_FILE, 3), &tabla.rokada(), &tabla.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu(), tabla.beli_je_na_potezu());
            assert_eq!(5, potezi_konja.len());
            
        }

    }