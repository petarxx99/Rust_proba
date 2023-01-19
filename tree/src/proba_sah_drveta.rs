use std::collections::{HashSet, HashMap};

use crate::tabla::{Tabla, potez::{Potez_bits, Potez}};
use crate::tabla::File_rank;
static BELI_JE_MATIRAO_CRNOG: f32 = 100.0;
static CRNI_JE_MATIRAO_BELOG: f32 = -100.0;

pub struct Eval_deteta{
    pub eval: f32,
    pub najbolji_potez_do_sad: bool,
}

impl Eval_deteta{
    pub fn new(eval: f32, najbolji_potez_do_sad: bool) -> Eval_deteta{
        Eval_deteta{eval, najbolji_potez_do_sad}
    }
}

impl Tabla{


    pub fn najbolji_potez_i_njegova_evaluacija(&self, dubina: u8) -> (Option<Potez_bits>, f32) {
        let protivnik_je_beli: bool = !self.beli_je_na_potezu();
        let ja_sam_beli: bool = self.beli_je_na_potezu();

        let mut najbolji_potez: Option<Potez_bits> = None;
        let mut najbolja_evaluacija: f32 = vrednost_mata(ja_sam_beli);

        let legalni_potezi: Vec<Potez_bits> = self.svi_legalni_potezi();
        for potez in legalni_potezi {
            let tabla: Tabla = self.tabla_nakon_poteza_bits(&potez);
            let (vrednost_poteza, _) = tabla.izracunaj_rekursivno(&None, protivnik_je_beli, dubina, 1, &mut HashMap::new())/* .unwrap() */ ;
            if ovo_je_najbolji_potez(najbolja_evaluacija, vrednost_poteza, ja_sam_beli){
                najbolji_potez = Some(potez);
                najbolja_evaluacija = vrednost_poteza;
            }
        }


        (najbolji_potez, najbolja_evaluacija)
    }

pub fn izracunaj_rekursivno(&self, vrednost_koju_protivnik_ima_u_dzepu: &Option<f32>, ja_volim_vise:  bool,
broj_rekursija: u8, trenutna_rekursija: u8, izracunate_table: &mut HashMap<Tabla, f32>) -> (f32, bool){
    if trenutna_rekursija >= broj_rekursija {
        let sopstvena_evaluacija: f32 = self.nerekursivno_evaluiraj_poziciju(&self.to_nekompresirana_tabla());
     //   return Some(sopstvena_evaluacija);
        if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, sopstvena_evaluacija, ja_volim_vise){
            return (sopstvena_evaluacija, false)
        } else {
            return (sopstvena_evaluacija, true)
        }
    }

    let ja_sam_beli: bool = self.beli_je_na_potezu();
    let legalni_potezi: Vec<Potez_bits> = self.svi_legalni_potezi();
    let broj_legalnih_poteza: usize = legalni_potezi.len();
    if broj_legalnih_poteza == 0 {
        if self.igrac_je_u_sahu(&self.to_nekompresirana_tabla()) {
         //   return Some(vrednost_mata(ja_volim_vise))
                return (vrednost_mata(ja_volim_vise), true)
        } else {
            if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, 0.0, ja_volim_vise){
                return (0.0, false)
            } else {
                return (0.0, true)
            }
        }
    }
    if self.pre_koliko_poteza_je_50_move_rule_pomeren() >= 50 {
        if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, 0.0, ja_volim_vise){
            return (0.0, false)
        } else {
            return (0.0, true)
        }
    }

    let mut najbolja_opcija_za_sad: f32 = vrednost_mata(ja_sam_beli);
    for legalan_potez in legalni_potezi {
        let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(&legalan_potez);
        
        
        let (vrednost_poteza, najbolji_potez) = tabla_nakon_poteza.izracunaj_rekursivno(&Some(najbolja_opcija_za_sad), !ja_volim_vise, broj_rekursija, trenutna_rekursija+1, izracunate_table);
        if najbolji_potez {
                 najbolja_opcija_za_sad = vrednost_poteza;

        }
        if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, najbolja_opcija_za_sad, ja_volim_vise){
                return (najbolja_opcija_za_sad, false)
        }
       
    }
    
 (najbolja_opcija_za_sad, true)

}

}

