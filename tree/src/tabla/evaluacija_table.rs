use crate::proba_sah_drveta::vrednost_mata;

use super::{Figura, Tabla, nekompresirana_tabla::{Nekompresirana_tabla, Tabla_pijuna}, File_rank, D_FILE, E_FILE, F_FILE, KRALJ, LEVI_KONJ, DESNI_KONJ, LEVI_LOVAC, DESNI_LOVAC, A_FILE, H_FILE, E_PIJUN, D_PIJUN, F_PIJUN, LEVI_TOP, C_FILE, DESNI_TOP, KRALJICA, G_PIJUN, G_FILE, B_PIJUN, kretanje_figura::figure::abs, A_PIJUN};

static KOLIKO_POLJA_KRETANJA_FIGURA_VREDI_JEDAN_POEN: f32 = 32.0;
static PREDNOST_POTEZA: f32 = 0.2;
static KRALJ_NA_OTVORENOM: f32 = 3.0;
static KRALJ_NA_SREDINI: f32 = 1.0;
static KRALJ_NA_SREDINI_I_NEMA_ROKADE: f32 = 2.4;

static KRALJ_JE_DALEKO_U_ZAVRSNICI:f32 = 2.5;
static KRALJ_JE_NA_TRECEM_RANKU_U_ZAVRSNICI: f32 = 1.0;

static FIGURA_NIJE_NA_KRAJNJEM_RANKU: f32 = 0.325;
static KONJ_NIJE_NA_IVICNOM_FAJLU: f32 = 0.5;
static MATERIJAL_KAD_JE_PARTIJA_U_ZAVRSNICI:f32 = 20.7;

static CENTRALNI_PIJUN_NA_TRECEM_RANKU: f32 = 0.25;
static CENTRALNI_PIJUN_NA_CETVRTOM_RANKU: f32 = 0.5;
static CENTRALNI_PIJUN_NA_DRUGOJ_STRANI_TABLE: f32 = 0.75;
static NIJEDAN_CENTRALNI_PIJUN_NIJE_POMEREN_2_POLJA: f32 = 0.5;
static POMERANJE_F_PIJUNA_PRE_ROKADE: f32 = 0.5;

static TOP_NA_SREDINI: f32 = 0.125;
static TOP_IZA_PIJUNA_NA_6_7_ranku: f32 = 0.125;
static TOP_NA_FAJLU_GDE_NEMA_SOPSTVENOG_PIJUNA: f32 = 0.25;
static TOP_NA_ISTOM_RANKU_KAO_KRALJ: f32 = 0.125;
static TOP_NA_ISTOM_FAJLU_KAO_KRALJ: f32 = 0.125;
static TOP_NA_ISTOM_RANKU_FAJLU_KAO_PROTIVNICKA_KRALJICA: f32 = 0.125;

