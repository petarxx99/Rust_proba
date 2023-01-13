use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};

use super::figure::abs;




pub fn prirodno_kretanje_pijuna<T>(
    tabla: &T,
    polje_na_kom_se_nalazim: u8,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: Option<u8>, ja_sam_beli: bool) -> Vec<u8>
    where T:Ima_podatke_o_tabli

    {
        let mut polja: Vec<u8> = Vec::new();
        let (rank, file) = crate::broj_to_rank_file(polje_na_kom_se_nalazim);
        let (pocetni_rank, napred_jedno_polje, en_passant_rank) = pocetni_rank_napred_jedno_polje_en_passant_rank(ja_sam_beli);
/* Pijun moze da ide jedno polje napred. */
        polja.push(crate::file_rank_to_broj(file, ((rank as i8) + napred_jedno_polje) as u8));

       /*  ako_nisam_pomerao_pijuna_mogu_ga_pomeriti_dvaput*/
        if rank == pocetni_rank {
            polja.push(crate::file_rank_to_broj(file, ((rank as i8) + 2 * napred_jedno_polje) as u8));
        }

/* Pijun jede ukoso, drugacije nego sto ide. Zato moram posebno da obradjujem ovaj slucaj. */
        let rank_ispred = ((rank as i8) + napred_jedno_polje) as u8;
        za_slucaj_da_pijun_moze_da_jede_drugu_figuru(&mut polja, tabla, ja_sam_beli, file, rank_ispred);

        if en_passant_rank == rank {
            probaj_da_dodas_en_passant(&mut polja, fajl_pijuna_2_polja, file, rank_ispred);
        }
        polja
    }

    fn probaj_da_dodas_en_passant(polja: &mut Vec<u8>, file_pijun_za_en_passant: Option<u8>, file: u8, rank_ispred: u8){
        match file_pijun_za_en_passant{
            None => {},
            Some(en_passant_file) => {
                if abs(file as i32 - en_passant_file as i32) == 1{
                    polja.push(crate::file_rank_to_broj(en_passant_file, rank_ispred));
                }
            }
        }
    }

    fn za_slucaj_da_pijun_moze_da_jede_drugu_figuru<T>(polja: &mut Vec<u8>, tabla: &T, ja_sam_beli: bool, file: u8, rank_ispred:u8)
    where T: Ima_podatke_o_tabli{

          if (file as i32) - 1 >= A_FILE as i32{
             if tabla.da_li_je_figura_boje_na_polju(!ja_sam_beli, rank_ispred, file-1){
                    polja.push(crate::file_rank_to_broj(file-1, rank_ispred));
            }    
        }

        if file + 1 <= H_FILE {
            if tabla.da_li_je_figura_boje_na_polju(!ja_sam_beli, rank_ispred, file+1){
                polja.push(crate::file_rank_to_broj(file+1, rank_ispred));
            }
        }
    }

    fn pocetni_rank_napred_jedno_polje_en_passant_rank(ja_sam_beli: bool) -> (u8, i8, u8) {
        let pocetni_rank: u8;
        let napred_jedno_polje: i8;
        let en_passant_rank: u8;
    
        if ja_sam_beli {
            pocetni_rank = 2;
            napred_jedno_polje = 1;
            en_passant_rank = 5;
        }  else {
            pocetni_rank = 7;
            napred_jedno_polje = -1;
            en_passant_rank = 4;
        }
        (pocetni_rank, napred_jedno_polje, en_passant_rank)
    }



pub fn pijun_napada_polje<T>(tabla: &T, polje_meta: u8, moje_polje: u8, ja_sam_beli: bool) -> bool 
where T:Ima_podatke_o_tabli{
    let (moj_rank, moj_file) = crate::broj_to_rank_file(moje_polje);
    let (rank_destinacije, file_destinacije) = crate::broj_to_rank_file(polje_meta);
   
    if abs(file_destinacije as i32 - moj_file as i32) != 1 {
        return false
    }
    
    if ja_sam_beli && (rank_destinacije as i8 - moj_rank as i8) == 1 {
        return true
    }
  
    moj_rank as i8 - rank_destinacije as i8 == 1
}


pub fn pijun_moze_doci_na_polje<T>(tabla: &T, polje_na_koje_dolazim: u8, moje_polje: u8, ja_sam_beli: bool) -> bool
    where T:Ima_podatke_o_tabli
    {
        let polja_prirodnog_kretanja: Vec<u8> =  prirodno_kretanje_pijuna(tabla,  moje_polje, &tabla.get_rokada(), tabla.get_file_pijuna_koji_se_pomerio_2_polja(), ja_sam_beli);
        if !polja_prirodnog_kretanja.contains(&polje_na_koje_dolazim){
            return false
        }  /* Sad obradjujem samo polja prirodnog kretanja. Ukoliko pijun ne moze nista da pojede ukoso, 
        to polje ne bi bilo polje prirodnog kretanja. Zato sledeca linija koda radi. */
        if pijun_napada_polje(tabla, polje_na_koje_dolazim, moje_polje, ja_sam_beli){
            return true;
        } /* Sad preostaju samo pravolinijska kretanja koja su prirodna kretanja pijuna. */

        let (rank_pijuna,file_pijuna) = crate::broj_to_rank_file(moje_polje);
        let (rank_destinacije, _) = crate::broj_to_rank_file(polje_na_koje_dolazim);

        if abs(rank_destinacije as i32 - rank_pijuna as i32) == 2 {
            let rank_polja_izmedju:u8;
            if rank_destinacije > rank_pijuna {
                rank_polja_izmedju = rank_pijuna + 1;
            } else {
                rank_polja_izmedju = rank_pijuna - 1;
            }

            let polje_izmedju = crate::file_rank_to_broj(file_pijuna, rank_polja_izmedju);
            tabla.da_li_su_polja_prazna(&vec![polje_izmedju, polje_na_koje_dolazim])
        } else {
            tabla.da_li_su_polja_prazna(&vec![polje_na_koje_dolazim])
        }

    }

