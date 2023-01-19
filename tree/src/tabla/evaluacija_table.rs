use crate::proba_sah_drveta::vrednost_mata;

use super::{Figura, Tabla, nekompresirana_tabla::Nekompresirana_tabla, File_rank, D_FILE, E_FILE, F_FILE, KRALJ, LEVI_KONJ, DESNI_KONJ, LEVI_LOVAC, DESNI_LOVAC, A_FILE, H_FILE, E_PIJUN, D_PIJUN, F_PIJUN};

static PREDNOST_POTEZA: f32 = 0.2;
static KRALJ_NA_OTVORENOM: f32 = 3.0;
static KRALJ_NA_SREDINI: f32 = 1.0;
static KRALJ_NA_SREDINI_I_NEMA_ROKADE: f32 = 1.4;

static KRALJ_JE_DALEKO_U_ZAVRSNICI:f32 = 2.5;
static KRALJ_JE_NA_TRECEM_RANKU_U_ZAVRSNICI: f32 = 1.0;

static FIGURA_NIJE_NA_KRAJNJEM_RANKU: f32 = 0.325;
static KONJ_NIJE_NA_IVICNOM_FAJLU: f32 = 0.125;
static MATERIJAL_KAD_JE_PARTIJA_U_ZAVRSNICI:f32 = 20.7;

static CENTRALNI_PIJUN_NA_TRECEM_RANKU: f32 = 0.25;
static CENTRALNI_PIJUN_NA_CETVRTOM_RANKU: f32 = 0.5;
static CENTRALNI_PIJUN_NA_DRUGOJ_STRANI_TABLE: f32 = 0.75;
static NIJEDAN_CENTRALNI_PIJUN_NIJE_POMEREN_2_POLJA: f32 = 0.375;
static POMERANJE_F_PIJUNA_PRE_ROKADE: f32 = 0.5;

impl Tabla {
    /* Ovo je preciznija funkcija, jer gleda i stalemate, ali je zato i sporija.
    if self.nema_legalnih_poteza(nekompresirana_tabla) {
            if self.igrac_je_u_sahu(nekompresirana_tabla) {
                return vrednost_mata(beli_je_na_potezu)
            } else {
                return 0.0
            }
    } 
         */

    pub fn nerekursivno_evaluiraj_poziciju(&self, nekompresirana_tabla: &Nekompresirana_tabla) -> f32 {
        let beli_je_na_potezu: bool = self.beli_je_na_potezu(); 
        if self.igrac_je_u_sahu(nekompresirana_tabla){
            if self.nema_legalnih_poteza(nekompresirana_tabla){
                return vrednost_mata(beli_je_na_potezu);
            }
        }
        if self.pre_koliko_poteza_je_50_move_rule_pomeren() >= 50 {
            return 0.0
        } 

        let (beli_materijal,crni_materijal,beli_ima_kraljicu,crni_ima_kraljicu) =  self.evaluacija_materijala(beli_je_na_potezu);
        let beli_kralj: File_rank = File_rank::new_iz_broja(self.bele_figure[KRALJ]);
        let crni_kralj: File_rank = File_rank::new_iz_broja(self.crne_figure[KRALJ]);

        let (beli_kralj_eval, crni_kralj_eval) = self.eval_pozicija_kralja(beli_materijal, crni_materijal, &beli_kralj, &crni_kralj, beli_ima_kraljicu, crni_ima_kraljicu);
        let (eval_belih_figura, eval_crnih_figura) = self.eval_pozicije_figura_podrazumeva_figure_se_nalaze_izmedju_levog_i_desnog_konja();
        let (eval_belih_pijuna, eval_crnih_pijuna) = self.eval_pijuna();

        let beli_eval: f32 = beli_materijal + beli_kralj_eval + eval_belih_figura + eval_belih_pijuna;
        let crni_eval: f32 = crni_materijal + crni_kralj_eval + eval_crnih_figura + eval_crnih_pijuna;
        beli_eval - crni_eval
    }

