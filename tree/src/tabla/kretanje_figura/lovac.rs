use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};

use super::figure::abs;

pub fn prirodno_kretanje_lovca<T>(
    tabla: &T,
    polje_na_kom_se_nalazim: &File_rank,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: &Option<u8>, ja_sam_beli: bool) -> Vec<File_rank>
    where T:Ima_podatke_o_tabli{
        let mut polja: Vec<File_rank> = Vec::new();
        let rank_lovca: i8 = polje_na_kom_se_nalazim.rank as i8;
        let file_lovca: i8 = polje_na_kom_se_nalazim.file as i8;
        
        let mut rank: i8 = rank_lovca as i8  - 1;
        let mut file_razlika: i8 = 1;

        while rank>=1 {
            if (file_lovca - file_razlika) >= A_FILE as i8 {
                polja.push(File_rank{file:(file_lovca - file_razlika) as u8, rank: rank as u8});
            } 
            if (file_lovca + file_razlika) <= H_FILE as i8{
                polja.push(File_rank{file: (file_lovca + file_razlika) as u8, rank: rank as u8});
            }
            rank -=1;
            file_razlika += 1;
        }

        file_razlika = 1;
        rank = 1+ rank_lovca as i8;
        while rank <=8 {
            if (file_lovca - file_razlika) >= A_FILE as i8{
                polja.push(File_rank{file: (file_lovca - file_razlika) as u8, rank: rank as u8});
            }
            if (file_lovca + file_razlika) <= H_FILE as i8{
                polja.push(File_rank{file: (file_lovca + file_razlika) as u8, rank: rank as u8});
            }
            rank += 1;
            file_razlika += 1;
        }

        polja
    }



pub fn lovac_napada_polje<T>(tabla: &T, polje_meta: &File_rank, moje_polje: &File_rank, ja_sam_beli: bool) -> bool 
where T:Ima_podatke_o_tabli{
    let rank: u8 = polje_meta.rank;
    let file: u8 = polje_meta.file;
    let moj_rank: u8 = moje_polje.rank;
    let moj_file: u8 = moje_polje.file;

    if rank == moj_rank || file == moj_file {
        return false
    }
    if abs(moj_file as i32 - file as i32) != abs(moj_rank as i32 - rank as i32){
        return false
    }

    let (min_file, max_file) = crate::min_max_broj(file, moj_file);
    let mut polja_izmedju: Vec<File_rank> = Vec::new();
    
/* y=kx+n, n=y-kx ---- k = (y1-y2) / (x1-x2) */
    let k: i8 = (rank as i8 - moj_rank as i8) / (file as i8 - moj_file as i8);
    /*     n = y - kx */
    let n: i8 = rank as i8 - k * (file as i8);
    for i in (min_file+1)..max_file {
        let x: i8 = i as i8; 
        let y = k*x + n;
        polja_izmedju.push(File_rank{file: i, rank: y as u8});
    }

    tabla.da_li_su_polja_prazna(&polja_izmedju)
}

pub fn lovac_moze_doci_na_polje<T>(tabla: &T, polje_na_koje_dolazim: &File_rank, moje_polje: &File_rank, ja_sam_beli: bool) -> bool
    where T:Ima_podatke_o_tabli
    {
        lovac_napada_polje(tabla, polje_na_koje_dolazim, moje_polje, ja_sam_beli)  
        &&
        !tabla.da_li_je_figura_boje_na_polju(ja_sam_beli, polje_na_koje_dolazim.rank, polje_na_koje_dolazim.file)

    }

pub fn potezi_lovca<T>(tabla: &T,
polje_na_kom_se_nalazim: &File_rank,
rokada: &Rokada, 
fajl_pijuna_2_polja: &Option<u8>, ja_sam_beli: bool) -> Vec<File_rank>
where T:Ima_podatke_o_tabli 

{
    let mut polja: Vec<File_rank> = Vec::new();
    potez_lovca_uzbrdo(tabla, &mut polja, polje_na_kom_se_nalazim.file, polje_na_kom_se_nalazim.rank, ja_sam_beli);
    potez_lovca_nizbrdo(tabla, &mut polja, polje_na_kom_se_nalazim.file, polje_na_kom_se_nalazim.rank, ja_sam_beli);
    polja
}