fn updejtuj_najbolji_potez(najbolji_potez_za_sad: & mut f32, novi_potez: f32, ja_volim_vise: bool){

            if ja_volim_vise && (novi_potez > *najbolji_potez_za_sad) {
                *najbolji_potez_za_sad = novi_potez;
            } 

            if !ja_volim_vise && (novi_potez < *najbolji_potez_za_sad) {
                *najbolji_potez_za_sad = novi_potez;
            }
    }




fn ovo_je_najbolji_potez(najbolji_potez_za_sad: f32, novi_potez: f32, ja_volim_vise: bool) -> bool {
            if ja_volim_vise && novi_potez > najbolji_potez_za_sad {
                return true
            }
            if !ja_volim_vise && novi_potez < najbolji_potez_za_sad{
                return true
            }
            false

}


fn protivnik_se_zajebo(potez_koji_je_protivnik_trebalo_da_odigra: &Option<f32>, evaluacija_posle_mog_poteza: f32, ja_volim_vise: bool) -> bool{
    if potez_koji_je_protivnik_trebalo_da_odigra.is_none() {
        return false;
    }
 
    if ja_volim_vise {
        evaluacija_posle_mog_poteza > potez_koji_je_protivnik_trebalo_da_odigra.unwrap()
    } else {
        evaluacija_posle_mog_poteza < potez_koji_je_protivnik_trebalo_da_odigra.unwrap()
    }
}

pub fn vrednost_mata(matiran_igrac_voli_vise: bool) -> f32 {
    if matiran_igrac_voli_vise {
        CRNI_JE_MATIRAO_BELOG
    } else {
        BELI_JE_MATIRAO_CRNOG
    }
}

fn vrati_bolju_evaluaciju(sopstvena_evaluacija: f32, rekursivna_evaluacija: Eval_deteta) -> f32{
    if sopstvena_evaluacija > rekursivna_evaluacija.eval {
        sopstvena_evaluacija
    } else {
        rekursivna_evaluacija.eval
    }
}

fn vrati_eval_deteta(evaluacija_cvora: f32, potez_koji_protivnik_ima_u_dzepu: &Option<f32>, ja_sam_beli: bool) -> Eval_deteta{
    if potez_koji_protivnik_ima_u_dzepu.is_none() {
        return Eval_deteta::new(evaluacija_cvora, true)
    }
    let potez_koji_protivnik_ima_u_dzepu = potez_koji_protivnik_ima_u_dzepu.unwrap();
    
    if ja_sam_beli && evaluacija_cvora >= potez_koji_protivnik_ima_u_dzepu{
        return Eval_deteta::new(evaluacija_cvora, false)
    }
    if ja_sam_beli && evaluacija_cvora <= potez_koji_protivnik_ima_u_dzepu{
        return Eval_deteta::new(evaluacija_cvora, true)
    }
    if !ja_sam_beli && evaluacija_cvora >= potez_koji_protivnik_ima_u_dzepu{
        return Eval_deteta::new(evaluacija_cvora, true)
    }
    
    Eval_deteta::new(evaluacija_cvora, false)
  
}


impl Tabla{

    pub fn najbolji_potez_i_njegova_evaluacija2(&self, dubina: u8) -> (Option<Potez_bits>, f32) {
        let protivnik_je_beli: bool = !self.beli_je_na_potezu();
        let ja_sam_beli: bool = self.beli_je_na_potezu();

        let mut najbolji_potez: Option<Potez_bits> = None;
        let mut najbolja_evaluacija: f32 = vrednost_mata(ja_sam_beli);

        let legalni_potezi: Vec<Potez_bits> = self.svi_legalni_potezi();
        for potez in legalni_potezi {
            let tabla: Tabla = self.tabla_nakon_poteza_bits(&potez);
            let eval_poteza: Eval_deteta = tabla.rekursivno_evaluiraj_poziciju(&None, protivnik_je_beli, dubina, 1)/* .unwrap() */ ;
            if eval_poteza.najbolji_potez_do_sad {
                najbolji_potez = Some(potez);
                najbolja_evaluacija = eval_poteza.eval;
            }
        }


        (najbolji_potez, najbolja_evaluacija)
    }

