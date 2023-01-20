use std::{fs::File, io::{Write, Read}};

use crate::tabla::{potez::Potez, Promocija, A_FILE};

static STRING_KOJI_ODVAJA_POTEZE: &str = "\nKRAJ_POTEZA\n";
static ODVAJA_VREDNOST_OD_NAZIVA: &str = ":";


pub trait Zapisivac_partije{
    fn zapisi_partiju(&mut self, potezi_partije: &[Potez]);
}


pub struct Zapisivac_partije_u_fajl{
    ime_fajla: String,

}


impl Zapisivac_partije_u_fajl{
    pub fn new(ime_fajla: String) -> Zapisivac_partije_u_fajl {
        Zapisivac_partije_u_fajl {ime_fajla }
    }

    pub fn zapisi_partiju_u_fajl(&mut self, potezi: &[Potez]) -> std::io::Result<()>{
        let mut potezi_string: String = String::new();
        let broj_poteza: usize = potezi.len();
        if broj_poteza > 0 {
            potezi_string = self.jedan_potez_u_string(&potezi[0]);
        }

        for i in 1..broj_poteza {
            potezi_string = potezi_string + STRING_KOJI_ODVAJA_POTEZE;
            potezi_string = potezi_string + &self.jedan_potez_u_string(&potezi[i]);
        }

        let mut file: File = File::create(&self.ime_fajla)?;
        file.write_all(potezi_string.as_bytes())?;
        return Ok(())
    }

    pub fn preuzmi_poteze_iz_fajla(&mut self, ime_fajla: &str) -> std::io::Result<Vec<Potez>>{
        let potezi_string: String = self.preuzmi_string_iz_fajla(ime_fajla)?;
        let potezi: core::str::Split<&str> = potezi_string.split(STRING_KOJI_ODVAJA_POTEZE);
        
        let mut za_return: Vec<Potez> = Vec::new();
        for potez_str in potezi {
            match self.iz_stringa_u_potez(potez_str){
                None => {eprintln!("Dogodila se greska pri parsiranju poteza."); return Ok(Vec::new())},
                Some(_potez) => {
                    za_return.push(_potez);
                }
            }
        }
        
        Ok(za_return)
    }



    pub fn preuzmi_string_iz_fajla(&mut self, ime_fajla: &str) -> 
    std::io::Result<String>{
        let mut file: File = File::open(ime_fajla)?;
        let mut buffer: String = String::new();
        file.read_to_string(&mut buffer)?;

        return Ok(buffer)
    }


    pub fn jedan_potez_u_string(&mut self, potez: &Potez) -> String {
        let mut string: String = String::new();
        string = "start_rank:".to_owned() + &potez.start_rank.to_string() + "\n";
        string = string + "start_file:" + &potez.start_file.to_string() + "\n";
        string = string + "end_rank:" + &potez.rank_destinacije.to_string() + "\n";
        string = string + "end_file:" + &potez.file_destinacije.to_string() + "\n";
        match &potez.promocija {
            &Promocija::None => {string = string + "promocija:None";},
            &Promocija::KRALJICA => {string = string + "promocija:KRALJICA";},
            &Promocija::TOP => {string = string + "promocija:TOP";},
            &Promocija::LOVAC => {string = string + "promocija:LOVAC";},
            &Promocija::KONJ => {string = string + "promocija:KONJ";}
        }  
        string = string + "\n";
        string
    }

    pub fn iz_stringa_u_potez(&self, potez_string: &str) -> Option<Potez> {
        let mut potez_informacije: core::str::Split<&str> = potez_string.split("\n");
        let start_rank = daj_vrednost(potez_informacije.next()?);
        let start_file = daj_vrednost(potez_informacije.next()?); 
        let end_rank = daj_vrednost(potez_informacije.next()?);
        let end_file = daj_vrednost(potez_informacije.next()?);
        let promocija = Promocija::iz_stringa(daj_vrednost(potez_informacije.next()?));
        let mut potez: Potez = Potez::new(A_FILE, 1, A_FILE, 1, promocija);

        if prvi_trimovan_karakter_je_broj(start_rank) {
            potez.start_rank = daj_prvu_cifru_unsafe(start_rank);
        } else {
            return None;
        }
        if prvi_trimovan_karakter_je_broj(start_file){
            potez.start_file = daj_prvu_cifru_unsafe(start_file);
        } else {
            return None
        }

        if prvi_trimovan_karakter_je_broj(end_rank){
            potez.rank_destinacije = daj_prvu_cifru_unsafe(end_rank);
        } else {
            return None
        }

        if prvi_trimovan_karakter_je_broj(end_file){
            potez.file_destinacije = daj_prvu_cifru_unsafe(end_file);
        } else {
            return None
        }
    
        Some(potez)
    }
}

fn prvi_trimovan_karakter_je_broj(string: &str) -> bool {
    let b = string.trim().as_bytes();
    if b.len() == 0 {
        return false
    }
    b[0] >= 0x30 && b[0] <= 0x39
}

fn daj_prvu_cifru_unsafe(string: &str) -> u8 {
    let b = string.trim().as_bytes();
    b[0] - 0x30
}

fn daj_vrednost<'a>(informacija_o_potezu: &'a str) -> &'a str{
    let iter = informacija_o_potezu.split(ODVAJA_VREDNOST_OD_NAZIVA);
    iter.last().expect("Greska prilikom citanja vrednosti poteza.").trim()
}


impl Zapisivac_partije for Zapisivac_partije_u_fajl {
    fn zapisi_partiju(&mut self, potezi_partije: &[Potez]){
        self.zapisi_partiju_u_fajl(potezi_partije);
    }
}


#[cfg(test)]
mod permanencija_test{
    use crate::tabla::{A_FILE, B_FILE, C_FILE, potez::Potez, Promocija};

    use super::Zapisivac_partije_u_fajl;


    #[test]
    fn test_1(){
        let mut zapisivac = Zapisivac_partije_u_fajl::new("log_partije_proba1.txt".to_owned());
        zapisivac.zapisi_partiju_u_fajl(&vec![Potez::new(B_FILE, 2, B_FILE, 4, Promocija::None), 
        Potez::new(A_FILE, 7, A_FILE, 6, Promocija::None),
        Potez::new(C_FILE, 2, C_FILE, 4, Promocija::None)
        ]);
        let potezi = Zapisivac_partije_u_fajl::new("log_partije_proba1.txt".to_owned()).preuzmi_poteze_iz_fajla("log_partije_proba1.txt").unwrap();
        assert_eq!(3, potezi.len());
    }
}