pub fn potez_lovca_uzbrdo<T>(tabla: &T, polja: &mut Vec<File_rank>, file_lovca: u8, rank_lovca: u8, ja_sam_beli: bool)
where T:Ima_podatke_o_tabli{
    let mut file = file_lovca + 1;
    let mut rank: u8 = rank_lovca + 1;
    while file <= H_FILE && rank <= 8{
        if tabla.da_li_je_figura_boje_na_polju(ja_sam_beli, rank, file){
            break;
        }
        polja.push(File_rank{file,rank});
        if !tabla.da_li_je_polje_prazno(&File_rank{file,rank}){
            break;
        }
        file += 1;
        rank += 1;
    }

    file = file_lovca - 1;
    rank = rank_lovca - 1;
    while file >= A_FILE && rank >= 1 {
        if tabla.da_li_je_figura_boje_na_polju(ja_sam_beli, rank, file){
            break;
        }
        polja.push(File_rank{file,rank});
        if !tabla.da_li_je_polje_prazno(&File_rank{file,rank}){
            break;
        }
        rank -= 1;
        file -= 1;
    }

}

pub fn potez_lovca_nizbrdo<T>(tabla: &T, polja: &mut Vec<File_rank>, file_lovca: u8, rank_lovca: u8, ja_sam_beli: bool)
where T:Ima_podatke_o_tabli{
    let mut file = file_lovca + 1;
    let mut rank = rank_lovca - 1;
    while file <= H_FILE && rank >= 1 {
        if tabla.da_li_je_figura_boje_na_polju(ja_sam_beli, rank, file){
            break;
        }
        polja.push(File_rank{file,rank});
        if !tabla.da_li_je_polje_prazno(&File_rank{file,rank}){
            break;
        }
        file += 1;
        rank -= 1;
    }

    file = file_lovca - 1;
    rank = rank_lovca + 1;
    while file >= A_FILE && rank <= 8 {
        if tabla.da_li_je_figura_boje_na_polju(ja_sam_beli, rank, file){
            break;
        }
        polja.push(File_rank{file,rank});
        if !tabla.da_li_je_polje_prazno(&File_rank{file,rank}){
            break;
        }
        file -= 1;
        rank += 1;
    }
}

#[cfg(test)]
pub mod test_lovac{
    use crate::tabla::{Tabla, self, A_FILE, B_FILE, C_FILE, D_FILE, E_FILE, F_FILE, G_FILE, H_FILE,
        kretanje_figura::lovac::{lovac_napada_polje, potezi_lovca}, Rokada, File_rank};
        
        fn test_kretanje_lovca(broj_polja: usize, file_lovca: u8, rank_lovca: u8){
            let polje: File_rank = File_rank{file: file_lovca, rank: rank_lovca};
            assert_eq!(broj_polja, 
                crate::tabla::kretanje_figura::lovac::prirodno_kretanje_lovca(&Tabla::pocetna_pozicija(), &polje, &Rokada::new_sve_rokade_moguce(), &None, true).len());
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
        
        let polje_lovca: File_rank = File_rank::new(file_lovca, rank_lovca);
        let polje_koje_napadam: File_rank = File_rank::new(file_kralja, rank_kralja);
        assert_eq!(
            lovac_napada_kralja,
             crate::tabla::kretanje_figura::lovac::lovac_napada_polje(&tabla, &polje_koje_napadam, &polje_lovca, true));
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

    #[test]
    fn ako_stavim_lovca_na_d4_imace_8_poteza(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(C_FILE, 1, D_FILE, 4);

        let potezi_lovca: Vec<File_rank> = potezi_lovca(&tabla, &File_rank::new(D_FILE,4), &tabla.rokada(), &None, true);
        assert_eq!(8, potezi_lovca.len());
        assert_eq!(true, potezi_lovca.contains(&File_rank::new(G_FILE, 7)));
        assert_eq!(true, potezi_lovca.contains(&File_rank::new(A_FILE, 7)));
    }
}
