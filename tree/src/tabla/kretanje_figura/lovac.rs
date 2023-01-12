use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};

use super::figure::abs;

pub fn prirodno_kretanje_lovca<T>(
    tabla: &T,
    polje_na_kom_se_nalazim: u8,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: Option<u8>, ja_sam_beli: bool) -> Vec<u8>
    where T:Ima_podatke_o_tabli{
        
        let mut polja: Vec<u8> = Vec::new();
        let (rank_lovca_u8, file_lovca_u8) = crate::broj_to_rank_file(polje_na_kom_se_nalazim);
        let rank_lovca: i8 = rank_lovca_u8 as i8;
        let file_lovca: i8 = file_lovca_u8 as i8;
        
        let mut rank: i8 = rank_lovca as i8  - 1;
        let mut file_razlika: i8 = 1;

        while rank>=1 {
            if (file_lovca - file_razlika) >= A_FILE as i8 {
                polja.push(crate::file_rank_to_broj((file_lovca - file_razlika) as u8, rank as u8));
            } 
            if (file_lovca + file_razlika) <= H_FILE as i8{
                polja.push(crate::file_rank_to_broj((file_lovca + file_razlika) as u8, rank as u8));
            }
            rank -=1;
            file_razlika += 1;
        }

        file_razlika = 1;
        rank = 1+ rank_lovca as i8;
        while rank <=8 {
            if (file_lovca - file_razlika) >= A_FILE as i8{
                polja.push(crate::file_rank_to_broj((file_lovca - file_razlika) as u8, rank as u8));
            }
            if (file_lovca + file_razlika) <= H_FILE as i8{
                polja.push(crate::file_rank_to_broj((file_lovca + file_razlika) as u8, rank as u8));
            }
            rank += 1;
            file_razlika += 1;
        }

        polja
    }



pub fn lovac_napada_polje<T>(tabla: &T, polje: u8, polje_lovca: u8, ja_sam_beli: bool) -> bool 
where T:Ima_podatke_o_tabli{
    let (rank, file) = crate::broj_to_rank_file(polje);
    let (moj_rank, moj_file) = crate::broj_to_rank_file(polje_lovca);

    if rank == moj_rank || file == moj_file {
        return false
    }
    if abs(moj_file as i32 - file as i32) != abs(moj_rank as i32 - rank as i32){
        return false
    }

    let min_file: u8;
    let max_file: u8;
  
    if file < moj_file{
        min_file = file;
        max_file = moj_file;
    } else {
        max_file = file;
        min_file = moj_file;
    }
    
    let mut polja_izmedju: Vec<u8> = Vec::new();
/* y=kx+n, n=y-kx ---- k = (y1-y2) / (x1-x2) */
    let k: i8 = (rank as i8 - moj_rank as i8) / (file as i8 - moj_file as i8);
    /* n = y - kx */
    let n: i8 = rank as i8 - k * (file as i8);
    for i in (min_file+1)..max_file {
        let x: i8 = i as i8;
        let y = k*x + n;
        polja_izmedju.push(crate::file_rank_to_broj(i, y as u8));
    }

    tabla.da_li_su_polja_prazna(&polja_izmedju)
}

pub fn lovac_moze_doci_na_polje<T>(tabla: &T, moje_polje: u8, polje_na_koje_dolazim: u8, ja_sam_beli: bool) -> bool
    where T:Ima_podatke_o_tabli
    {
        lovac_napada_polje(tabla, polje_na_koje_dolazim, moje_polje, ja_sam_beli)   
    }

#[cfg(test)]
pub mod test_lovac{
    use crate::tabla::{Tabla, self, A_FILE, B_FILE, C_FILE, D_FILE, E_FILE, F_FILE, G_FILE, H_FILE,
        kretanje_figura::lovac::lovac_napada_polje, Rokada};
        
        fn test_kretanje_lovca(broj_polja: usize, file_lovca: u8, rank_lovca: u8){
            let polje: u8 = crate::file_rank_to_broj(file_lovca, rank_lovca);
            assert_eq!(broj_polja, 
                crate::tabla::kretanje_figura::lovac::prirodno_kretanje_lovca(&Tabla::pocetna_pozicija(), polje, &Rokada::new_sve_rokade_moguce(), None, true).len());
        }

        #[test]
        fn lovac_vidi_11_polja_sa_d3(){
            test_kretanje_lovca(11, D_FILE, 3);
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
        
        let polje_lovca: u8 = crate::file_rank_to_broj(file_lovca, rank_lovca);
        let polje_koje_napadam: u8 = crate::file_rank_to_broj(file_kralja, rank_kralja);
        assert_eq!(
            lovac_napada_kralja,
             crate::tabla::kretanje_figura::lovac::lovac_napada_polje(&tabla, polje_koje_napadam, polje_lovca, true));
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
