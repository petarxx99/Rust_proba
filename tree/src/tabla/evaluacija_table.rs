use super::{Figura, Tabla};



impl Tabla {

    pub fn nerekursivno_evaluiraj_poziciju(&self) -> i8 {
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

        (evaluacija_belog - evaluacija_crnog) as i8 
    }

}