    fn eval_pijuna(&self) -> (f32, f32){
        let mut beli_eval: f32 = 0.0;
        let mut crni_eval: f32 = 0.0;
        let beli_e_pijun: File_rank = File_rank::new_iz_broja(self.bele_figure[E_PIJUN]);
        let beli_d_pijun: File_rank = File_rank::new_iz_broja(self.bele_figure[D_PIJUN]);
        let crni_e_pijun: File_rank = File_rank::new_iz_broja(self.crne_figure[E_PIJUN]);
        let crni_d_pijun: File_rank = File_rank::new_iz_broja(self.crne_figure[D_PIJUN]);

        let mut makar_jedan_beli_centralni_pijun_je_pomeren_2_polja = false;
        if !Tabla::figura_je_pojedena(&self.bele_figure, E_PIJUN){
          if beli_e_pijun.rank == 3 {
                beli_eval += CENTRALNI_PIJUN_NA_TRECEM_RANKU;
          } else if beli_e_pijun.rank == 4 {
                makar_jedan_beli_centralni_pijun_je_pomeren_2_polja = true;
                beli_eval += CENTRALNI_PIJUN_NA_CETVRTOM_RANKU;
          } else if beli_e_pijun.rank > 4 {
                makar_jedan_beli_centralni_pijun_je_pomeren_2_polja = true;
                beli_eval += CENTRALNI_PIJUN_NA_DRUGOJ_STRANI_TABLE;
            }
        }

        if !Tabla::figura_je_pojedena(&self.bele_figure, D_PIJUN){
            if beli_d_pijun.rank == 3 {
                beli_eval += CENTRALNI_PIJUN_NA_TRECEM_RANKU;
            } else if beli_d_pijun.rank == 4 {
                makar_jedan_beli_centralni_pijun_je_pomeren_2_polja = true;
                beli_eval += CENTRALNI_PIJUN_NA_CETVRTOM_RANKU;
             } else if beli_d_pijun.rank > 4 {
                makar_jedan_beli_centralni_pijun_je_pomeren_2_polja = true;
                beli_eval += CENTRALNI_PIJUN_NA_DRUGOJ_STRANI_TABLE;
            }
        }

        if !makar_jedan_beli_centralni_pijun_je_pomeren_2_polja {
            beli_eval += -NIJEDAN_CENTRALNI_PIJUN_NIJE_POMEREN_2_POLJA;
        }

        let mut makar_jedan_crni_centralni_pijun_je_pomeren_2_polja = false;
        if !Tabla::figura_je_pojedena(&self.crne_figure, E_PIJUN){
           if crni_e_pijun.rank == 6 {
                crni_eval += CENTRALNI_PIJUN_NA_TRECEM_RANKU;
           } else if crni_e_pijun.rank == 5 {
                makar_jedan_crni_centralni_pijun_je_pomeren_2_polja = true;
                crni_eval += CENTRALNI_PIJUN_NA_CETVRTOM_RANKU;
           } else if crni_e_pijun.rank < 5 {
                 makar_jedan_crni_centralni_pijun_je_pomeren_2_polja = true;
                 crni_eval += CENTRALNI_PIJUN_NA_DRUGOJ_STRANI_TABLE;
           }
        }

        if !Tabla::figura_je_pojedena(&self.crne_figure, D_PIJUN){
            if crni_d_pijun.rank == 6 {
                crni_eval += CENTRALNI_PIJUN_NA_TRECEM_RANKU;
            } else if crni_d_pijun.rank == 5 {
                 makar_jedan_crni_centralni_pijun_je_pomeren_2_polja = true;
                 crni_eval += CENTRALNI_PIJUN_NA_CETVRTOM_RANKU;
            } else if crni_d_pijun.rank < 5 {
                 makar_jedan_crni_centralni_pijun_je_pomeren_2_polja = true;
                 crni_eval += CENTRALNI_PIJUN_NA_DRUGOJ_STRANI_TABLE;
            }
        }

        if !makar_jedan_crni_centralni_pijun_je_pomeren_2_polja{
            crni_eval += -NIJEDAN_CENTRALNI_PIJUN_NIJE_POMEREN_2_POLJA;
        }

        (beli_eval, crni_eval)
    }

   

    fn eval_pozicije_figura_podrazumeva_figure_se_nalaze_izmedju_levog_i_desnog_konja(&self) -> (f32, f32) {
        let mut i: usize = LEVI_KONJ;
        let granica: usize = DESNI_KONJ;
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
        if beli_kralj.file == E_FILE && !self.rokada().nijedna_rokada_ove_boje_nije_moguca(false) {
            let f_pijun: File_rank = File_rank::new_iz_broja(self.bele_figure[F_PIJUN]);
            if f_pijun.rank != 2 {
                eval_beli_kralj += -POMERANJE_F_PIJUNA_PRE_ROKADE;
            }
        }

        let mut eval_crni_kralj: f32 = 0.0;
        if (beli_materijal > MATERIJAL_KAD_JE_PARTIJA_U_ZAVRSNICI) || beli_ima_kraljicu{
            eval_crni_kralj += self.evaluacija_kralja_protivnik_ima_dosta_materijala(false, crni_kralj);
        } else {
            eval_crni_kralj += self.evaluacija_kralja_protivnik_ima_manje_materijala(false, crni_kralj);
        }
        if crni_kralj.file == E_FILE && !self.rokada().nijedna_rokada_ove_boje_nije_moguca(false) {
            let f_pijun: File_rank = File_rank::new_iz_broja(self.crne_figure[F_PIJUN]);
            if f_pijun.rank != 7 {
                eval_crni_kralj += -POMERANJE_F_PIJUNA_PRE_ROKADE;
            }
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
        } else if udaljenost_od_ivice == 3 {
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
    use crate::tabla::{Tabla, A_FILE, B_FILE, C_FILE, D_FILE, E_FILE, F_FILE, G_FILE, H_FILE, F_PIJUN};


    #[test]
    pub fn test_beli_je_bolji_nakon_e4_f5_exf5(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(F_FILE, 7, F_FILE, 5);
        let tabla: Tabla = tabla.odigraj_validan_potez_bez_promocije(E_FILE, 4, F_FILE, 5);
        assert_eq!(true, Tabla::figura_je_pojedena(&tabla.crne_figure, F_PIJUN));

        let eval: f32 = tabla.nerekursivno_evaluiraj_poziciju(&tabla.to_nekompresirana_tabla());
        println!("eval nakon exf5: {}",eval);
        assert_eq!(true, tabla.nerekursivno_evaluiraj_poziciju(&tabla.to_nekompresirana_tabla()) > 0.0);
        assert_eq!(true, tabla.nerekursivno_evaluiraj_poziciju(&tabla.to_nekompresirana_tabla()) < 3.0);
    }
}
