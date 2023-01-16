use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};
use super::figure::abs;
use super::figure::ako_su_validni_dodaj_u_vektor;


pub fn polja_na_koja_ide_top<T>(
    tabla: &T,
    polje_na_kom_se_nalazim: &File_rank,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: &Option<u8>, ja_sam_beli: bool) -> Vec<File_rank>
    where T:Ima_podatke_o_tabli{
         let mut polja: Vec<File_rank> = Vec::new();
         let rank: u8 = polje_na_kom_se_nalazim.rank;
         let file: u8 = polje_na_kom_se_nalazim.file;

         let mut i: u8 = rank + 1;
         while i<= 8 {
            polja.push(File_rank{file, rank: i});
            i += 1;
         }
         i = rank - 1;
         while i>=1{
            polja.push(File_rank{file, rank: i});
            i -= 1;
         }
         i = file + 1;
         while i<=H_FILE{
            polja.push(File_rank{file: i, rank});
            i += 1;
         }
         i = file - 1;
         while i>=A_FILE{
            polja.push(File_rank{file: i, rank});
            i -= 1;
         }
         polja
    }



pub fn top_napada_polje<T>(tabla: &T, polje_meta: &File_rank, polje_na_kom_se_nalazim: &File_rank,  ja_sam_beo: bool) -> bool
where T: Ima_podatke_o_tabli
{
    let rank: u8 = polje_meta.rank;
    let file: u8 = polje_meta.file;
    let moj_rank: u8 = polje_na_kom_se_nalazim.rank;
    let moj_file: u8 = polje_na_kom_se_nalazim.file;

    if moj_rank == rank && moj_file == file {
        return false /* Figura ne moze da napada polje na kom se nalazi. */
    }
    if moj_rank != rank && moj_file != file {
        return false  /* Top napada vodoravno, ili horizontalno. */
    }
    /* Posle ovih uslova je ustanovljeno da se top nalazi vodoravno ili horizontalno u odnosu
    na polje koje napada. */
    if abs(moj_rank as i32 - rank as i32) == 1 || abs(moj_file as i32 - file as i32) == 1 {
        return true; /* Ako je top odmah pored mete, onda ne moze da bude nista izmedju topa i mete. */
    }

    let mut polja_izmedju: Vec<File_rank> = Vec::new();
    if moj_rank == rank {    
        let (min_file, max_file) = crate::min_max_broj(moj_file, file);  
        let mut i: u8 = min_file + 1;
        while i < max_file{
            polja_izmedju.push(File_rank{file: i, rank});
            i += 1;
        }     
    } else if moj_file == file{  /* Slucaj kad je isti fajl, a razlicit rank. */
        let (min_rank, max_rank) = crate::min_max_broj(moj_rank, rank);
        let mut i: u8 = min_rank + 1;
        while i < max_rank {
            polja_izmedju.push(File_rank{file, rank: i});
            i += 1;
        }
    }

    tabla.da_li_su_polja_prazna(&polja_izmedju)
}



pub fn top_moze_doci_na_polje<T>(tabla: &T, polje_na_koje_dolazim: &File_rank, moje_polje: &File_rank, ja_sam_beli: bool) -> bool
    where T:Ima_podatke_o_tabli
    {
        top_napada_polje(tabla, polje_na_koje_dolazim, moje_polje, ja_sam_beli)   
        &&
        !tabla.da_li_je_figura_boje_na_polju(ja_sam_beli, polje_na_koje_dolazim.rank, polje_na_koje_dolazim.file)
    }


