use crate::proba_sah_drveta::vrednost_mata;

use super::{Figura, Tabla, nekompresirana_tabla::Nekompresirana_tabla, File_rank, D_FILE, E_FILE, F_FILE, KRALJ, LEVI_KONJ, DESNI_KONJ, DESNI_LOVAC, A_FILE, H_FILE};

static PREDNOST_POTEZA: f32 = 0.2;
static KRALJ_NA_OTVORENOM: f32 = 3.0;
static KRALJ_NA_SREDINI: f32 = 1.0;
static KRALJ_NA_SREDINI_I_NEMA_ROKADE: f32 = 1.4;

static KRALJ_JE_DALEKO_U_ZAVRSNICI:f32 = 2.0;
static KRALJ_JE_NA_TRECEM_RANKU_U_ZAVRSNICI: f32 = 1.0;

static FIGURA_NIJE_NA_KRAJNJEM_RANKU: f32 = 0.3;
static KONJ_NIJE_NA_IVICNOM_FAJLU: f32 = 0.125;
static MATERIJAL_KAD_JE_PARTIJA_U_ZAVRSNICI:f32 = 20.7;


impl Tabla {

    pub fn nerekursivno_evaluiraj_poziciju(&self, nekompresirana_tabla: &Nekompresirana_tabla) -> f32 {
        let beli_je_na_potezu: bool = self.beli_je_na_potezu();

        /* 
        if self.nema_legalnih_poteza(nekompresirana_tabla) {
            if self.igrac_je_u_sahu(nekompresirana_tabla) {
                return vrednost_mata(beli_je_na_potezu)
            } else {
                return 0.0
            }
        } 
        if self.pre_koliko_poteza_je_50_move_rule_pomeren() >= 50 {
            return 0.0
        } */
        let (beli_materijal,crni_materijal,beli_ima_kraljicu,crni_ima_kraljicu) =  self.evaluacija_materijala(beli_je_na_potezu);
        
        let beli_kralj: File_rank = File_rank::new_iz_broja(self.bele_figure[KRALJ]);
        let crni_kralj: File_rank = File_rank::new_iz_broja(self.crne_figure[KRALJ]);

        let (beli_kralj_eval, crni_kralj_eval) = self.eval_pozicija_kralja(beli_materijal, crni_materijal, &beli_kralj, &crni_kralj, beli_ima_kraljicu, crni_ima_kraljicu);
        let (eval_belih_figura, eval_crnih_figura) = self.eval_pozicije_figura();
        
        let beli_eval: f32 = beli_materijal + beli_kralj_eval + eval_belih_figura;
        let crni_eval: f32 = crni_materijal + crni_kralj_eval + eval_crnih_figura;
        beli_eval - crni_eval
    }

    

    fn eval_pozicije_figura(&self) -> (f32, f32) {
        let mut i: usize = LEVI_KONJ;
        let granica: usize = DESNI_LOVAC;
        let mut bela_evaluacija: f32 = 0.0;
        let mut crna_evaluacija: f32 = 0.0;

        while i<= granica {
            if !Tabla::figura_je_pojedena(&self.bele_figure, i){
                let (rank, _) = crate::broj_to_rank_file(self.bele_figure[i]);
                if rank != 1 && rank != 8 {
                    bela_evaluacija += FIGURA_NIJE_NA_KRAJNJEM_RANKU;
                }
            }

            if !Tabla::figura_je_pojedena(&self.crne_figure, i){
                let (rank, _) = crate::broj_to_rank_file(self.crne_figure[i]);
                if rank != 1 && rank != 8 {
                    crna_evaluacija += FIGURA_NIJE_NA_KRAJNJEM_RANKU;
                }
            }
            i += 1;
        }

        let beli_konj: f32 = self.eval_konj_nije_na_ivici(&self.bele_figure);
        let crni_konj: f32 = self.eval_konj_nije_na_ivici(&self.crne_figure);

        (bela_evaluacija + beli_konj,  crna_evaluacija + crni_konj)
        
    }

    fn eval_konj_nije_na_ivici(&self, figure: &[u8;16]) -> f32 {
        let mut eval:f32 = 0.0;
        
        if !Tabla::figura_je_pojedena(figure, LEVI_KONJ){
            let ( _, file) = crate::broj_to_rank_file(figure[LEVI_KONJ]);
            if file != A_FILE && file != H_FILE {
                eval += KONJ_NIJE_NA_IVICNOM_FAJLU;
            }
        }

        if !Tabla::figura_je_pojedena(figure, DESNI_KONJ){
            let ( _, file) = crate::broj_to_rank_file(figure[DESNI_KONJ]);
            if file != A_FILE && file != H_FILE {
                eval += KONJ_NIJE_NA_IVICNOM_FAJLU;
            }
        }

        eval
    }



