use crate::tabla::{F_FILE, C_FILE, B_FILE, D_FILE, E_FILE};
use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};

use super::figure::abs;
use super::figure::ako_su_validni_dodaj_u_vektor;

pub fn prirodno_kretanje_kralja<T>(
    tabla: &T,
    polje_na_kom_se_nalazim: &File_rank,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: &Option<u8>, ja_sam_beli: bool) -> Vec<File_rank>
    where T:Ima_podatke_o_tabli{
     
        let rank: i32 = polje_na_kom_se_nalazim.rank as i32;
        let file: i32 = polje_na_kom_se_nalazim.file as i32;

        let mut polja: Vec<File_rank> = Vec::new();
        ako_su_validni_dodaj_u_vektor(&mut polja, rank, file-1);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank-1, file-1);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank-1, file);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank-1, file+1);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank, file+1);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank+1, file+1);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank+1, file);
        ako_su_validni_dodaj_u_vektor(&mut polja, rank+1, file-1);

        probaj_rokadu(&mut polja, tabla, rokada, ja_sam_beli);
        polja
    }

    fn probaj_rokadu<T>(polja: &mut Vec<File_rank>, tabla: &T, rokada: &Rokada, ja_sam_beli: bool)
    where T: Ima_podatke_o_tabli{
        if rokada.nijedna_rokada_nije_moguca() {
            return;
        }
    
        let (kraljicina_rokada, kraljeva_rokada, kraljev_rank) = 
        get_kraljicina_rokada_kraljeva_rokada_kraljev_rank(rokada, ja_sam_beli);

        if moze_kraljeva_rokada(kraljev_rank, tabla, kraljeva_rokada, ja_sam_beli){
            polja.push(File_rank{file: G_FILE, rank: kraljev_rank});
        }

        if moze_kraljicina_rokada(kraljev_rank, tabla, kraljicina_rokada, ja_sam_beli){
            polja.push(File_rank{file: C_FILE, rank: kraljev_rank});
        }
    }

    fn moze_kraljicina_rokada<T>(kraljev_rank: u8, tabla: &T, kraljicina_rokada: bool, ja_sam_beli: bool) -> bool 
    where T: Ima_podatke_o_tabli
    {
        let mut polja_izmedju_kraljicine_rokade: Vec<File_rank> = vec![
            File_rank{file: D_FILE, rank: kraljev_rank},
            File_rank{file: C_FILE, rank: kraljev_rank},
            File_rank{file: B_FILE, rank: kraljev_rank},
        ];
        let polja_koja_ne_smeju_da_budu_napadnuta: Vec<File_rank> = vec![
            File_rank{file: E_FILE, rank: kraljev_rank},
            File_rank{file: D_FILE, rank: kraljev_rank},
            File_rank{file: C_FILE, rank: kraljev_rank},
        ];

        if kraljicina_rokada && tabla.da_li_su_polja_prazna(&polja_izmedju_kraljicine_rokade){
            if tabla.polja_nisu_napadnuta(&polja_koja_ne_smeju_da_budu_napadnuta, !ja_sam_beli){
                return true;
            }
        }
        false
    }


    fn moze_kraljeva_rokada<T>(kraljev_rank: u8, tabla: &T, kraljeva_rokada: bool, ja_sam_beli: bool) -> bool
    where T:Ima_podatke_o_tabli
    {
        let polja_izmedju_kraljeve_rokade: Vec<File_rank> = vec![
            File_rank{file: F_FILE, rank: kraljev_rank},
            File_rank{file: G_FILE, rank: kraljev_rank}
        ];
        let polja_koja_ne_smeju_da_budu_napadnuta: Vec<File_rank> = vec![
            File_rank{file: E_FILE, rank: kraljev_rank},
            File_rank{file: F_FILE, rank: kraljev_rank},
            File_rank{file: G_FILE, rank: kraljev_rank}
        ];
        if kraljeva_rokada &&
            tabla.da_li_su_polja_prazna(&polja_izmedju_kraljeve_rokade)
            &&
            tabla.polja_nisu_napadnuta(&polja_koja_ne_smeju_da_budu_napadnuta, !ja_sam_beli){
                return true;
            }

        false    
    }


    fn get_kraljicina_rokada_kraljeva_rokada_kraljev_rank(rokada: &Rokada, ja_sam_beli: bool) 
    -> (bool, bool, u8) {
          if ja_sam_beli{
                return (
                        !rokada.bela_kraljeva_rokada_vise_nije_moguca,
                        !rokada.bela_kraljicina_rokada_vise_nije_moguca,
                        1 as u8
                        )  
            } 

            return
                (
                    !rokada.crna_kraljeva_rokada_vise_nije_moguca,
                    !rokada.crna_kraljicina_rokada_vise_nije_moguca,
                     8 as u8
                )
            
        }
    

