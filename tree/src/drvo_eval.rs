use std::vec::Vec;
use std::boxed::Box;

 /* Ako mogu da odigram potez tako da evaluacija bude povoljnija po mene nego u slucaju da je
  u proslom potezu protivnik odigrao najbolji potez koji je do sada racunao,
   to znaci da bi se protivnik zajebo ako bi odigrao ovaj potez. 
   Nema potrebe dalje da se gledaju moji drugi odgovori, refutacija je pronadjena.
 Na primer, ako protivnik ima "u dzepu" potez koji mu daje prednost, nazovimo taj potez X, 
 ali on odigra potez koji mi daje mogucnost da evaluacija bude bolja po mene nego 
 da je protivnik odigrao potez X, tu se analiza ove varijacije zavrsava,
  jer je jasno da protivnik ne treba da odigra ovaj potez, zato sto ima potez X na raspolaganju.

 Zamislimo da koristim ovaj algoritam za evaluaciju saha (sto mi je i bila motivacija da napisem ovaj kod).
  Beli analizira poziciju, analizira poteze koje ima. Posle jednog poteza beli bi bio na +2.
  To je najbolji potez koji je beli nasao za sada. Nazovimo ga potezom X. On nastavlja da gleda poteze.
   Dok beli razmatra naredni potez, nazovimo ga potezom Y, beli vidi da protivnik ima odgovor na potez Y 
  koji ce evaluaciju spustiti ispod +2, sto znaci da beli ne treba da odigra potez Y,
   zato sto taj potez dozvoljava protivniku da dodje do bolje evaluacije nego da je beli
  odigrao potez X. Nema potrebe da se dalje gledaju drugi odgovori koje crni ima na potez Y, onog trenutka
  kada se ustanovi da nakon poteza Y crni ima i jedan potez koji ga dovodi u bolju situaciju nego
  sto bi mogao da dodje da je beli odigrao potez X, to znaci da je potez Y inferiorniji od poteza X. */  

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

    
    pub fn napravi_grane(dubina_drveta: u8, sirina_drveta: u8, number: u16) -> Grana{
        let (random, baza) = Grana::napravi_sve_random_podatke(dubina_drveta, sirina_drveta, number);

        let mut koren = Grana::new(1);
        koren.napravi_grane_recursive(dubina_drveta - 1, sirina_drveta, random, baza);
        koren
    }

    fn napravi_grane_recursive(&mut self, dubina_dece: u8, broj_dece: u8, random: u32, baza: u16){
        if dubina_dece <= 0 {
            return;
        }
        
        let broj = random % 20;

        for i in 0..broj_dece {
            let dete = Grana::new((broj+i as u32) as i16);
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

   

    fn napravi_sve_random_podatke(dubina_drveta: u8, sirina_drveta: u8, number: u16) ->
    (u32, u16){
        let random = Grana::random(dubina_drveta, sirina_drveta, number);
        let mut baza = 10;

        if number > 3 {
            baza = number & ((1<<7) -1);
        }

        println!("random = {}, baza = {}", random, baza);
        (random, baza)
    }

    fn random(br1: u8, br2: u8, number: u16) -> u32 {
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
        let podaci_o_svim_granama = self.podaci_o_celom_drvetu();
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

    /* Dalje moze da se optimizuje da ova metoda vraca evaluaciju i boolean da li je ovo najbolji protivnikov potez do sad,
    ali trenutno mislim da je ovakav kod citkiji, pa ga necu menjati (sumnjam da bi bila velika razlika u performansama).
    Kod kakav je sad 2 puta proverava istu stvar. Ako se u petlji desi rani return, 
    to znaci da protivnik nije odigrao najbolji potez za sada, a ako se ne desi rani return 
    to znaci da je protivnik odigrao najbolji potez do sada, tako da je logika u
     Grana::updejtuj_najbolji_potez suvisna. Ipak, smatram da je ovakav kod razumljiviji, 
     pa ga necu menjati.*/
    fn izracunaj_rekursivno(&mut self, vrednost_koju_protivnik_ima_u_dzepu: &Option<i16>, ja_volim_vise:  bool) -> i16 {
        if self.deca.len() == 0{
            return self.sam_izracunaj_svoju_vrednost();
        }

        let mut vrednost_mog_najboljeg_poteza: Option<i16> = None;
        for dete in &mut self.deca {
            let vrednost_mog_poteza = dete.izracunaj_rekursivno(&vrednost_mog_najboljeg_poteza, !ja_volim_vise);
            if Grana::protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, vrednost_mog_poteza, ja_volim_vise){
                return vrednost_mog_poteza;
            } 
            Grana::updejtuj_najbolji_potez(& mut vrednost_mog_najboljeg_poteza, vrednost_mog_poteza, ja_volim_vise);
        }

        vrednost_mog_najboljeg_poteza.unwrap()
    }


    fn updejtuj_najbolji_potez(najbolji_potez_za_sad: & mut Option<i16>, novi_potez: i16, ja_volim_vise: bool){
        match najbolji_potez_za_sad {
            Some(_najbolji_potez) => {
                if ja_volim_vise && (novi_potez > *_najbolji_potez) {
                    *_najbolji_potez = novi_potez;
                } 

                if !ja_volim_vise && (novi_potez < *_najbolji_potez) {
                    *_najbolji_potez = novi_potez;
                }
            }
            None => {*najbolji_potez_za_sad = Some(novi_potez);}
        }
    }

    fn protivnik_se_zajebo(potez_koji_je_protivnik_trebalo_da_odigra: &Option<i16>, evaluacija_posle_mog_poteza: i16, ja_volim_vise: bool) -> bool{
        if potez_koji_je_protivnik_trebalo_da_odigra.is_none() {
            return false;
        }
     
        if ja_volim_vise {
            evaluacija_posle_mog_poteza > potez_koji_je_protivnik_trebalo_da_odigra.unwrap()
        } else {
            evaluacija_posle_mog_poteza < potez_koji_je_protivnik_trebalo_da_odigra.unwrap()
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