use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE};
use std::boxed::Box;


pub fn prirodno_kretanje_konja(
    polje_na_kom_se_nalazim: u8,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: Option<u8>) -> Vec<u8>{

        let (rank_u8, file_u8) = Tabla::broj_to_rank_file(polje_na_kom_se_nalazim);
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
            vektor.push(Tabla::file_rank_to_broj(file as u8, rank as u8));
         }
    }

    #[cfg(test)]
    mod konj_test{
        use crate::tabla::{Tabla, Rokada, A_FILE, G_FILE, E_FILE, B_FILE};

        use super::prirodno_kretanje_konja;

        fn gde_moze_konj(file: u8, rank: u8) -> Vec<u8> {
            prirodno_kretanje_konja(Tabla::file_rank_to_broj(file, rank), &Rokada::new_sve_rokade_moguce(), None)
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
    }