pub fn kralj_napada_polje<T>(tabla: &T, protivnikovo_polje: &File_rank, moje_polje: &File_rank, ja_sam_beli: bool) -> bool 
where T:Ima_podatke_o_tabli{
    let rank: u8 = protivnikovo_polje.rank;
    let file: u8 = protivnikovo_polje.file;
    let moj_rank: u8 = moje_polje.rank;
    let moj_file: u8 = moje_polje.file;

    if moj_rank == rank && abs(file as i32 - moj_file as i32) == 1 {
        return true;
    }
    if abs(moj_rank as i32 - rank as i32) == 1 && abs(moj_file as i32 - file as i32) == 1{
        return true;
    }
    moj_file == file && abs(rank as i32 - moj_rank as i32) == 1
}

pub fn kralj_moze_doci_na_polje<T>(tabla: &T, polje_na_koje_dolazim: &File_rank, moje_polje: &File_rank, ja_sam_beli: bool) -> bool
    where T:Ima_podatke_o_tabli
    {
        let rank_destinacije: u8 = polje_na_koje_dolazim.rank;
        let file_destinacije: u8 = polje_na_koje_dolazim.file;

        if tabla.da_li_je_figura_boje_na_polju(ja_sam_beli, rank_destinacije, file_destinacije){
            return false
        }

        if kralj_napada_polje(tabla, polje_na_koje_dolazim, moje_polje, ja_sam_beli) {
            return true;
        }
        
        let (kraljicina_rokada, kraljeva_rokada, kraljev_rank) = 
        get_kraljicina_rokada_kraljeva_rokada_kraljev_rank(&tabla.get_rokada(), ja_sam_beli);

        if file_destinacije == G_FILE && moze_kraljeva_rokada(kraljev_rank, tabla, kraljeva_rokada, ja_sam_beli){
            return true
        }

        if file_destinacije == C_FILE && moze_kraljicina_rokada(kraljev_rank, tabla, kraljicina_rokada, ja_sam_beli){
            return true
        }
        false
}

pub fn potezi_kralja<T>(
    tabla: &T,
    polje_na_kom_se_nalazim: &File_rank,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: &Option<u8>, ja_sam_beli: bool) -> Vec<File_rank>
    where T:Ima_podatke_o_tabli{
        let mut polja_prirodnog_kretanja: Vec<File_rank> = prirodno_kretanje_kralja(tabla, polje_na_kom_se_nalazim, rokada, fajl_pijuna_2_polja, ja_sam_beli);
        
        let mut polja_kralja: Vec<File_rank> = Vec::new();
        for polje in polja_prirodnog_kretanja {
            if kralj_moze_doci_na_polje(tabla, &polje, polje_na_kom_se_nalazim, ja_sam_beli){
                polja_kralja.push(polje);
            }
        }
        polja_kralja
}