    pub fn rekursivno_evaluiraj_poziciju(&self, vrednost_koju_protivnik_ima_u_dzepu: &Option<f32>, ja_volim_vise:  bool,
        broj_rekursija: u8, trenutna_rekursija: u8) -> Eval_deteta {
            if trenutna_rekursija >= broj_rekursija {
                return self.gledaj_samo_forsirane_poteze(vrednost_koju_protivnik_ima_u_dzepu, ja_volim_vise, 6, 0)
            }

            let svi_legalni_potezi: Vec<Potez_bits> = self.svi_legalni_potezi();
            let (evaluacija_gotove_partije, partija_je_gotova) = self.evaluiraj_poziciju_ako_je_partija_gotova(svi_legalni_potezi.len(), vrednost_koju_protivnik_ima_u_dzepu, ja_volim_vise);
            if partija_je_gotova {
                return evaluacija_gotove_partije
            }
            
           let mut najbolja_opcija_za_sad: f32 = vrednost_mata(ja_volim_vise);
           for legalan_potez in svi_legalni_potezi {
                let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(&legalan_potez);
                let vrednost_poteza: Eval_deteta = tabla_nakon_poteza.rekursivno_evaluiraj_poziciju(&Some(najbolja_opcija_za_sad), !ja_volim_vise, broj_rekursija, trenutna_rekursija+1);
                if vrednost_poteza.najbolji_potez_do_sad {
                    najbolja_opcija_za_sad = vrednost_poteza.eval;
                    if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, vrednost_poteza.eval, ja_volim_vise){
                        return Eval_deteta::new(vrednost_poteza.eval, false)
                    }
                }
            }
            Eval_deteta::new(najbolja_opcija_za_sad, true)
    }
        
    pub fn gledaj_samo_forsirane_poteze(&self, vrednost_koju_protivnik_ima_u_dzepu: &Option<f32>, ja_volim_vise:  bool,
    max_rekursija: u8, trenutna_rekursija: u8) -> Eval_deteta {
            if trenutna_rekursija >= max_rekursija {
                let eval: f32 = self.nerekursivno_evaluiraj_poziciju(&self.to_nekompresirana_tabla());
                if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, eval, ja_volim_vise){
                    return Eval_deteta::new(eval, false)
                } 
                return Eval_deteta::new(eval, true)
            }

            let svi_legalni_potezi: Vec<Potez_bits> = self.svi_legalni_potezi();
            let (evaluacija_gotove_partije, partija_je_gotova) = self.evaluiraj_poziciju_ako_je_partija_gotova(svi_legalni_potezi.len(), vrednost_koju_protivnik_ima_u_dzepu, ja_volim_vise);
            if partija_je_gotova {
                return evaluacija_gotove_partije
            }
            
            if self.protivnik_preti_mat_u_jednom_potezu() || self.igrac_je_u_sahu(&self.to_nekompresirana_tabla()) {
                return self.evaluiraj_poziciju_zovuci_gledaj_samo_forsirane_poteze(&svi_legalni_potezi, vrednost_koju_protivnik_ima_u_dzepu, ja_volim_vise, max_rekursija, trenutna_rekursija)
            }

            let mut evaluacija_najgoreg_slucaja: f32 = vrednost_mata(self.beli_je_na_potezu());
            let moje_napadnute_figure: HashSet<u8> = self.moje_napadnute_figure();
            let broj_mojih_napadnutih_figura: usize = moje_napadnute_figure.len();
            let (forsirani_potezi, sahovi_uzimanja) = self.filtriraj_forsirane_poteze(&svi_legalni_potezi, moje_napadnute_figure);
            if sahovi_uzimanja.len() == 0 {
                let nerekursivna_evaluacija: f32 = self.nerekursivno_evaluiraj_poziciju(&self.to_nekompresirana_tabla());
                if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, nerekursivna_evaluacija, ja_volim_vise){
                    return Eval_deteta::new(nerekursivna_evaluacija, false)
                } else {
                    return Eval_deteta::new(nerekursivna_evaluacija, true)
                }
            }
            if broj_mojih_napadnutih_figura == 0  {
                evaluacija_najgoreg_slucaja = self.nerekursivno_evaluiraj_poziciju(&self.to_nekompresirana_tabla());
                if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, evaluacija_najgoreg_slucaja, ja_volim_vise){
                    return Eval_deteta::new(evaluacija_najgoreg_slucaja, false)
                }
            }

            let rekursivna_evaluacija: Eval_deteta = self.evaluiraj_poziciju_zovuci_gledaj_samo_forsirane_poteze(&sahovi_uzimanja, vrednost_koju_protivnik_ima_u_dzepu, ja_volim_vise, max_rekursija, trenutna_rekursija);
            vrati_eval_deteta(vrati_bolju_evaluaciju(evaluacija_najgoreg_slucaja, rekursivna_evaluacija),  vrednost_koju_protivnik_ima_u_dzepu, ja_volim_vise)
        }


    
    fn evaluiraj_poziciju_zovuci_gledaj_samo_forsirane_poteze(&self, legalni_potezi: &Vec<Potez_bits>, vrednost_koju_protivnik_ima_u_dzepu: &Option<f32>,
        ja_volim_vise: bool, max_rekursija: u8, trenutna_rekursija: u8) -> Eval_deteta{

        let mut najbolja_opcija_za_sad: f32 = vrednost_mata(ja_volim_vise);
        for legalan_potez in legalni_potezi {
            let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(legalan_potez);
            let vrednost_poteza: Eval_deteta = tabla_nakon_poteza.gledaj_samo_forsirane_poteze(&Some(najbolja_opcija_za_sad), !ja_volim_vise, max_rekursija, trenutna_rekursija+1);
            if vrednost_poteza.najbolji_potez_do_sad {
                najbolja_opcija_za_sad = vrednost_poteza.eval;
                if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, vrednost_poteza.eval, ja_volim_vise){
                    return Eval_deteta::new(vrednost_poteza.eval, false)
                }
            }
        }
        Eval_deteta::new(najbolja_opcija_za_sad, true)
    }

    fn sahovi_i_uzimanja_figura(&self, legalni_potezi: &Vec<Potez_bits>) -> Vec<Potez_bits> {
        let mut forsirani_potezi: Vec<Potez_bits> = Vec::new();

        for potez in legalni_potezi {  
/*  Ako napadam tudju figuru, ili ako potez daje sah. */
            if !self.polje_je_prazno(&File_rank{file: potez.file, rank:potez.rank}){
                forsirani_potezi.push(potez.copy());
            } else {
                let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(potez);
                if tabla_nakon_poteza.igrac_je_u_sahu(&self.to_nekompresirana_tabla()) {
                    forsirani_potezi.push(potez.copy())
                }
            }
        }

        forsirani_potezi
    }

    pub fn filtriraj_forsirane_poteze(&self, legalni_potezi: &Vec<Potez_bits>, moje_napadnute_figure: HashSet<u8>) -> (Vec<Potez_bits>, Vec<Potez_bits>){
        let mut forsirani_potezi: Vec<Potez_bits> = Vec::new();
        let mut sahovi_uzimanja: Vec<Potez_bits> = Vec::new();

        for potez in legalni_potezi {  
/* Ako je moja figura napadnuta, ako napadam tudju figuru, ili ako potez daje sah. */
            let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(potez);

            if !self.polje_je_prazno(&File_rank{file: potez.file, rank:potez.rank}){
                forsirani_potezi.push(potez.copy());
                sahovi_uzimanja.push(potez.copy());
            } else if tabla_nakon_poteza.igrac_je_u_sahu(&self.to_nekompresirana_tabla()) {
                forsirani_potezi.push(potez.copy());
                sahovi_uzimanja.push(potez.copy());
            } else if moje_napadnute_figure.contains(&potez.broj_figure) {
                forsirani_potezi.push(potez.copy());
            }
        }

        (forsirani_potezi, sahovi_uzimanja)
    }

   

    pub fn evaluiraj_poziciju_ako_je_partija_gotova(&self, broj_legalnih_poteza: usize, 
        vrednost_koju_protivnik_ima_u_dzepu: &Option<f32>,  ja_volim_vise: bool)
        -> (Eval_deteta, bool)
        {
            if broj_legalnih_poteza == 0 {
                if self.igrac_je_u_sahu(&self.to_nekompresirana_tabla()) {
                    return (Eval_deteta::new(vrednost_mata(ja_volim_vise), true), true)
                } else {
                    if Self::protivniku_odgovara_remi(vrednost_koju_protivnik_ima_u_dzepu, !ja_volim_vise) {                      
                        return (Eval_deteta::new(0.0, true), true)
                    } else {
                        return (Eval_deteta::new(0.0, false), true)
                    }
                }
            }
            if self.pre_koliko_poteza_je_50_move_rule_pomeren() >= 50 {
                if Self::protivniku_odgovara_remi(vrednost_koju_protivnik_ima_u_dzepu, !ja_volim_vise){
                    return (Eval_deteta::new(0.0, true), true)
                } else {
                    return (Eval_deteta::new(0.0, false), true)
                }
            }

            return (Eval_deteta::new(0.0, false), false)
        }

        fn protivniku_odgovara_remi(vrednost_najboljeg_protivnikovog_poteza: &Option<f32>, protivnik_je_beli: bool) -> bool {
            match vrednost_najboljeg_protivnikovog_poteza {
                None => true,
                Some(protivnikov_potez) => {
                    if protivnik_je_beli {
                        if *protivnikov_potez > 0.0 {
                            return false 
                        } else {
                            return true
                        }
                    } else { /* Ja sam beli, protivnik je crni. */
                        if *protivnikov_potez > 0.0 {
                            return true
                        } else {
                            return false
                        }
                    }
                }
            }
        }
}