    fn eval_pozicija_kralja(&self, beli_materijal: f32, crni_materijal: f32, beli_kralj: &File_rank, crni_kralj: &File_rank, beli_ima_kraljicu: bool, crni_ima_kraljicu: bool) -> (f32, f32){
        let mut eval_beli_kralj: f32 = 0.0;
        if (crni_materijal > MATERIJAL_KAD_JE_PARTIJA_U_ZAVRSNICI) || crni_ima_kraljicu{
            eval_beli_kralj += self.evaluacija_kralja_protivnik_ima_dosta_materijala(true, beli_kralj)
        } else {
            eval_beli_kralj += self.evaluacija_kralja_protivnik_ima_manje_materijala(true, beli_kralj);
        }

        let mut eval_crni_kralj: f32 = 0.0;
        if (beli_materijal > MATERIJAL_KAD_JE_PARTIJA_U_ZAVRSNICI) || beli_ima_kraljicu{
            eval_crni_kralj += self.evaluacija_kralja_protivnik_ima_dosta_materijala(false, crni_kralj);
        } else {
            eval_crni_kralj += self.evaluacija_kralja_protivnik_ima_manje_materijala(false, crni_kralj);
        }

        (eval_beli_kralj, eval_crni_kralj)
    }


    fn evaluacija_kralja_protivnik_ima_dosta_materijala(&self, kralj_je_beo: bool, kralj: &File_rank) -> f32 {
        if kralj_je_beo {
            if kralj.rank > 2 {
                return -KRALJ_NA_OTVORENOM
            }
        } else {
            if kralj.rank < 7 {
                return -KRALJ_NA_OTVORENOM
            }
        }
       
        if (kralj.file == E_FILE || kralj.file == D_FILE || kralj.file == F_FILE){
            if self.rokada().nijedna_rokada_ove_boje_nije_moguca(kralj_je_beo){
                return -KRALJ_NA_SREDINI_I_NEMA_ROKADE
            }
            return -KRALJ_NA_SREDINI
        } 

        0.0
    }

    fn evaluacija_kralja_protivnik_ima_manje_materijala(&self, kralj_je_beli:bool, kralj: &File_rank) -> f32 {
        let udaljenost_od_ivice: u8;
        if kralj_je_beli {
            udaljenost_od_ivice = kralj.rank;
        } else {
            udaljenost_od_ivice = 9 - kralj.rank;    
        }

        if udaljenost_od_ivice <= 2 {
            return -KRALJ_JE_DALEKO_U_ZAVRSNICI
        }
        if udaljenost_od_ivice == 3 {
            return -KRALJ_JE_NA_TRECEM_RANKU_U_ZAVRSNICI
        }
        0.0
    }

    fn evaluacija_materijala(&self, beli_je_na_potezu: bool) -> (f32,f32, bool, bool) {
        let mut evaluacija_belog: f32 = 0f32;
        let mut evaluacija_crnog: f32 = 0f32;
        let mut beli_ima_kraljicu: bool = false;
        let mut crni_ima_kraljicu: bool = false;

        let mut i:usize = 1; /* 0 je kralj, on je uvek na tabli. */
        while i<16 {
            match Tabla::koja_figura_se_nalazi_u_bitu(&self.bele_figure, i){
                None => {},
                Some(figura) => {
                    evaluacija_belog += figura.vrednost();
                    match &figura{
                        &Figura::KRALJICA => {beli_ima_kraljicu = true;},
                        _ => {}
                    }
                }
            }

            match Tabla::koja_figura_se_nalazi_u_bitu(&self.crne_figure, i){
                None => {},
                Some(figura) => {
                    evaluacija_crnog += figura.vrednost();
                    match &figura {
                        &Figura::KRALJICA => {crni_ima_kraljicu = true;},
                        _ => {}
                    }
                }
            }
            i += 1;
        }

        (evaluacija_belog,evaluacija_crnog, beli_ima_kraljicu, crni_ima_kraljicu)
    }
}



#[cfg(test)]
mod test_evaluacija_table{
    use crate::tabla::{Tabla, A_FILE, B_FILE, C_FILE, D_FILE, E_FILE, F_FILE, G_FILE, H_FILE};


    #[test]
    pub fn test_beli_je_bolji_nakon_e4_d5_exd5(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 5)
        .odigraj_validan_potez_bez_promocije(E_FILE, 4, D_FILE, 5);
        assert_eq!(true, tabla.nerekursivno_evaluiraj_poziciju(&tabla.to_nekompresirana_tabla()) > 0.0);
        assert_eq!(true, tabla.nerekursivno_evaluiraj_poziciju(&tabla.to_nekompresirana_tabla()) < 2.0);
    }
}
