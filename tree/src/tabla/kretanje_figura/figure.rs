use crate::tabla::{kretanje_figura
::{konj::{prirodno_kretanje_konja,  konj_napada_polje}, 
kralj::{prirodno_kretanje_kralja, kralj_napada_polje}, 
lovac::{prirodno_kretanje_lovca,lovac_napada_polje},
 kraljica::{prirodno_kretanje_kraljice,  kraljica_napada_polje},
  pijun::{prirodno_kretanje_pijuna, pijun_napada_polje},
   top::{polja_na_koja_ide_top, top_napada_polje}}, Figura, Tabla, A_FILE, H_FILE, Ima_podatke_o_tabli, nekompresirana_tabla::Nekompresirana_tabla, File_rank};

use super::{Figura_interfejs, pijun::{pijun_moze_doci_na_polje, potezi_pijuna}, kralj::{kralj_moze_doci_na_polje, potezi_kralja}, kraljica::{kraljica_moze_doci_na_polje, potezi_kraljice},
     lovac::{lovac_moze_doci_na_polje, potezi_lovca},
konj::{konj_moze_doci_na_polje, potezi_konja},
top::{top_moze_doci_na_polje, potezi_topa}};



impl Figura {
    pub fn iz_niza_u_figure_interfejs(figure: &[u8;16], redni_broj_figure: usize) -> Option<Figura_interfejs<Nekompresirana_tabla>>{
        let figura: Option<Figura> = Tabla::koja_figura_se_nalazi_u_bitu(figure, redni_broj_figure);
        match figura {
            None => None,
            Some(f) => Some(Self::napravi_figure_interfejs(f))
        }
    

    }

    pub fn napravi_figure_interfejs(figura: Figura) -> Figura_interfejs<Nekompresirana_tabla>{
        match figura {
            Figura::KONJ => Figura_interfejs::new(potezi_konja, prirodno_kretanje_konja, konj_napada_polje, konj_moze_doci_na_polje),
            Figura::KRALJ => Figura_interfejs::new(potezi_kralja, prirodno_kretanje_kralja, kralj_napada_polje, kralj_moze_doci_na_polje),
            Figura::LOVAC => Figura_interfejs::new(potezi_lovca, prirodno_kretanje_lovca, lovac_napada_polje, lovac_moze_doci_na_polje),  
            Figura::KRALJICA => Figura_interfejs::new(potezi_kraljice, prirodno_kretanje_kraljice, kraljica_napada_polje, kraljica_moze_doci_na_polje),
            Figura::TOP => Figura_interfejs::new(potezi_topa, polja_na_koja_ide_top, top_napada_polje, top_moze_doci_na_polje),
            Figura::PIJUN => Figura_interfejs::new(potezi_pijuna, prirodno_kretanje_pijuna, pijun_napada_polje, pijun_moze_doci_na_polje)
        }
    }

    pub fn iz_niza_u_figure_interfejse(figure: &[u8;16]) -> Vec<Figura_interfejs<Nekompresirana_tabla>>{
        let mut rezultat: Vec<Figura_interfejs<Nekompresirana_tabla>> = Vec::new();
        for i in 0..16 {
            match Self::iz_niza_u_figure_interfejs(figure, i) {
                None => {},
                Some(f_interfejs) => rezultat.push(f_interfejs)
            }
        }

        rezultat
    }


   
}



pub fn abs(broj: i32) -> u32 {
    if broj<0 {
        return (-broj) as u32
    }
    broj as u32
}

pub fn ako_su_validni_dodaj_u_vektor(vektor: &mut Vec<File_rank>, rank: i32, file: i32){
    if rank >= 1 && rank <=8 && file>=A_FILE as i32 && file <=H_FILE as i32{
       vektor.push(File_rank{file: file as u8, rank: rank as u8});
    }
}


pub fn polje_je_validno(polje: &File_rank) -> bool {
     polje.rank >= 1 && polje.rank <= 8 && polje.file >= A_FILE && polje.file <= H_FILE
}