#[cfg(test)]
pub mod test_kralj{
    use crate::tabla::{Tabla, E_FILE,A_FILE, B_FILE, C_FILE, D_FILE, F_FILE, G_FILE, H_FILE, Rokada, File_rank, kretanje_figura::kralj::potezi_kralja};

    use super::prirodno_kretanje_kralja;


    fn test_kralj_napada_kralja(file_belog_kralja: u8, rank_belog_kralja: u8,
         file_crnog_kralja: u8, rank_crnog_kralja: u8, 
        napadam_kralja: bool){
            let tabla: Tabla = Tabla::pocetna_pozicija()
            .odigraj_validan_potez_bez_promocije(E_FILE, 1, file_belog_kralja, rank_belog_kralja)
            .odigraj_validan_potez_bez_promocije(E_FILE, 8, file_crnog_kralja, rank_crnog_kralja);
            
            let polje_kralja: File_rank = File_rank::new(file_belog_kralja, rank_belog_kralja);
            let polje_koje_napadam: File_rank = File_rank::new(file_crnog_kralja, rank_crnog_kralja);
            assert_eq!(
                napadam_kralja,
                 crate::tabla::kretanje_figura::kralj::kralj_napada_polje(&tabla, &polje_koje_napadam, &polje_kralja, true));
        }

        #[test]
        fn na_e4_e5(){
            test_kralj_napada_kralja(E_FILE, 4, E_FILE, 5, true);
        }

        #[test]
        fn na_b4_c4(){
            test_kralj_napada_kralja(B_FILE, 4, C_FILE, 4, true);
        }

        #[test]
        fn na_g6_g5(){
            test_kralj_napada_kralja(G_FILE, 6, G_FILE, 5, true);
        }

        #[test]
        fn na_c6_b5(){
            test_kralj_napada_kralja(C_FILE, 6, B_FILE, 5, true);
        }

        #[test]
        fn kralj_na_d3_ne_napada_kralja_na_f4(){
            test_kralj_napada_kralja(D_FILE, 3, F_FILE, 4, false);
        }


        #[test]
        fn na_f4_e5(){
            test_kralj_napada_kralja(F_FILE, 4, E_FILE, 5, true);
        }

        #[test]
        fn kralj_na_c5_ne_napada_kralja_na_e4(){
            test_kralj_napada_kralja(C_FILE, 5, E_FILE, 4, false);
        }

        #[test]
        fn na_b3_b5_ne_napada(){
            test_kralj_napada_kralja(B_FILE, 3, B_FILE, 5, false);
        }

        #[test]
        fn na_c5_e6_ne_napada(){
            test_kralj_napada_kralja(C_FILE, 5, E_FILE, 6, false);
        }

        fn obe_rokade_mogu() -> Tabla{
             Tabla::pocetna_pozicija().odigraj_validan_potez_bez_promocije (F_FILE, 1, F_FILE, 3)
            .odigraj_validan_potez_bez_promocije(F_FILE, 8, F_FILE, 6)
            .odigraj_validan_potez_bez_promocije (G_FILE, 1, G_FILE, 3)
            .odigraj_validan_potez_bez_promocije(G_FILE, 8, G_FILE, 6)
        }

        fn broj_kraljevih_poteza(file: u8, rank: u8, rokada: &Rokada, ja_sam_beli: bool) -> usize {
            prirodno_kretanje_kralja(&Tabla::pocetna_pozicija(),&File_rank{file, rank}, rokada, &None, ja_sam_beli).len()
        }

