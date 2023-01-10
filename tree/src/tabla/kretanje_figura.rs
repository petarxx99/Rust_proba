
use std::boxed::Box;

use super::{Rokada, Tabla, Ima_podatke_o_tabli};
mod konj;
mod top;

pub struct Figura_interfejs<T> 
where T: Ima_podatke_o_tabli
{
    pub prirodno_kretanje: 
    fn(
         polje_na_kom_se_nalazim: u8,
         rokada: &Rokada, 
         fajl_pijuna_2_polja: Option<u8>) -> Vec<u8>,

    pub da_li_napadam_protivnickog_kralja: fn(tabla: &T, polje_na_kom_se_nalazim: u8) -> bool,
}


impl<T> Figura_interfejs<T> 
where T: Ima_podatke_o_tabli
{
    pub fn new(prirodno_kretanje: fn(
        polje_na_kom_se_nalazim: u8,
        rokada: &Rokada, 
        fajl_pijuna_2_polja: Option<u8>) -> Vec<u8>,

        da_li_napadam_protivnickog_kralja: fn(tabla: &T, polje_na_kom_se_nalazim: u8) -> bool
    ) -> Figura_interfejs<T>{
        Figura_interfejs{prirodno_kretanje, da_li_napadam_protivnickog_kralja}
    }


}