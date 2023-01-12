
use std::boxed::Box;

use self::{konj::{prirodno_kretanje_konja, konj_napada_polje}, 
kralj::{prirodno_kretanje_kralja,  kralj_napada_polje}, 
lovac::{prirodno_kretanje_lovca, lovac_napada_polje},
 kraljica::{prirodno_kretanje_kraljice, kraljica_napada_polje},
  pijun::{prirodno_kretanje_pijuna, pijun_napada_polje},
   top::{polja_na_koja_ide_top, top_napada_polje}};

use super::{Rokada, Tabla, Ima_podatke_o_tabli, Figura};


mod konj;
mod top;
mod kraljica;
mod lovac;
mod kralj;
mod pijun;
pub(crate) mod figure;

/* Da bi potez bio legalan, 4 stavke moraju da se ispune:
1) Figura mora da moze da se krece onako kako je igrac pokusao odigrati.
2) Ne sme da bude figure izmedju pocetnog polja i polje destinacije (ovo ne vazi za konja)
3) Ne smem da jedem sopstvenu figuru
4) Kralj igraca koji odigrava potez ne sme da bude u sahu nakon odigranog poteza.
Funkcija figura_moze_doci_na_polje kombinuje prva 2.
Zasebna funkcija za 1) postoji da bi se brze nasli legalni potezi.
Funkcija napadam_polje je potrebna prilikom utvrdjivanja stavke da li ce igrac biti u sahu
nakon odigranog poteza. Zovem tu funkciju nad svim nepojedenim figurama protivnika da vidim da li 
napadaju polje na kojem se nalazi kralj. */

pub struct Figura_interfejs<T> 
where T: Ima_podatke_o_tabli
{
    pub prirodno_kretanje: 
    fn(
        tabla: &T,
        polje_na_kom_se_nalazim: u8,
        rokada: &Rokada, 
        fajl_pijuna_2_polja: Option<u8>, ja_sam_beli: bool) -> Vec<u8>,
        

    pub napada_polje: fn(tabla: &T, polje: u8, polje_na_kom_se_nalazim: u8, ja_sam_beli: bool) -> bool,
    pub figura_moze_doci_na_polje: fn(tabla: &T, polje: u8, polje_na_kom_se_nalazim: u8, ja_sam_beli: bool) -> bool,
}


impl<T> Figura_interfejs<T> 
where T: Ima_podatke_o_tabli
{
    pub fn new(prirodno_kretanje: fn(
        tabla: &T,
        polje_na_kom_se_nalazim: u8,
        rokada: &Rokada, 
        fajl_pijuna_2_polja: Option<u8>, ja_sam_beli: bool) -> Vec<u8>,
       
        napada_polje: fn(tabla: &T, polje: u8, polje_na_kom_se_nalazim: u8, kralj_je_beli: bool) -> bool,
        figura_moze_doci_na_polje: fn(tabla: &T, polje: u8, polje_na_kom_se_nalazim: u8, ja_sam_beli: bool) -> bool,
    ) -> Figura_interfejs<T>{
        Figura_interfejs{prirodno_kretanje,  napada_polje, figura_moze_doci_na_polje}
    }


}

