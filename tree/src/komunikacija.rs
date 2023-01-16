use crate::tabla::{potez::Potez, Promocija};
use std::{io, u8};

pub trait Komunikator{
    fn posalji_primi_potez(&mut self, potez: Option<Potez>) -> Potez;
}


pub struct Konzola_sah {

}
impl Konzola_sah {
    pub fn new() -> Konzola_sah{
        Konzola_sah{}
    }
}

impl Komunikator for Konzola_sah {
    fn posalji_primi_potez(&mut self, potez: Option<Potez>) -> Potez {
        match potez {
            None => {println!("Odigrajte potez.");},
            Some(p) => {println!("Kompjuter je odigrao potez {}", p);}
        }

        let mut pocetni_rank: String = String::new();
        println!("Upisite rank polja sa kog pomerate figuru: ");
        std::io::stdin().read_line(&mut pocetni_rank).expect("Greska");
        let pocetni_rank_br: u8 = pocetni_rank.trim().parse().expect("Niste upisali broj");

        let mut pocetni_file: String = String::new();
        println!("Upisite file polja sa kog pomerate figuru: ");
        std::io::stdin().read_line(&mut pocetni_file).expect("Greska.");
        let pocetni_file_br: u8 = pocetni_file.trim().parse().expect("Niste upisali broj");

        let mut zavrsni_rank: String = String::new();
        println!("Upisite rank destinacije: ");
        std::io::stdin().read_line(&mut zavrsni_rank).expect("Greska.");
        let krajnji_rank_br: u8 = zavrsni_rank.trim().parse().expect("Niste upisali broj");

        let mut zavrsni_file: String = String::new();
        println!("Upisite file destinacije: ");
        std::io::stdin().read_line(&mut zavrsni_file).expect("Greska");
        let krajnji_file_br: u8 = zavrsni_file.trim().parse().expect("Niste upisali broj");

        let mut promocija_string: String = String::new();
        println!("Upisite 0 za promociju kraljice, 1 za topa, 2 za lovca, 3 za konja, bilo koji drugi broj ako nema promocije.");
        std::io::stdin().read_line(&mut promocija_string).expect("Greska");
        let promocija_id: i64 = promocija_string.trim().parse().expect("Greska.");

        let mut promocija: Promocija = Promocija::None;

        if promocija_id == 0 {
            promocija = Promocija::KRALJICA;
        }
        if promocija_id == 1 {
            promocija = Promocija::TOP;
        }
        if promocija_id == 2 {
            promocija = Promocija::LOVAC;
        }
        if promocija_id ==3 {
            promocija = Promocija::KONJ;
        }

        println!("Sacekajte odgovor. Wait for response...");
        Potez::new(pocetni_file_br, pocetni_rank_br, krajnji_file_br, krajnji_rank_br, promocija)
    }
}