#[cfg(test)]
mod proba_test{
    use std::collections::HashSet;

    use crate::{tabla::{Tabla, E_FILE, F_FILE, D_FILE, G_FILE, C_FILE, B_FILE, potez::Potez_bits, H_FILE}, proba_sah_drveta::BELI_JE_MATIRAO_CRNOG};

    
    #[test]
    fn test_broj_forsiranih_poteza_posle_e4_e5_Nc3_Bb4_d4_Qf6_Bc4_Ne7(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(B_FILE, 1, C_FILE, 3)
        .odigraj_validan_potez_bez_promocije(F_FILE, 8, B_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 8, F_FILE, 6)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4)
        .odigraj_validan_potez_bez_promocije(G_FILE, 8, E_FILE, 7);

        let moje_napadnute_figure: HashSet<u8> = tabla.moje_napadnute_figure();
        assert_eq!(3, moje_napadnute_figure.len());
        let legalni_potezi: Vec<Potez_bits> = tabla.svi_legalni_potezi();
        assert_eq!(38, legalni_potezi.len());
        let (forsirani_potezi, sahovi_uzimanja) = tabla.filtriraj_forsirane_poteze(&legalni_potezi, moje_napadnute_figure);
      /* Dva poteza f pijuna, dva poteza d pijuna, lovac uzima na f7. */  
        assert_eq!(5, forsirani_potezi.len());
    }
    


    #[test]
    fn test_evaluiraj_poziciju_posle_e4_e5_Qh5_Nc6_Bc4_Nf6_Qxf7(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(D_FILE, 1, H_FILE, 5)
        .odigraj_validan_potez_bez_promocije(B_FILE, 8, C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4)
        .odigraj_validan_potez_bez_promocije(G_FILE, 8, F_FILE, 6)
        .odigraj_validan_potez_bez_promocije(H_FILE, 5, F_FILE, 7);

        let (eval, partija_gotova) = tabla.evaluiraj_poziciju_ako_je_partija_gotova(0, &Some(2.0), false);
        assert_eq!(true, partija_gotova);
        assert_eq!(true, (BELI_JE_MATIRAO_CRNOG - 1.0) < eval.eval);
        assert_eq!(true, (BELI_JE_MATIRAO_CRNOG + 1.0) > eval.eval);
    }

    #[test]
    fn test_ako_je_pat_a_protivnik_bi_se_protivnik_zajebo_da_ode_u_varijaciju_koja_dozvoljava_pat(){
        let mut tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4);
        for _ in 0..50{
            tabla.povecaj_fifty_move_rule_za_1();
        }
        let potez_koji_je_protivnik_trebalo_da_odigra: Option<f32> = Some(3.0);
        let (eval, _) = tabla.evaluiraj_poziciju_ako_je_partija_gotova(0, &potez_koji_je_protivnik_trebalo_da_odigra, false);
        assert_eq!(false, eval.najbolji_potez_do_sad);
    }

    #[test]
    fn test_evaluacija_partije_ako_je_partija_gotova_u_ovom_slucaju_vraca_0_zbog_pata(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5);
        
        let potez_koji_je_crni_imao_u_dzepu: Option<f32> = Some(1.5);
        let (eval, _) = tabla.evaluiraj_poziciju_ako_je_partija_gotova(0, &potez_koji_je_crni_imao_u_dzepu, true);
        assert_eq!(true, eval.eval < 0.5);
        assert_eq!(true, eval.eval > -0.5);    
    }
}