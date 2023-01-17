use crate::proba_sah_drveta::vrednost_mata;

use super::{Figura, Tabla, nekompresirana_tabla::Nekompresirana_tabla};

static PREDNOST_POTEZA: f32 = 0.2;


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
        let mut eval: f32 =  self.evaluacija_materijala(beli_je_na_potezu);

        eval
    }

    fn evaluacija_materijala(&self, beli_je_na_potezu: bool) -> f32 {
        let mut evaluacija_belog: f32 = 0f32;
        let mut evaluacija_crnog: f32 = 0f32;

        let mut i:usize = 1; /* 0 je kralj, on je uvek na tabli. */
        while i<16 {
            match Tabla::koja_figura_se_nalazi_u_bitu(&self.bele_figure, i){
                None => {},
                Some(figura) => {evaluacija_belog += figura.vrednost();}
            }

            match Tabla::koja_figura_se_nalazi_u_bitu(&self.crne_figure, i){
                None => {},
                Some(figura) => {evaluacija_crnog += figura.vrednost();}
            }
            i += 1;
        }

        if beli_je_na_potezu {
            evaluacija_belog += PREDNOST_POTEZA;
        } else {
            evaluacija_crnog += PREDNOST_POTEZA;
        }

        evaluacija_belog - evaluacija_crnog
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
