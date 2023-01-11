use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};
use super::figure::abs;
use super::figure::ako_su_validni_dodaj_u_vektor;


pub fn polja_na_koja_ide_top<T>(
    tabla: &T,
    polje_na_kom_se_nalazim: u8,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: Option<u8>, ja_sam_beli: bool) -> Vec<u8>
    where T:Ima_podatke_o_tabli{

         let mut polja: Vec<u8> = Vec::new();
         let (rank_u8, file_u8) = Tabla::broj_to_rank_file(polje_na_kom_se_nalazim);

         for i in A_FILE..(H_FILE+1){
            for j in 1..9 {
                if i == file_u8 && j==rank_u8{
                    continue; /* Nijedna figura ne moze da ode na polje na kom se vec nalazi. */
                }
                if i != file_u8 && j != rank_u8 {
                    continue; /* Top se krece ili po istom fajlu, ili po istom ranku. 
                    Ako polje destinacije nije ni isti fajl, ni isti rank, na to polje top ne moze da ode. */
                }
                
                polja.push(Tabla::file_rank_to_broj(i, j))
            }
         }
         polja
    }

pub fn top_napada_kralja<T>(tabla: &T, polje_na_kom_se_nalazim: u8, kralj_je_beo: bool) -> bool
where T: Ima_podatke_o_tabli
{
    let polje_kralja: u8 = tabla.pozicija_kralja(kralj_je_beo);
    top_napada_polje(tabla, polje_kralja, polje_na_kom_se_nalazim, !kralj_je_beo)
}

pub fn top_napada_polje<T>(tabla: &T, polje: u8, polje_na_kom_se_nalazim: u8, ja_sam_beo: bool) -> bool
where T: Ima_podatke_o_tabli
{
    let (rank, file) = Tabla::broj_to_rank_file(polje);
    let (moj_rank, moj_file) = Tabla::broj_to_rank_file(polje_na_kom_se_nalazim);
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

    let mut polja: Vec<u8> = Vec::new();

    if moj_rank == rank {    
        let min_file: u8;
        let max_file: u8;
        if moj_file < file {
            min_file = moj_file;
            max_file = file;
        } else {
            min_file = file;
            max_file = moj_file;
        }        
        for i in (min_file+1)..max_file{
            polja.push(Tabla::file_rank_to_broj(i, rank));
        }
        return tabla.da_li_su_polja_prazna(&polja)
    }

    /* Slucaj kad je isti fajl, a razlicit rank. */
    if moj_rank < rank {
        for i in (moj_rank+1)..rank{
            polja.push(Tabla::file_rank_to_broj(file, i));
        }
        tabla.da_li_su_polja_prazna(&polja)
    } else{
        for i in (rank+1)..moj_rank{
            polja.push(Tabla::file_rank_to_broj(file, i));
        }
        tabla.da_li_su_polja_prazna(&polja)
    }

    
}


pub fn top_moze_doci_na_polje<T>(tabla: &T, moje_polje: u8, polje_na_koje_dolazim: u8, ja_sam_beli: bool) -> bool
    where T:Ima_podatke_o_tabli
    {
        top_napada_polje(tabla, polje_na_koje_dolazim, moje_polje, ja_sam_beli)   
    }

#[cfg(test)]
mod top_test{
    use crate::tabla::{Tabla, E_FILE, A_FILE, G_FILE, Rokada, H_FILE, B_FILE, Ima_podatke_o_tabli};

    use super::{polja_na_koja_ide_top, top_napada_kralja};

    fn top_na_polje_kralj_na_polje(file_topa: u8, rank_topa: u8, file_kralja: u8, rank_kralja: u8)->Tabla{
        let tabla0 : Tabla = Tabla::pocetna_pozicija();
        let tabla1: Tabla = tabla0.odigraj_validan_potez_bez_promocije(A_FILE, 1, file_topa, rank_topa);
        tabla1.odigraj_validan_potez_bez_promocije(E_FILE, 8, file_kralja, rank_kralja)
    }

    fn na_koliko_polja<T>(file: u8, rank: u8, tabla: &T) -> usize
    where T:Ima_podatke_o_tabli
    {
        let polje_na_kom_se_nalazim: u8= Tabla::file_rank_to_broj(file, rank);
        polja_na_koja_ide_top(tabla, polje_na_kom_se_nalazim, &tabla.get_rokada(), tabla.get_file_pijuna_koji_se_pomerio_2_polja(), tabla.get_beli_je_na_potezu()).len()
    }
    #[test]
    fn top_sa_a4_vidi_14_polja(){
        let tabla: Tabla = top_na_polje_kralj_na_polje(A_FILE, 4, G_FILE, 8);
        assert_eq!(14, na_koliko_polja(A_FILE, 4, &tabla));

    }

    fn testiraj_top_napada_crnog_kralja<T>(tabla: &T, file_topa: u8, rank_topa: u8) -> bool 
    where T:Ima_podatke_o_tabli
    {
        let polje: u8 = Tabla::file_rank_to_broj(file_topa, rank_topa);
        top_napada_kralja(tabla, polje, false)
    }

    #[test]
    fn top_sa_h3_vidi_kralja_na_h6_kad_nema_nista_izmedju(){
        let tabla: Tabla = top_na_polje_kralj_na_polje(H_FILE, 3, H_FILE, 6);
        assert_eq!(true, testiraj_top_napada_crnog_kralja(&tabla, H_FILE, 3));
    }

    #[test]
    fn top_sa_b8_ne_vidi_kralja_na_e8_jer_ima_figura_izmedju(){
        let tabla: Tabla = top_na_polje_kralj_na_polje(B_FILE, 8, E_FILE, 8);
        assert_eq!(false, testiraj_top_napada_crnog_kralja(&tabla, H_FILE, 8));
    }

    #[test]
    fn top_sa_a3_napada_kralja_na_h3(){
        let tabla: Tabla = top_na_polje_kralj_na_polje(A_FILE, 3, H_FILE, 3);
        assert_eq!(true, testiraj_top_napada_crnog_kralja(&tabla, A_FILE, 3));
    }

    #[test]
    fn top_sa_b4_ne_napada_kralja_na_g6(){
        let polje: u8 = Tabla::file_rank_to_broj(B_FILE, 4);
        let tabla: Tabla = top_na_polje_kralj_na_polje(B_FILE, 4, G_FILE, 6);
        assert_eq!(false, testiraj_top_napada_crnog_kralja(&tabla, B_FILE, 4));
    }

}

