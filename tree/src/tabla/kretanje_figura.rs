
use std::boxed::Box;

use super::{Rokada, Tabla, Zna_da_li_su_polja_prazna};
mod konj;

pub struct Figura_interfejs<T> 
where T: Zna_da_li_su_polja_prazna
{
    pub prirodno_kretanje: fn(
         polje_na_kom_se_nalazim: u8,
         rokada: &Rokada, 
         fajl_pijuna_2_polja: Option<u8>) -> Vec<u8>,

    pub da_li_napadam_protivnickog_kralja: fn(tabla: &T) -> bool,
}


impl<T> Figura_interfejs<T> 
where T: Zna_da_li_su_polja_prazna
{
    pub fn new(prirodno_kretanje: fn(
        polje_na_kom_se_nalazim: u8,
        rokada: &Rokada, 
        fajl_pijuna_2_polja: Option<u8>) -> Vec<u8>,

        da_li_napadam_protivnickog_kralja: fn(tabla: &T) -> bool
    ) -> Figura_interfejs<T>{
        Figura_interfejs{prirodno_kretanje, da_li_napadam_protivnickog_kralja}
    }


}