pub fn potezi_topa<T>(tabla: &T,
    polje_na_kom_se_nalazim: &File_rank,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: &Option<u8>, ja_sam_beli: bool) -> Vec<File_rank>
    where T:Ima_podatke_o_tabli{
        let mut polja: Vec<File_rank> = Vec::new();
         let rank: u8 = polje_na_kom_se_nalazim.rank;
         let file: u8 = polje_na_kom_se_nalazim.file;

         dodaj_polja_topa_vertikalno(tabla, &mut polja, ja_sam_beli, 1, 9, rank, file);
         dodaj_polja_topa_vertikalno(tabla, &mut polja, ja_sam_beli, -1, 0, rank, file);
         dodaj_polja_topa_horizontalno(tabla, &mut polja, ja_sam_beli, 1, H_FILE+1, rank, file);
         dodaj_polja_topa_horizontalno(tabla, &mut polja, ja_sam_beli, -1, A_FILE-1, rank, file);
         polja
    }

    fn dodaj_polja_topa_vertikalno<T>(tabla: &T, polja: &mut Vec<File_rank>, ja_sam_beli: bool, increment: i8,
    ekskluzivna_granica: u8, rank_topa: u8, file: u8)
    where T:Ima_podatke_o_tabli {
        let mut rank: u8 = (rank_topa as i8 + increment) as u8;
        while rank != ekskluzivna_granica {
            if tabla.da_li_je_figura_boje_na_polju(ja_sam_beli, rank, file){
                break;
            }
            polja.push(File_rank{file, rank});
            if !tabla.da_li_je_polje_prazno(&File_rank{file, rank}){
                break;
            }
            rank = (rank as i8 + increment) as u8;
        }
    }


    fn dodaj_polja_topa_horizontalno<T>(tabla: &T, polja: &mut Vec<File_rank>, ja_sam_beli: bool, increment: i8,
        ekskluzivna_granica: u8, rank: u8, file_topa: u8)
        where T:Ima_podatke_o_tabli {
            let mut file: u8 = (file_topa as i8 + increment) as u8;
            while file != ekskluzivna_granica {
                if tabla.da_li_je_figura_boje_na_polju(ja_sam_beli, rank, file){
                    break;
                }
                polja.push(File_rank{file, rank});
                if !tabla.da_li_je_polje_prazno(&File_rank{file,rank}){
                    break;
                }
                file = (file as i8 + increment) as u8;
            }
        }
 
    
#[cfg(test)]
mod top_test{
    use crate::tabla::{Tabla, E_FILE, A_FILE, G_FILE, Rokada, H_FILE, B_FILE, Ima_podatke_o_tabli, F_FILE, D_FILE, C_FILE, DESNI_TOP, LEVI_TOP, kretanje_figura::top::top_moze_doci_na_polje, File_rank};

    use super::{polja_na_koja_ide_top, top_napada_polje, potezi_topa};

    fn top_na_polje_kralj_na_polje(file_topa: u8, rank_topa: u8, file_kralja: u8, rank_kralja: u8)->Tabla{
        let tabla0 : Tabla = Tabla::pocetna_pozicija();
        let tabla1: Tabla = tabla0.odigraj_validan_potez_bez_promocije(A_FILE, 1, file_topa, rank_topa);
        tabla1.odigraj_validan_potez_bez_promocije(E_FILE, 8, file_kralja, rank_kralja)
    }

    fn na_koliko_polja(file: u8, rank: u8, tabla: &Tabla) -> usize

    {
        let polje_na_kom_se_nalazim: File_rank = File_rank{file, rank};
        let nekompresirana_tabla = tabla.to_nekompresirana_tabla();
        
        polja_na_koja_ide_top(tabla, &polje_na_kom_se_nalazim, &tabla.get_rokada(), &nekompresirana_tabla.get_file_pijuna_koji_se_pomerio_2_polja(), tabla.get_beli_je_na_potezu()).len()
    }
    #[test]
    fn top_sa_a4_vidi_14_polja(){
        let tabla: Tabla = top_na_polje_kralj_na_polje(A_FILE, 4, G_FILE, 8);
        assert_eq!(14, na_koliko_polja(A_FILE, 4, &tabla));

    }

    fn testiraj_beli_top_napada_polje(file_topa: u8, rank_topa: u8, file_destinacije: u8, rank_destinacije: u8) -> bool 
 
