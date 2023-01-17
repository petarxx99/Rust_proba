use crate::tabla::{potez::Potez, Promocija};



pub trait Enkoder_poteza{
    fn enkoduj_potez(&self, potez: &Potez) -> Vec<u8>;
    fn desifruj_potez(&self, potez: &[u8]) -> Potez;
}

pub struct Trobajtni_enkoder_poteza{

}

impl Enkoder_poteza for Trobajtni_enkoder_poteza {
    fn enkoduj_potez(&self, potez: &Potez) -> Vec<u8> {
        let prvi_bajt: u8 = potez.start_rank * 10 + potez.start_file;
        let drugi_bajt: u8 = potez.rank_destinacije * 10 + potez.file_destinacije;
        let treci_bajt: u8;
        match &potez.promocija {
            &Promocija::None => {treci_bajt = 255;},
            &Promocija::KRALJICA => {treci_bajt = 0;},
            &Promocija::TOP => {treci_bajt = 1;},
            &Promocija::LOVAC => {treci_bajt = 2;},
            &Promocija::KONJ => {treci_bajt = 3;}
        }
        vec![prvi_bajt, drugi_bajt, treci_bajt]
    }

    fn desifruj_potez(&self, potez: &[u8]) -> Potez {
        let start_rank = (potez[0] / 10) as u8;
        let start_file = (potez[0] % 10) as u8;
        let end_rank = (potez[1] / 10) as u8;
        let end_file = (potez[1] % 10) as u8;
        let promocija: Promocija;
        if potez[2] == 0 {
            promocija = Promocija::KRALJICA;
        } else if potez[2] == 1 {
            promocija = Promocija::TOP;
        } else if potez[2] == 2 {
            promocija = Promocija::LOVAC;
        } else if potez[2] == 3 {
            promocija = Promocija::KONJ;
        } else {
            promocija = Promocija::None;
        }

        Potez::new(start_file, start_rank, end_file, end_rank, promocija)
    }
}

impl Trobajtni_enkoder_poteza{
    pub fn new() -> Trobajtni_enkoder_poteza {
        Trobajtni_enkoder_poteza { }
    }
}