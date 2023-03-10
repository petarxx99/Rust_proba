use super::Tabla;

use crate::{tabla::potez::Potez, komunikacija::Komunikator, permanencija::Zapisivac_partije};



impl Tabla{
    pub fn pocni_partiju<T,V>(dobavljac_odgovora: T, kompjuter_je_beli: bool, dubina_pretrage: u8,
    mut zapisivac_partije: V)
    where T:Komunikator, V: Zapisivac_partije
    {
        let mut dobavljac_odgovora: T = dobavljac_odgovora;
        let mut table: Vec<Tabla> = vec![Tabla::pocetna_pozicija()];
        let mut potezi: Vec<Potez> = Vec::new();
        
        if !kompjuter_je_beli {
            let prvi_potez: Potez = dobavljac_odgovora.primi_potez();
            table.push(Tabla::pocetna_pozicija().tabla_nakon_validnog_poteza(&prvi_potez));
            potezi.push(prvi_potez);
        }

        loop {
            let poslednji_indeks = table.len() -1;
            let (potez, eval) = table[poslednji_indeks].najbolji_potez_i_njegova_evaluacija(dubina_pretrage);
            
            println!("Evaluacija: {}", eval);
            match potez {
                None => {println!("Partija gotova."); break;},
                Some(_potez) => {    
                    let potez_za_slanje: Potez = _potez.to_Potez(table[poslednji_indeks].figure_koje_su_na_potezu());
                    let t: Tabla = table[poslednji_indeks].tabla_nakon_poteza_bits(&_potez);
                    table.push(t.copy());
                    potezi.push(potez_za_slanje.copy());
                    
                    let protivnicki_potez: Potez = dobavljac_odgovora.posalji_primi_potez(Some(potez_za_slanje));
                    potezi.push(protivnicki_potez.copy());
                    zapisivac_partije.zapisi_partiju(&potezi);
                    let t:Tabla = t.tabla_nakon_validnog_poteza(&protivnicki_potez);
                    table.push(t);
                }
            }
        }
        zapisivac_partije.zapisi_partiju(&potezi);
    }



    pub fn pocni_partiju2<T>(dobavljac_odgovora: T, kompjuter_je_beli: bool, dubina_pretrage: u8)
    where T:Komunikator
    {
        let mut dobavljac_odgovora: T = dobavljac_odgovora;
        let mut table: Vec<Tabla> = vec![Tabla::pocetna_pozicija()];
        
        if !kompjuter_je_beli {
            let prvi_potez: Potez = dobavljac_odgovora.posalji_primi_potez(None);
            table.push(Tabla::pocetna_pozicija().tabla_nakon_validnog_poteza(&prvi_potez));
        }

        loop {
            let poslednji_indeks = table.len() -1;
            let (potez, eval) = table[poslednji_indeks].najbolji_potez_i_njegova_evaluacija2(2);
            println!("Evaluacija: {}", eval);
            match potez {
                None => {println!("Partija gotova."); break;},
                Some(_potez) => {    
                    let potez_za_slanje: Potez = _potez.to_Potez(table[poslednji_indeks].figure_koje_su_na_potezu());
                    let t: Tabla = table[poslednji_indeks].tabla_nakon_poteza_bits(&_potez);
                    table.push(t.copy());
                    let protivnicki_potez: Potez = dobavljac_odgovora.posalji_primi_potez(Some(potez_za_slanje));
                    table.push(t.tabla_nakon_validnog_poteza(&protivnicki_potez));
                }
            }
        }
    }

}