    {
        let tabla: Tabla = top_na_polje_kralj_na_polje(file_topa, rank_topa, file_destinacije, rank_destinacije);
        let polje: File_rank = File_rank{file: file_topa, rank: rank_topa};
        let polje_koje_napadam: File_rank = File_rank{file: file_destinacije, rank: rank_destinacije};
        top_napada_polje(&tabla, &polje_koje_napadam, &polje, true)
    }

    #[test]
    fn top_sa_h3_vidi_kralja_na_h6_kad_nema_nista_izmedju(){
        assert_eq!(true, testiraj_beli_top_napada_polje(H_FILE, 3, H_FILE, 6));
    }

    #[test]
    fn top_sa_b8_ne_vidi_kralja_na_e8_jer_ima_figura_izmedju(){     
        assert_eq!(false, testiraj_beli_top_napada_polje(B_FILE, 8, H_FILE, 8));
    }

    #[test]
    fn top_sa_a3_napada_kralja_na_h3(){
        assert_eq!(true, testiraj_beli_top_napada_polje(A_FILE, 3, H_FILE, 3));
    }

    #[test]
    fn top_sa_b4_ne_napada_kralja_na_g6(){
        assert_eq!(false, testiraj_beli_top_napada_polje(B_FILE, 4, G_FILE, 6));
    }

    #[test]
    fn desni_top_moze_na_2_polja_levi_top_ni_na_jedno_posle_e4_e5_Nf3_Nc6_Bc4_Bc5_d4_Nf6(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(G_FILE, 1, F_FILE, 3)
        .odigraj_validan_potez_bez_promocije(B_FILE, 8, C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4)
        .odigraj_validan_potez_bez_promocije(F_FILE, 8, C_FILE, 5)
        .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 4)
        .odigraj_validan_potez_bez_promocije(G_FILE, 8, F_FILE, 6);
           
        assert_eq!(true, top_moze_doci_na_polje(&tabla, &File_rank::new(F_FILE, 1), &File_rank::new_iz_broja(tabla.bele_figure[DESNI_TOP]), tabla.beli_je_na_potezu()));
        assert_eq!(true, top_moze_doci_na_polje(&tabla, &File_rank::new(G_FILE, 1), &File_rank::new_iz_broja(tabla.bele_figure[DESNI_TOP]), tabla.beli_je_na_potezu()));
        assert_eq!(false, top_moze_doci_na_polje(&tabla, &File_rank::new(F_FILE, 1), &File_rank::new_iz_broja(tabla.bele_figure[LEVI_TOP]), tabla.beli_je_na_potezu()));
        assert_eq!(14, polja_na_koja_ide_top(&tabla, &File_rank::new_iz_broja(tabla.bele_figure[DESNI_TOP]), &tabla.rokada(), &None, tabla.beli_je_na_potezu()).len());
    }

    #[test]
    fn test_potezi_topa_posle_a4_i_Nc3(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(A_FILE, 2, A_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(B_FILE, 1, C_FILE, 3)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 6);
        
        let polja_topa: Vec<File_rank> = potezi_topa(&tabla, &File_rank::new(A_FILE, 1), &tabla.rokada(), &None, tabla.beli_je_na_potezu());
        assert_eq!(3, polja_topa.len());
        assert_eq!(true, polja_topa.contains(&File_rank::new(B_FILE, 1)));
        assert_eq!(true, polja_topa.contains(&File_rank::new(A_FILE, 2)));
        assert_eq!(true, polja_topa.contains(&File_rank::new(A_FILE, 3)));
    }

    #[test]
    fn test_top_sa_e4_dolazi_na_10_polja(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(A_FILE, 1, E_FILE, 4);

        let polja_topa: Vec<File_rank> = potezi_topa(&tabla, &File_rank::new(E_FILE, 4), &tabla.rokada(), &None, true);
        assert_eq!(11, polja_topa.len());
    }
}