        #[test]
        fn testiraj_da_je_kraljeva_rokada_moguca(){
            let tabla: Tabla = obe_rokade_mogu(); 
            let potezi: Vec<File_rank> = prirodno_kretanje_kralja(&tabla, &File_rank::new(E_FILE, 1), &tabla.rokada(), &None, true);
            assert_eq!(true, potezi.contains(&File_rank::new(G_FILE, 1)));
           
            let tabla_2: Tabla = tabla.odigraj_validan_potez_bez_promocije(E_FILE, 1,G_FILE, 1);
            assert_eq!(true, tabla_2.rokada().bela_kraljeva_rokada_vise_nije_moguca);
            let potezi_crnog: Vec<File_rank> = prirodno_kretanje_kralja(&tabla, &File_rank::new(E_FILE,8), &tabla.rokada(), &None, false);
            assert_eq!(true, potezi_crnog.contains(&File_rank::new(G_FILE, 8)));
            let tabla_nakon_obe_rokade: Tabla = tabla_2.odigraj_validan_potez_bez_promocije(E_FILE, 8, D_FILE, 8);
            assert_eq!(true, tabla_nakon_obe_rokade.rokada().crna_kraljicina_rokada_vise_nije_moguca);
        }

        #[test]
        fn testiraj_poteze_kralja_testiraj_da_je_kraljeva_rokada_moguca(){
            let tabla: Tabla = obe_rokade_mogu(); 
            let potezi: Vec<File_rank> = potezi_kralja(&tabla, &File_rank::new(E_FILE, 1), &tabla.rokada(), &None, true);
            assert_eq!(true, potezi.contains(&File_rank::new(G_FILE, 1)));
           
            let tabla_2: Tabla = tabla.odigraj_validan_potez_bez_promocije(E_FILE, 1,G_FILE, 1);
            assert_eq!(true, tabla_2.rokada().bela_kraljeva_rokada_vise_nije_moguca);
            let potezi_crnog: Vec<File_rank> = potezi_kralja(&tabla, &File_rank::new(E_FILE,8), &tabla.rokada(), &None, false);
            assert_eq!(true, potezi_crnog.contains(&File_rank::new(G_FILE, 8)));
            let tabla_nakon_obe_rokade: Tabla = tabla_2.odigraj_validan_potez_bez_promocije(E_FILE, 8, D_FILE, 8);
            assert_eq!(true, tabla_nakon_obe_rokade.rokada().crna_kraljicina_rokada_vise_nije_moguca);
        }

        #[test]
        fn kralj_na_f3_crni_ima_pijune_na_f4_e4_kralj_ima_5_poteza(){
            let tabla: Tabla = Tabla::pocetna_pozicija()
            .odigraj_validan_potez_bez_promocije(A_FILE, 2, A_FILE, 3)
            .odigraj_validan_potez_bez_promocije(F_FILE, 7, F_FILE, 4)
            .odigraj_validan_potez_bez_promocije(E_FILE, 1, F_FILE, 3)
            .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 4);

            let potezi_kralja: Vec<File_rank> = potezi_kralja(&tabla, &File_rank::new(F_FILE, 3), &tabla.rokada(), &None, tabla.beli_je_na_potezu());
            assert_eq!(5, potezi_kralja.len());
        }

        #[test]
        fn crni_kralj_ne_moze_rokadu_jer_ga_top_sprecava(){
            let tabla: Tabla = Tabla::pocetna_pozicija()
            .odigraj_validan_potez_bez_promocije(H_FILE, 1, F_FILE, 4)
            .odigraj_validan_potez_bez_promocije(F_FILE, 7, F_FILE, 5)
            .odigraj_validan_potez_bez_promocije(A_FILE, 2, A_FILE, 3)
            .odigraj_validan_potez_bez_promocije(G_FILE, 8, H_FILE, 6)
            .odigraj_validan_potez_bez_promocije(A_FILE, 3, A_FILE, 4)
            .odigraj_validan_potez_bez_promocije(C_FILE, 8, B_FILE, 4)
            .odigraj_validan_potez_bez_promocije(A_FILE, 4, A_FILE, 5);

            let potezi_kralja: Vec<File_rank> = potezi_kralja(&tabla, &File_rank::new(E_FILE, 8), &tabla.rokada(), &None, tabla.beli_je_na_potezu());
            assert_eq!(false, potezi_kralja.contains(&File_rank::new(G_FILE, 8)));
        }
}