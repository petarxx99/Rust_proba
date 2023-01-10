
use std::boxed::Box;

use self::{konj::{prirodno_kretanje_konja, konj_napada_kralja, konj_napada_polje}, 
kralj::{prirodno_kretanje_kralja, kralj_napada_kralja, kralj_napada_polje}, 
lovac::{prirodno_kretanje_lovca,lovac_napada_kralja, lovac_napada_polje},
 kraljica::{prirodno_kretanje_kraljice, kraljica_napada_kralja, kraljica_napada_polje},
  pijun::{prirodno_kretanje_pijuna, pijun_napada_kralja, pijun_napada_polje},
   top::{polja_na_koja_ide_top, top_napada_kralja, top_napada_polje}};

use super::{Rokada, Tabla, Ima_podatke_o_tabli, Figura};


mod konj;
mod top;
mod kraljica;
mod lovac;
mod kralj;
mod pijun;
pub(crate) mod figure;

pub struct Figura_interfejs<T> 
where T: Ima_podatke_o_tabli
{
    pub prirodno_kretanje: 
    fn(
        tabla: &T,
        polje_na_kom_se_nalazim: u8,
        rokada: &Rokada, 
        fajl_pijuna_2_polja: Option<u8>, ja_sam_beli: bool) -> Vec<u8>,
        

    pub napadam_kralja: fn(tabla: &T, polje_na_kom_se_nalazim: u8, kralj_je_beo: bool) -> bool,
    pub napadam_polje: fn(polje: u8, tabla: &T, polje_na_kom_se_nalazim: u8, ja_sam_beli: bool) -> bool,
}


impl<T> Figura_interfejs<T> 
where T: Ima_podatke_o_tabli
{
    pub fn new(prirodno_kretanje: fn(
        tabla: &T,
        polje_na_kom_se_nalazim: u8,
        rokada: &Rokada, 
        fajl_pijuna_2_polja: Option<u8>, ja_sam_beli: bool) -> Vec<u8>,
       
        napadam_kralja: fn(tabla: &T, polje_na_kom_se_nalazim: u8, kralj_je_beo: bool) -> bool,
        napadam_polje: fn(polje: u8, tabla: &T, polje_na_kom_se_nalazim: u8, kralj_je_beli: bool) -> bool,
    ) -> Figura_interfejs<T>{
        Figura_interfejs{prirodno_kretanje, napadam_kralja, napadam_polje}
    }


}

