use crate::tabla::{F_FILE, C_FILE, B_FILE, D_FILE};
use crate::tabla::{Rokada, Tabla, File_rank, H_FILE, A_FILE, G_FILE, Ima_podatke_o_tabli};

use super::figure::abs;
use super::figure::ako_su_validni_dodaj_u_vektor;

pub fn prirodno_kretanje_kralja<T>(
    tabla: &T,
    polje_na_kom_se_nalazim: u8,
    rokada: &Rokada, 
    fajl_pijuna_2_polja: Option<u8>, ja_sam_beli: bool) -> Vec<u8>
    where T:Ima_podatke_o_tabli{
        let (rank_u8, file_u8) = crate::broj_to_rank_file(polje_na_kom_se_nalazim);
        let rank: i32 = rank_u8 as i32;
        let file: i32 = file_u8 as i32;

        let mut polja: Vec<u8> = Vec::new();
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

    fn probaj_rokadu<T>(polja: &mut Vec<u8>, tabla: &T, rokada: &Rokada, ja_sam_beli: bool)
    where T: Ima_podatke_o_tabli{
        if rokada.nijedna_rokada_nije_moguca() {
            return;
        }
    
        let (kraljicina_rokada, kraljeva_rokada, kraljev_rank) = 
        get_kraljicina_rokada_kraljeva_rokada_kraljev_rank(rokada, ja_sam_beli);

        if moze_kraljeva_rokada(kraljev_rank, tabla, kraljeva_rokada, ja_sam_beli){
            polja.push(crate::file_rank_to_broj(G_FILE, kraljev_rank));
        }

        if moze_kraljicina_rokada(kraljev_rank, tabla, kraljicina_rokada, ja_sam_beli){
            polja.push(crate::file_rank_to_broj(C_FILE, kraljev_rank));
        }
    }

    fn moze_kraljicina_rokada<T>(kraljev_rank: u8, tabla: &T, kraljicina_rokada: bool, ja_sam_beli: bool) -> bool 
    where T: Ima_podatke_o_tabli
    {
        let mut polja_izmedju_kraljicine_rokade: Vec<u8> = vec![
            crate::file_rank_to_broj(D_FILE, kraljev_rank),
            crate::file_rank_to_broj(C_FILE, kraljev_rank),
            crate::file_rank_to_broj(B_FILE, kraljev_rank),
        ];
        if kraljicina_rokada && tabla.da_li_su_polja_prazna(&polja_izmedju_kraljicine_rokade){
            polja_izmedju_kraljicine_rokade.pop();
            if tabla.polja_nisu_napadnuta(&polja_izmedju_kraljicine_rokade, !ja_sam_beli){
                return true;
            }
        }
        false
    }


    fn moze_kraljeva_rokada<T>(kraljev_rank: u8, tabla: &T, kraljeva_rokada: bool, ja_sam_beli: bool) -> bool
    where T:Ima_podatke_o_tabli
    {
        let polja_izmedju_kraljeve_rokade: Vec<u8> = vec![
            crate::file_rank_to_broj(F_FILE, kraljev_rank),
            crate::file_rank_to_broj(G_FILE, kraljev_rank)
        ];
        if kraljeva_rokada &&
            tabla.da_li_su_polja_prazna(&polja_izmedju_kraljeve_rokade)
            &&
            tabla.polja_nisu_napadnuta(&polja_izmedju_kraljeve_rokade, !ja_sam_beli){
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
    

pub fn kralj_napada_polje<T>(tabla: &T, protivnikovo_polje: u8, moje_polje: u8, ja_sam_beli: bool) -> bool 
where T:Ima_podatke_o_tabli{
    let (rank, file) = crate::broj_to_rank_file(protivnikovo_polje);
    let (moj_rank, moj_file) = crate::broj_to_rank_file(moje_polje);

    if moj_rank == rank && abs(file as i32 - moj_file as i32) == 1 {
        return true;
    }
    if abs(moj_rank as i32 - rank as i32) == 1 && abs(moj_file as i32 - file as i32) == 1{
        return true;
    }
    moj_file == file && abs(rank as i32 - moj_rank as i32) == 1
}

pub fn kralj_moze_doci_na_polje<T>(tabla: &T, polje_na_koje_dolazim: u8, moje_polje: u8, ja_sam_beli: bool) -> bool
    where T:Ima_podatke_o_tabli
    {
        let (rank_destinacije, file_destinacije) = crate::broj_to_rank_file(polje_na_koje_dolazim);
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


#[cfg(test)]
pub mod test_kralj{
    use crate::tabla::{Tabla, E_FILE,A_FILE, B_FILE, C_FILE, D_FILE, F_FILE, G_FILE, H_FILE, Rokada};

    use super::prirodno_kretanje_kralja;


    fn test_kralj_napada_kralja(file_belog_kralja: u8, rank_belog_kralja: u8,
         file_crnog_kralja: u8, rank_crnog_kralja: u8, 
        napadam_kralja: bool){
            let tabla: Tabla = Tabla::pocetna_pozicija()
            .odigraj_validan_potez_bez_promocije(E_FILE, 1, file_belog_kralja, rank_belog_kralja)
            .odigraj_validan_potez_bez_promocije(E_FILE, 8, file_crnog_kralja, rank_crnog_kralja);
            
            let polje_kralja: u8 = crate::file_rank_to_broj(file_belog_kralja, rank_belog_kralja);
            let polje_koje_napadam: u8 = crate::file_rank_to_broj(file_crnog_kralja, rank_crnog_kralja);
            assert_eq!(
                napadam_kralja,
                 crate::tabla::kretanje_figura::kralj::kralj_napada_polje(&tabla, polje_koje_napadam, polje_kralja, true));
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
            prirodno_kretanje_kralja(&Tabla::pocetna_pozicija(),crate::file_rank_to_broj(file, rank), rokada, None, ja_sam_beli).len()
        }

        #[test]
        fn testiraj_da_je_kraljeva_rokada_moguca(){
            let tabla: Tabla = obe_rokade_mogu(); 
            let potezi: Vec<u8> = prirodno_kretanje_kralja(&tabla, 4, &tabla.rokada(), None, true);
            assert_eq!(true, potezi.contains(&6));
           
            let tabla_2: Tabla = tabla.odigraj_validan_potez_bez_promocije(E_FILE, 1,G_FILE, 1);
            assert_eq!(true, tabla_2.rokada().bela_kraljeva_rokada_vise_nije_moguca);
            let potezi_crnog: Vec<u8> = prirodno_kretanje_kralja(&tabla, 60, &tabla.rokada(), None, false);
            assert_eq!(true, potezi_crnog.contains(&62));
            let tabla_nakon_obe_rokade: Tabla = tabla_2.odigraj_validan_potez_bez_promocije(E_FILE, 8, D_FILE, 8);
            assert_eq!(true, tabla_nakon_obe_rokade.rokada().crna_kraljicina_rokada_vise_nije_moguca);
        }
}