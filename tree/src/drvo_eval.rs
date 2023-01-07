use std::vec::Vec;
use std::boxed::Box;



pub struct Grana{
    deca: Vec<Grana>,
    sopstvena_vrednost: i16,
    izracunata_vrednost: Option<i32>
}


impl Grana {
    pub fn sam_izracunaj_svoju_vrednost(&self) -> i16 {
        self.sopstvena_vrednost
    }
}

impl Grana{
    pub fn new(sopstvena_vrednost: i16) -> Grana {
        Grana {
            deca: Vec::new(),
            sopstvena_vrednost,
            izracunata_vrednost: None
        }
    }

    
    pub fn napravi_grane(mut dubina_drveta: u8, sirina_drveta: u8, mut number: u16) -> Grana{
        let (random, baza) = Grana::napravi_sve_random_podatke(dubina_drveta, sirina_drveta, number);

        let mut koren = Grana::new(1);
        koren.napravi_grane_recursive(dubina_drveta - 1, sirina_drveta, random, baza);
        koren
    }

    fn napravi_grane_recursive(&mut self, dubina_dece: u8, broj_dece: u8, mut random: u32, baza: u16){
        if dubina_dece <= 0 {
            return;
        }
        
        let broj = random % 20;

        for i in 0..broj_dece {
            let mut dete = Grana::new((broj+i as u32) as i16);
            self.deca.push(dete);
        }

        let mut i=0;
        for dete in & mut self.deca {
            let novi_random = (baza * (i+5) as u16) as u32 + (random / baza as u32);
            dete.napravi_grane_recursive(dubina_dece-1, broj_dece, novi_random, baza);
            i += 1;
        }
    }
   
}



impl Grana{

   

    fn napravi_sve_random_podatke(mut dubina_drveta: u8, mut sirina_drveta: u8, mut number: u16) ->
    (u32, u16){
        let random = Grana::random(dubina_drveta, sirina_drveta, number);
        let mut baza = 10;

        if number > 3 {
            baza = number & ((1<<7) -1);
        }

        println!("random = {}, baza = {}", random, baza);
        (random, baza)
    }

    fn random(br1: u8, br2: u8, mut number: u16) -> u32 {
        let treca = number & (1 << 3);
        let druga = number & (1<<2);
        
        let mut izmenjen = number << 4;
        izmenjen |= (br1 ^ br2) as u16;

    
        let rezultat: u32 = izmenjen as u32 * (druga*5 + treca*3 + 7 + br1 as u16+ br2 as u16) as u32;
        rezultat
    } 
   
}


impl std::fmt::Display for Grana{
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result{
        let mut podaci_o_svim_granama = self.podaci_o_celom_drvetu();
        return write!(formatter, "{}", podaci_o_svim_granama)
    }
}


impl Grana {
    pub  fn daj_podatke_o_sebi(&self) -> String {
        match &self.izracunata_vrednost {
            Some(izracunata_vrednost) => format!("sopstvena v: {}, izracunata v: {}, broj dece: {}\n",
             self.sopstvena_vrednost, izracunata_vrednost, self.deca.len()),

            None => format!("sopstvena v: {}, broj dece: {}\n", 
            self.sopstvena_vrednost, self.deca.len())
        }
        
    }

  pub fn podaci_o_deci_recursive(&self, string: &mut String, dubina: u16){
        string.push_str(&format!("dubina {} \n", dubina));
     //   println!("Dubina {}\n", dubina);

        for dete in &self.deca{
            let tabovi = Grana::napravi_tabove(dubina);
      //      println!("{} vrednost = {}, broj dece = {}\n", &tabovi, dete.sopstvena_vrednost, dete.deca.len());

            string.push_str(&tabovi);
            string.push_str(&format!("vrednost:{}\n", dete.sopstvena_vrednost));
        }

        for dete in &self.deca {
            dete.podaci_o_deci_recursive(string, dubina+1);
        }
    } 

    fn napravi_tabove(broj_tabova: u16) -> String {
        let mut rezultat = String::new();
        for _ in 0..broj_tabova {
            rezultat.push_str("\t");
        }   
        rezultat
    }    

    pub fn podaci_o_celom_drvetu(&self) -> String {
        let mut podaci_o_svim_granama = self.daj_podatke_o_sebi();
        self.podaci_o_deci_recursive(&mut podaci_o_svim_granama, 1);
        podaci_o_svim_granama
    }

    
}



impl Grana {
    pub fn izracunaj(&mut self, ja_volim_vise: bool) -> i16 {
        self.izracunaj_rekursivno(&None, ja_volim_vise)
    }

    fn izracunaj_rekursivno(&mut self, vrednost_koju_on_ima_u_dzepu: &Option<i16>, ja_volim_vise:  bool) -> i16 {
        if self.deca.len() == 0{
            return self.sam_izracunaj_svoju_vrednost();
        }

        let mut vrednost_mog_najboljeg_poteza: Option<i16> = None;
        for dete in &mut self.deca {
            let vrednost_mog_poteza = dete.izracunaj_rekursivno(&vrednost_mog_najboljeg_poteza, !ja_volim_vise);
            if Grana::protivnik_se_zajebo(vrednost_koju_on_ima_u_dzepu, vrednost_mog_poteza, ja_volim_vise){
                return vrednost_mog_poteza;
            } 
            Grana::updejtuj_najbolji_potez(& mut vrednost_mog_najboljeg_poteza, vrednost_mog_poteza, ja_volim_vise);
        }

        vrednost_mog_najboljeg_poteza.unwrap()
    }


    fn updejtuj_najbolji_potez(stari: & mut Option<i16>, novi: i16, ja_volim_vise: bool){
        match stari {
            Some(_stara_vrednost) => {
                if ja_volim_vise && (novi > *_stara_vrednost) {
                    *_stara_vrednost = novi;
                } 

                if !ja_volim_vise && (novi < *_stara_vrednost) {
                    *_stara_vrednost = novi;
                }
            }
            None => {*stari = Some(novi);}
        }
    }

    fn protivnik_se_zajebo(drugi: &Option<i16>, moj_broj: i16, ja_volim_vise: bool) -> bool{
        if drugi.is_none() {
            return false;
        }
        
        if ja_volim_vise {
            moj_broj > drugi.unwrap()
        } else {
            moj_broj < drugi.unwrap()
        }
    }

}

pub fn proba(){
    let mut koren= Grana::napravi_grane(4, 2, 876);
    let vrednost = koren.izracunaj(true);
    println!("\n\n Dobijena vrednost = {}. \n", vrednost);
    
    let podaci = koren.podaci_o_celom_drvetu();
    println!("{}", &podaci);
}