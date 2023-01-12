use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};
use std::boxed::Box;


pub fn prirodno_kretanje_konja<T>(
    tabla: &T,
    polje_na_kom_se_nalazim: u8,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: Option<u8>, ja_sam_beli: bool) -> Vec<u8>
    where T:Ima_podatke_o_tabli{

        let (rank_u8, file_u8) = crate::broj_to_rank_file(polje_na_kom_se_nalazim);
        let rank: i32 = rank_u8 as i32;
        let file: i32 = file_u8 as i32;
        let mut polja: Vec<u8> = Vec::new();
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

    


    fn ako_su_validni_dodaj_u_vektor(vektor: &mut Vec<u8>, rank: i32, file: i32){
         if rank >= 1 && rank <=8 && file>=A_FILE as i32 && file <=H_FILE as i32{
            vektor.push(crate::file_rank_to_broj(file as u8, rank as u8));
         }
    }

    pub fn konj_napada_polje<T>(tabla: &T, polje: u8, polje_na_kom_se_nalazim: u8, ja_sam_beli: bool)->bool
    where T:Ima_podatke_o_tabli
    {
        let (rank, file) = crate::broj_to_rank_file(polje);
        let (moj_rank, moj_file) = crate::broj_to_rank_file(polje_na_kom_se_nalazim);

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

    pub fn konj_moze_doci_na_polje<T>(tabla: &T, moje_polje: u8, polje_na_koje_dolazim: u8, ja_sam_beli: bool) -> bool
    where T:Ima_podatke_o_tabli
    {
        konj_napada_polje(tabla, polje_na_koje_dolazim, moje_polje, ja_sam_beli)   
    }

    fn abs(broj: i32) -> u32 {
        if broj<0 {
            return (-broj) as u32
        }
        broj as u32
    }

    #[cfg(test)]
    mod konj_test{
        use crate::tabla::{Tabla, Rokada, A_FILE, G_FILE, E_FILE, B_FILE, C_FILE, F_FILE};

        use super::{prirodno_kretanje_konja, konj_napada_polje};

        fn gde_moze_konj(file: u8, rank: u8) -> Vec<u8> {
            prirodno_kretanje_konja(&Tabla::pocetna_pozicija(), crate::file_rank_to_broj(file, rank), &Rokada::new_sve_rokade_moguce(), None, true)
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
               
               let polje_konja: u8 = crate::file_rank_to_broj(file_belog_konja, rank_belog_konja);
               let polje_koje_napadam: u8 = crate::file_rank_to_broj(file_crnog_kralja, rank_crnog_kralja);
               assert_eq!(
                   napadam_polje,
                    crate::tabla::kretanje_figura::konj::konj_napada_polje(&tabla, polje_koje_napadam, polje_konja, true));
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
    }