static GURANJE_G_PIJUNA_2_POLJA_AKO_KRALJ_NIJE_NA_DRUGOJ_STRANI: f32 = 1.25;
static GURANJE_B_PIJUNA_2_POLJA_AKO_JE_KRALJ_NA_KRALJICINOJ_STRANI: f32 = 1.25;
static GURANJE_B_PIJUNA_1_POLJE_AKO_JE_KRALJ_NA_KRALJICINOJ_STRANI: f32 = 0.5;
static POJEDEN_PIJUN_ISPRED_KRALJA: f32 = 0.75;
static POJEDEN_PIJUN_ISPRED_SOPSTVENOG_KRALJA: f32 = 0.75;
static PIJUN_ISPRED_KRALJA_GURNUT_VISE_OD_JEDNOG_POLJA_NAKON_ROKADE: f32 = 0.75;
static PIJUN_ISPRED_KRALJA_GURNUT_JEDNO_POLJE_NAKON_ROKADE: f32 = 0.25;

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

    pub fn nerekursivno_evaluiraj_poziciju_sa_proverom_mata(&self, nekompresirana_tabla: &Nekompresirana_tabla) -> f32{
        let beli_je_na_potezu: bool = self.beli_je_na_potezu(); 
        if self.igrac_je_u_sahu(nekompresirana_tabla){
            if self.nema_legalnih_poteza(nekompresirana_tabla){
                return vrednost_mata(beli_je_na_potezu);
            }
        }
        self.nerekursivno_evaluiraj_poziciju(nekompresirana_tabla)
    }     

    pub fn nerekursivno_evaluiraj_poziciju(&self, nekompresirana_tabla: &Nekompresirana_tabla) -> f32 {
        let beli_je_na_potezu: bool = self.beli_je_na_potezu(); 
        if self.pre_koliko_poteza_je_50_move_rule_pomeren() >= 50 {
            return 0.0
        } 

        let tabla_pijuna: Tabla_pijuna = self.to_tabla_pijuna();
        let (beli_materijal,crni_materijal,beli_ima_kraljicu,crni_ima_kraljicu) =  self.evaluacija_materijala(beli_je_na_potezu);
        let beli_kralj: File_rank = File_rank::new_iz_broja(self.bele_figure[KRALJ]);
        let crni_kralj: File_rank = File_rank::new_iz_broja(self.crne_figure[KRALJ]);

        let (beli_kralj_eval, crni_kralj_eval) = self.eval_pozicija_kralja(&tabla_pijuna, beli_materijal, crni_materijal, &beli_kralj, &crni_kralj, beli_ima_kraljicu, crni_ima_kraljicu);
        let (eval_belih_figura, eval_crnih_figura) = self.eval_pozicije_figura_podrazumeva_figure_se_nalaze_izmedju_levog_i_desnog_konja(&tabla_pijuna);
        let (eval_belih_pijuna, eval_crnih_pijuna) = self.eval_pijuna();
        let (beli_potezi, crni_potezi) = self.broj_poteza_kretanja_figura_belog_i_crnog(nekompresirana_tabla);

        let beli_eval: f32 = beli_materijal + beli_kralj_eval + eval_belih_figura + eval_belih_pijuna + beli_potezi as f32 / KOLIKO_POLJA_KRETANJA_FIGURA_VREDI_JEDAN_POEN;
        let crni_eval: f32 = crni_materijal + crni_kralj_eval + eval_crnih_figura + eval_crnih_pijuna + crni_potezi as f32 / KOLIKO_POLJA_KRETANJA_FIGURA_VREDI_JEDAN_POEN;
        beli_eval - crni_eval
    }

    fn eval_pijuna(&self) -> (f32, f32){
        let beli_eval: f32 = self.eval_centralnih_pijuna(&self.bele_figure, true);
        let crni_eval: f32 = self.eval_centralnih_pijuna(&self.crne_figure, false);
        
        (beli_eval, crni_eval)
    }

    fn centralni_pijun_dva_polja(&self, pijun: &File_rank, pijun_je_beli: bool, evaluacija: &mut f32) -> bool {
        let (treci_rank, cetvrti_rank, pijun_je_preko_pola): (u8,u8,bool);
        if pijun_je_beli {
            treci_rank = 3;
            cetvrti_rank = 4;
            pijun_je_preko_pola = pijun.rank > 4;
        } else {
            treci_rank = 6;
            cetvrti_rank = 5;
            pijun_je_preko_pola = pijun.rank < 5;
        }

        if pijun.rank == treci_rank {
            *evaluacija += CENTRALNI_PIJUN_NA_TRECEM_RANKU;
            return false
        } else if pijun.rank == cetvrti_rank{
            *evaluacija += CENTRALNI_PIJUN_NA_CETVRTOM_RANKU;
            return true
        } else if pijun_je_preko_pola {
            *evaluacija+= CENTRALNI_PIJUN_NA_DRUGOJ_STRANI_TABLE;
            return true
        } else {
            false
        }
    }

   fn eval_centralnih_pijuna(&self, figure: &[u8;16], bele_figure: bool) -> f32 {
    let e_pijun: File_rank = File_rank::new_iz_broja(figure[E_PIJUN]);
    let d_pijun: File_rank = File_rank::new_iz_broja(figure[D_PIJUN]);

    let mut eval: f32 = 0.0;
    let e_pijun_2_polja_pomeren: bool = self.centralni_pijun_dva_polja(&e_pijun, bele_figure, & mut eval);
    let d_pijun_2_polja_pomeren: bool = self.centralni_pijun_dva_polja(&d_pijun, bele_figure, &mut eval);
    
    if !e_pijun_2_polja_pomeren && !d_pijun_2_polja_pomeren {
        eval += -NIJEDAN_CENTRALNI_PIJUN_NIJE_POMEREN_2_POLJA;
    }
    eval
   }

    fn eval_pozicije_figura_podrazumeva_figure_se_nalaze_izmedju_levog_i_desnog_konja(&self,
    tabla_pijuna: &Tabla_pijuna) -> (f32, f32) {
        let mut bela_evaluacija: f32 = 0.0;
        let mut crna_evaluacija: f32 = 0.0;

        let mut i: usize = LEVI_KONJ;
        while i<= DESNI_KONJ {
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
        let (beli_topovi, crni_topovi): (f32, f32) = self.eval_topova_obe_boje(tabla_pijuna);

        (bela_evaluacija + beli_konj + beli_topovi,  crna_evaluacija + crni_konj + crni_topovi)
        
    }

    fn eval_topova_obe_boje(&self, tabla_pijuna: &Tabla_pijuna) -> (f32, f32) {
        let beli: f32 = self.eval_topova(true, tabla_pijuna);
        let crni: f32 = self.eval_topova(false, tabla_pijuna);
        (beli, crni)
    }

    fn eval_topova(&self, igrac_je_beo: bool, tabla_pijuna: &Tabla_pijuna) -> f32{
        let figure: &[u8;16];
        if igrac_je_beo{
            figure = &self.bele_figure;
        } else {
            figure = &self.crne_figure;
        }

        let mut eval_topova: f32 = 0.0;
        eval_topova += self.eval_jednog_topa(figure, LEVI_TOP, tabla_pijuna, igrac_je_beo);
        eval_topova += self.eval_jednog_topa(figure, DESNI_TOP, tabla_pijuna, igrac_je_beo);

        eval_topova
    }

    fn eval_jednog_topa(&self, figure: &[u8;16], redni_broj_topa: usize, tabla_pijuna: &Tabla_pijuna, igrac_je_beo: bool) -> f32{

        let mut eval_topa: f32 = 0.0;
        if !Tabla::figura_je_pojedena(figure, redni_broj_topa){
            let (rank,file): (u8,u8) = crate::broj_to_rank_file(figure[redni_broj_topa]);
            if file == C_FILE || file == D_FILE || file == E_FILE || file == F_FILE {
                eval_topa += TOP_NA_SREDINI;
            } 

            eval_topa += self.bonus_za_x_ray_topa(file, rank, igrac_je_beo);

            if igrac_je_beo{
                eval_topa += self.evaluacija_belog_topa_na_osnovu_fajla_na_kom_se_nalazi(tabla_pijuna, &File_rank::new_iz_broja(figure[redni_broj_topa]));
            } else {
                eval_topa += self.evaluacija_crnog_topa_na_osnovu_fajla_na_kom_se_nalazi(tabla_pijuna, &File_rank::new_iz_broja(figure[redni_broj_topa]));
            }
        }

        eval_topa
    }

    fn evaluacija_belog_topa_na_osnovu_fajla_na_kom_se_nalazi(&self, tabla_pijuna: &Tabla_pijuna, top: &File_rank)->f32{
        let mut i: u8 = 2;
        while i<=5{
            if tabla_pijuna.pijun_bele_boje(i, top.file){
                return 0.0;
            }
            i += 1;
        }

        while i<=8{
            if tabla_pijuna.pijun_bele_boje(i, top.file){
                return TOP_IZA_PIJUNA_NA_6_7_ranku
            }
            i += 1;
        }

        TOP_NA_FAJLU_GDE_NEMA_SOPSTVENOG_PIJUNA
    }

    fn evaluacija_crnog_topa_na_osnovu_fajla_na_kom_se_nalazi(&self, tabla_pijuna: &Tabla_pijuna, top: &File_rank) -> f32{
        let mut i: u8 = 7;
        while i>= 4 {
            if tabla_pijuna.pijun_crne_boje(i, top.file){
                return 0.0;
            }
            i -= 1;
        }

        while i>=1 {
            if tabla_pijuna.pijun_crne_boje(i,top.file){
                return TOP_IZA_PIJUNA_NA_6_7_ranku
            }
            i -= 1;
        }

        TOP_NA_FAJLU_GDE_NEMA_SOPSTVENOG_PIJUNA
    }

    fn bonus_za_x_ray_topa(&self, file_topa: u8, rank_topa: u8, igrac_je_beo: bool) -> f32 {
        let mut bonus_eval: f32 = 0.0;
        let protivnicki_kralj: File_rank;
        let protivnicke_figure: &[u8;16];
        if igrac_je_beo{
            protivnicki_kralj = File_rank::new_iz_broja(self.crne_figure[KRALJ]);
            protivnicke_figure = &self.crne_figure;
        } else {
            protivnicki_kralj = File_rank::new_iz_broja(self.bele_figure[KRALJ]);
            protivnicke_figure = &self.bele_figure;
        }

        if !Tabla::figura_je_pojedena(protivnicke_figure, KRALJICA){
            let (rank_kraljice, file_kraljice) = crate::broj_to_rank_file(protivnicke_figure[KRALJICA]);
            if rank_kraljice == rank_topa || file_kraljice == file_topa {
                bonus_eval += TOP_NA_ISTOM_RANKU_FAJLU_KAO_PROTIVNICKA_KRALJICA;
            }
        }
        
        if protivnicki_kralj.rank == rank_topa{
            bonus_eval += TOP_NA_ISTOM_RANKU_KAO_KRALJ;
        }
        if protivnicki_kralj.file == file_topa{
            bonus_eval += TOP_NA_ISTOM_FAJLU_KAO_KRALJ;
        }

        bonus_eval
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



    fn eval_pozicija_kralja(&self, tabla_pijuna: &Tabla_pijuna, beli_materijal: f32, crni_materijal: f32, beli_kralj: &File_rank, crni_kralj: &File_rank, beli_ima_kraljicu: bool, crni_ima_kraljicu: bool) -> (f32, f32){
        let mut eval_beli_kralj: f32 = 0.0;
        if (crni_materijal > MATERIJAL_KAD_JE_PARTIJA_U_ZAVRSNICI) || crni_ima_kraljicu{
            eval_beli_kralj += self.evaluacija_kralja_protivnik_ima_dosta_materijala(tabla_pijuna, true, beli_kralj)
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
            eval_crni_kralj += self.evaluacija_kralja_protivnik_ima_dosta_materijala(tabla_pijuna, false, crni_kralj);
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

    fn eval_figure_prvi_rank_drugi_rank_treci_rank_rokada_moguca_protivnicki_kralj(&self, kralj: &File_rank, kralj_je_beo: bool) -> (f32,&[u8;16],u8,u8,u8,bool,File_rank){
        let mut eval: f32 = 0.0;
        let figure: &[u8;16];
        let (prvi_rank, drugi_rank, treci_rank): (u8,u8,u8);
        let rokada_moguca: bool;
        let protivnicki_kralj: File_rank;

        if kralj_je_beo {
            if kralj.rank > 2 {
                eval -= KRALJ_NA_OTVORENOM;
            } 
            figure = &self.bele_figure;
            protivnicki_kralj = File_rank::new_iz_broja(self.crne_figure[KRALJ]);
            prvi_rank = 1;
            drugi_rank = 2;
            treci_rank = 3;
            rokada_moguca = self.rokada().nijedna_rokada_ove_boje_nije_moguca(true);
        } else {
            if kralj.rank < 7 {
                eval -= KRALJ_NA_OTVORENOM;
            }
            prvi_rank = 8;
            drugi_rank = 7;
            treci_rank = 6;
            figure = &self.crne_figure;
            protivnicki_kralj = File_rank::new_iz_broja(self.bele_figure[KRALJ]);
            rokada_moguca = self.rokada().nijedna_rokada_ove_boje_nije_moguca(false);
        }
        (eval, figure, prvi_rank, drugi_rank, treci_rank, rokada_moguca, protivnicki_kralj)
    }

    fn evaluacija_kralja_protivnik_ima_dosta_materijala(&self, tabla_pijuna: &Tabla_pijuna, kralj_je_beo: bool, kralj: &File_rank) -> f32 {
        let (eval, figure, prvi_rank, drugi_rank, treci_rank, rokada_moguca, protivnicki_kralj) = self.eval_figure_prvi_rank_drugi_rank_treci_rank_rokada_moguca_protivnicki_kralj(kralj, kralj_je_beo);
        let mut eval: f32 = eval;

        if (kralj.file == E_FILE || kralj.file == D_FILE || kralj.file == F_FILE){
            if self.rokada().nijedna_rokada_ove_boje_nije_moguca(kralj_je_beo){
                eval -= KRALJ_NA_SREDINI_I_NEMA_ROKADE;
            } else {
                eval -= KRALJ_NA_SREDINI;
            }
        } 

        eval += self.guranje_pijuna_ako_kralj_nije_na_drugoj_strani(figure, kralj, prvi_rank, drugi_rank, treci_rank);
        eval += self.eval_kralja_suprotna_rokada(tabla_pijuna, kralj, kralj_je_beo);
        eval += self.eval_kralja_posle_rokade_na_osnovu_pijuna_ispred_sebe(figure, kralj, drugi_rank, treci_rank, kralj_je_beo);
        eval
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

    fn guranje_pijuna_ako_kralj_nije_na_drugoj_strani(&self, figure: &[u8;16], kralj: &File_rank,
    prvi_rank: u8, drugi_rank: u8, treci_rank: u8) -> f32 {
       
        let (rank_g_pijuna, _) = crate::broj_to_rank_file(figure[G_PIJUN]);

        if kralj.file == E_FILE || kralj.file >= G_FILE{
            if Tabla::figura_je_pojedena(figure, G_PIJUN){
                return -GURANJE_G_PIJUNA_2_POLJA_AKO_KRALJ_NIJE_NA_DRUGOJ_STRANI;
            }
            if rank_g_pijuna == drugi_rank{
                return 0.0;
            }

            if rank_g_pijuna != treci_rank {
                return -GURANJE_G_PIJUNA_2_POLJA_AKO_KRALJ_NIJE_NA_DRUGOJ_STRANI;
            }
            
        }

        let (rank_b_pijuna,_) = crate::broj_to_rank_file(figure[B_PIJUN]);
        if kralj.file <= C_FILE {
            if Tabla::figura_je_pojedena(figure, B_PIJUN) {
                return -GURANJE_B_PIJUNA_2_POLJA_AKO_JE_KRALJ_NA_KRALJICINOJ_STRANI;
            }  

            if rank_b_pijuna == drugi_rank{
                return 0.0;
            } 

            if rank_b_pijuna == treci_rank {
                return -GURANJE_B_PIJUNA_1_POLJE_AKO_JE_KRALJ_NA_KRALJICINOJ_STRANI;
            }
            return -GURANJE_B_PIJUNA_2_POLJA_AKO_JE_KRALJ_NA_KRALJICINOJ_STRANI;
        }
        
        0.0
    }

    fn eval_kralja_suprotna_rokada_na_osnovu_protivnickih_pijuna(&self, tabla_pijuna: &Tabla_pijuna, file_kralja: u8, kralj_je_beo: bool) -> f32{
        if kralj_je_beo{
            let mut eval_za_pijune_ispred_belog: i8 = -8;
            let mut i: u8 = 2;
             while i<=7 {
                if tabla_pijuna.pijun_crne_boje(i, file_kralja){
                eval_za_pijune_ispred_belog = -(7 - i as i8);
                break;
            }
                i+=1;
            }
            let mut eval: f32 = eval_za_pijune_ispred_belog as f32 / 8.0;
            if Tabla::figura_je_pojedena(&self.bele_figure, A_PIJUN-1+file_kralja as usize) ||
            Tabla::pijun_je_promovisan(self.bele_figure[A_PIJUN-1+file_kralja as usize]){
                eval -= POJEDEN_PIJUN_ISPRED_KRALJA;
            }
            return eval;
        } else {

            let mut eval_za_pijuna_ispred_crnog: i8 = -8;
            let mut i: u8 = 7;
            while i>=2 {
                    if tabla_pijuna.pijun_bele_boje(i, file_kralja){
                         eval_za_pijuna_ispred_crnog = -(i as i8 - 2);
                    }
                    i-=1;
            }

            let mut eval: f32 = eval_za_pijuna_ispred_crnog as f32 / 8.0;
            if Tabla::figura_je_pojedena(&self.crne_figure, A_PIJUN-1+file_kralja as usize) ||
             Tabla::pijun_je_promovisan(self.crne_figure[A_PIJUN-1+file_kralja as usize]){
                eval -= POJEDEN_PIJUN_ISPRED_KRALJA;
            }
            return eval
        }
    }

    fn eval_kralja_posle_rokade_na_osnovu_pijuna_ispred_sebe(&self, figure: &[u8;16], kralj: &File_rank, drugi_rank: u8, treci_rank: u8, kralj_je_beo: bool) -> f32{
        if kralj.file < G_FILE && kralj.file > C_FILE {
            return 0.0
        }

        let mut eval: f32 = 0.0;
        let pijun_ispred_kralja: File_rank = File_rank::new_iz_broja(figure[A_PIJUN-1 + kralj.file as usize]);
        if !Tabla::pijun_postoji(figure, A_PIJUN-1 + kralj.file as usize){
            eval -= POJEDEN_PIJUN_ISPRED_SOPSTVENOG_KRALJA;
        } else if pijun_ispred_kralja. rank == treci_rank {
            if !self.lovac_boje_se_nalazi_na_polju(figure, &File_rank::new(kralj.file, drugi_rank)){
                eval -= PIJUN_ISPRED_KRALJA_GURNUT_JEDNO_POLJE_NAKON_ROKADE;
            }
        } else {
            eval -= PIJUN_ISPRED_KRALJA_GURNUT_VISE_OD_JEDNOG_POLJA_NAKON_ROKADE;
        }
        
        eval
    }

    fn eval_kralja_suprotna_rokada(&self, tabla_pijuna: &Tabla_pijuna, pozicija_kralja: &File_rank, kralj_je_beo: bool) -> f32 {
        let beli_kralj: File_rank = File_rank::new_iz_broja(self.bele_figure[KRALJ]);
        let crni_kralj: File_rank = File_rank::new_iz_broja(self.crne_figure[KRALJ]);

        if abs(crni_kralj.file as i32 - beli_kralj.file as i32) < 2{
            return 0.0
        }

        return self.eval_kralja_suprotna_rokada_na_osnovu_protivnickih_pijuna(tabla_pijuna, pozicija_kralja.file, kralj_je_beo)
    }

    pub fn materijalna_prednost_onog_ko_je_na_potezu(&self) -> f32 {
        let beli_je_na_potezu: bool = self.beli_je_na_potezu();
        let (beli_materijal, crni_materijal, _, _) = self.evaluacija_materijala(beli_je_na_potezu);
        if beli_je_na_potezu {
            beli_materijal - crni_materijal
        } else {
            crni_materijal - beli_materijal
        }
    }

    pub fn evaluacija_materijala(&self, beli_je_na_potezu: bool) -> (f32,f32, bool, bool) {
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