#[cfg(test)]
pub mod test_pijun{
    use crate::tabla::{Tabla, E_FILE,A_FILE, B_FILE, C_FILE, D_FILE, F_FILE, G_FILE, H_FILE, Rokada, potez::Potez, Promocija};

    use super::{prirodno_kretanje_pijuna, pijun_moze_doci_na_polje};


    fn beli_pijun_napada_kralja(file_belog_pijuna: u8, rank_belog_pijuna: u8,
         file_kralja: u8, rank_kralja: u8, 
        pijun_napada_polje: bool){
            let tabla: Tabla = Tabla::pocetna_pozicija()
            .odigraj_validan_potez_bez_promocije(E_FILE, 2, file_belog_pijuna, rank_belog_pijuna)
            .odigraj_validan_potez_bez_promocije(E_FILE, 8, file_kralja, rank_kralja);
 
            let polje_pijuna: u8 = crate::file_rank_to_broj(file_belog_pijuna, rank_belog_pijuna);
            let polje_koje_napada: u8 = crate::file_rank_to_broj(file_kralja, rank_kralja);
            assert_eq!(
                pijun_napada_polje,
                 crate::tabla::kretanje_figura::pijun::pijun_napada_polje(&tabla, polje_koje_napada, polje_pijuna, true));
        }

        fn crni_pijun_napada_kralja(file_belog_pijuna: u8, rank_belog_pijuna: u8,
            file_kralja: u8, rank_kralja: u8, 
           pijun_napada_polje: bool){
               let tabla: Tabla = Tabla::pocetna_pozicija()
               .odigraj_validan_potez_bez_promocije(E_FILE, 1, file_kralja, rank_kralja)
               .odigraj_validan_potez_bez_promocije(E_FILE, 7, file_belog_pijuna, rank_belog_pijuna)
               ;
               
               let polje_pijuna: u8 = crate::file_rank_to_broj(file_belog_pijuna, rank_belog_pijuna);
               let polje_koje_napadam: u8 = crate::file_rank_to_broj(file_kralja, rank_kralja);
               assert_eq!(
                   pijun_napada_polje,
                    crate::tabla::kretanje_figura::pijun::pijun_napada_polje(&tabla, polje_koje_napadam, polje_pijuna, false));
        }

        

       #[test]
       fn beli_pijun_sa_c5_napada_kralja_na_b6(){
            beli_pijun_napada_kralja(C_FILE, 5, B_FILE, 6, true);
       }

       #[test]
       fn beli_pijun_sa_f7_ne_napada_kralja_na_h8(){
            beli_pijun_napada_kralja(F_FILE, 7, H_FILE, 8, false);
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

       fn broj_polja(start_file: u8, start_rank: u8, en_passant_file: Option<u8>, ja_sam_beli: bool) -> usize{
            let start_polje = crate::file_rank_to_broj(start_file, start_rank);
            prirodno_kretanje_pijuna(&Tabla::pocetna_pozicija(), start_polje, &Rokada::new_sve_rokade_moguce(), en_passant_file, ja_sam_beli).len()
       }

       #[test]
       fn beli_pijun_sa_e4_moze_na_1_polja(){
            assert_eq!(1, broj_polja(E_FILE, 4, None, true));
       }

       #[test]
       fn beli_pijun_sa_e2_moze_na_2_polja(){
            assert_eq!(2, broj_polja(E_FILE, 2, None, true));
       }

       #[test]
       fn beli_pijun_sa_g6_moze_na_3_polja(){
            assert_eq!(3, broj_polja(G_FILE, 6, None, true));
       }

       #[test]
       fn en_passant_beli_pijun_na_f5_moze_na_2_polja_jer_je_g5_odigran(){
            assert_eq!(2, broj_polja(F_FILE, 5, Some(G_FILE), true));
       }

       #[test]
       fn crni_moze_na_2_polja_zbog_en_passant(){
            assert_eq!(2, broj_polja(B_FILE, 4, Some(C_FILE), false));
       }

       #[test]
       fn crni_moze_sa_c3_na_3_polja(){
            assert_eq!(3, broj_polja(C_FILE, 3, None, false));
       }

       #[test]
       fn crni_moze_sa_g7_na_2_polja(){
            assert_eq!(2, broj_polja(G_FILE, 7, None, false));
       }

       #[test]
       fn en_passant_je_validan_potez_posle_d4_h5_d5_e5(){
            let tabla: Tabla = Tabla::pocetna_pozicija()
            .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 4)
            .odigraj_validan_potez_bez_promocije(H_FILE, 7, H_FILE, 5)
            .odigraj_validan_potez_bez_promocije(D_FILE, 4, D_FILE, 5)
            .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5);

            let en_passant_polje: u8 = crate::file_rank_to_broj(E_FILE, 6);
            let moje_polje: u8 = crate::file_rank_to_broj(D_FILE, 5);
            let en_passant_potez: Potez = Potez::new(D_FILE, 5, E_FILE, 6, Promocija::None);
            let polja_pijuna: Vec<u8> = prirodno_kretanje_pijuna(&tabla, moje_polje, &tabla.rokada(), tabla.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu(), tabla.beli_je_na_potezu());
            assert_eq!(E_FILE, tabla.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu().unwrap());
            assert_eq!(true, polja_pijuna.contains(&en_passant_polje));
            assert_eq!(true, pijun_moze_doci_na_polje(&tabla, en_passant_polje, moje_polje, tabla.beli_je_na_potezu()));
        }

}