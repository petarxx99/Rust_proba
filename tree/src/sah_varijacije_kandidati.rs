use crate::proba_sah_drveta::{self, vrednost_mata, Evaluacija, Evaluacija_poteza_jedenja};
use crate::tabla::potez::Potez_bits;
use crate::tabla::{Tabla, Ima_podatke_o_tabli};

static MAX_BROJ_VARIJACIJA: usize = 3;

pub struct Evaluacija_poteza<T> where T:Ima_podatke_o_tabli{
    pub pozicija: Pozicija<T>,
    pub nije_greska_otici_u_ovu_varijaciju: bool,
}

impl<T> Evaluacija_poteza<T> where T:Ima_podatke_o_tabli{
    pub fn new(pozicija: Pozicija<T>, nije_greska_otici_u_ovu_varijaciju: bool)->Evaluacija_poteza<T>{
        Evaluacija_poteza{pozicija, nije_greska_otici_u_ovu_varijaciju}
    }

}

pub struct Evaluacija_gotove_partije{
    pub evaluacija_poteza: Evaluacija_poteza<Tabla>,
    pub partija_gotova: bool,
}
impl Evaluacija_gotove_partije{
    pub fn new(partija_gotova: bool, evaluacija_poteza: Evaluacija_poteza<Tabla>)->Evaluacija_gotove_partije{
        Evaluacija_gotove_partije{evaluacija_poteza, partija_gotova}
    }
}



pub struct Varijacija<T>
where T:Ima_podatke_o_tabli
{
    pub zavrsna_pozicija: T,
    pub potez: Potez_bits,
    pub evaluacija: f32,
}
impl<T> Varijacija<T> where T:Ima_podatke_o_tabli{
    pub fn new(zavrsna_pozicija: T, potez: Potez_bits, evaluacija: f32)-> Varijacija<T>{
        Varijacija{zavrsna_pozicija, potez, evaluacija}
    }

    pub fn pronadji_najbolji_potez(lista_varijacija: &[Varijacija<T>], ja_sam_beli: bool) -> (Option<Potez_bits>, f32){
        let mut najbolji_potez: Option<Potez_bits> = None;
        let mut najbolja_evaluacija: f32 = vrednost_mata(ja_sam_beli);

        for varijacija in lista_varijacija {
            let evaluacija_je_najbolja_do_sad: bool = (ja_sam_beli && varijacija.evaluacija > najbolja_evaluacija) || (!ja_sam_beli && varijacija.evaluacija < najbolja_evaluacija);
            if evaluacija_je_najbolja_do_sad{
                najbolja_evaluacija = varijacija.evaluacija;
                najbolji_potez = Some(varijacija.potez.copy());
            }
        }

        (najbolji_potez, najbolja_evaluacija)
    }

    
}


pub struct Pozicija<T> where T:Ima_podatke_o_tabli{
    pub zavrsna_pozicija: T,
    pub evaluacija: f32,
}
impl<T> Pozicija<T> where T:Ima_podatke_o_tabli{
    pub fn new(zavrsna_pozicija: T, evaluacija: f32)->Pozicija<T>{
        Pozicija{zavrsna_pozicija, evaluacija}
    }
    
    pub fn to_varijacija(self, potez: Potez_bits)->Varijacija<T>{
        Varijacija::new(self.zavrsna_pozicija, potez, self.evaluacija)
    }
    
}

pub fn ubaci_u_sortiranu_listu_varijacija(pozicija: Pozicija<Tabla>, potez: Potez_bits, lista: &mut Vec<Varijacija<Tabla>>, ja_sam_beli: bool) {
    let mut i: usize = 0;
    let velicina_liste: usize = lista.len();
    if velicina_liste == 0 {
        lista.push(pozicija.to_varijacija(potez));
        return;
    }

    while i<velicina_liste {
        if (ja_sam_beli && pozicija.evaluacija > lista[i].evaluacija)   ||   (!ja_sam_beli && pozicija.evaluacija < lista[i].evaluacija) {
           
            lista.insert(i, Varijacija::new(pozicija.zavrsna_pozicija.copy(), potez, pozicija.evaluacija));
            if lista.len() > MAX_BROJ_VARIJACIJA {
                lista.pop();
            }
            break;
        }
        i += 1;
    } 
}

impl Tabla{


    pub fn najbolji_potez_i_njegova_evaluacija_preko_kandidata(&self, dubina: u8) -> (Option<Potez_bits>, f32) {
        let mut varijacije_kandidati = self.izracunaj_varijacije_kandidate(dubina);
      
        for kandidat in &mut varijacije_kandidati{
            let kandidat_zavrsava_kad_je_beli_na_potezu: bool = kandidat.zavrsna_pozicija.beli_je_na_potezu();
            let najgora_vrednost_za_mene = vrednost_mata(kandidat.zavrsna_pozicija.beli_je_na_potezu());
            let najgora_vrednost_za_protivnika = vrednost_mata(!kandidat.zavrsna_pozicija.beli_je_na_potezu());
            
            let tabla_zavrsne_pozicije_kandidata = &kandidat.zavrsna_pozicija;
            kandidat.evaluacija = tabla_zavrsne_pozicije_kandidata.izracunaj_rekursivno_kandidate_drugi_talas(najgora_vrednost_za_protivnika, kandidat_zavrsava_kad_je_beli_na_potezu, dubina, 0, najgora_vrednost_za_protivnika, najgora_vrednost_za_mene).pozicija.evaluacija;
        }    
        
        let (najbolji_potez, evaluacija) = Varijacija::pronadji_najbolji_potez(&varijacije_kandidati, self.beli_je_na_potezu());
        (najbolji_potez, evaluacija)
    }

    fn izracunaj_varijacije_kandidate(&self, dubina: u8) -> Vec<Varijacija<Tabla>>{
        let protivnik_je_beli: bool = !self.beli_je_na_potezu();
        let ja_sam_beli: bool = self.beli_je_na_potezu();

        let mut najbolje_varijacije: Vec<Varijacija<Tabla>> = Vec::new();  
        let mut eval_poslednjeg_najboljeg_poteza: f32 = vrednost_mata(ja_sam_beli);

        let legalni_potezi: Vec<Potez_bits> = self.svi_legalni_potezi();
        for potez in legalni_potezi {
            let tabla: Tabla = self.tabla_nakon_poteza_bits(&potez);
            let eval = tabla.izracunaj_rekursivno_kandidate_prvi_talas(eval_poslednjeg_najboljeg_poteza, protivnik_je_beli, dubina, 1, self.materijalna_prednost_onog_ko_je_na_potezu(), vrednost_mata(!ja_sam_beli)) ;
           
            ubaci_u_sortiranu_listu_varijacija(eval.pozicija, potez, &mut najbolje_varijacije, ja_sam_beli);
            let poslednja_najbolja_varijacija: &Varijacija<Tabla> = &najbolje_varijacije[najbolje_varijacije.len()-1];
            eval_poslednjeg_najboljeg_poteza = poslednja_najbolja_varijacija.evaluacija;
        }

        najbolje_varijacije
    }








fn evaluiraj_gledajuci_poteze_jedenja_prvi_talas(&self, vrednost_koju_protivnik_ima_u_dzepu: f32,
materijalno_stanje: f32, materijal_proslog_poteza:f32, materijal_pretproslog_poteza: f32, ja_volim_vise:bool) ->
 Evaluacija_poteza<Tabla>{
        
    let mali_broj: f32 = 0.125;
        if materijalno_stanje + mali_broj > materijal_pretproslog_poteza {
            let (eval, protivnik_se_nije_zajebo) =  self.vrati_nerekursivnu_evaluaciju_koja_uzima_u_obzir_da_li_je_mat(&Some(vrednost_koju_protivnik_ima_u_dzepu), ja_volim_vise);
            let pozicija: Pozicija<Tabla> = Pozicija::new(self.copy(), eval);
            return Evaluacija_poteza::new(pozicija, protivnik_se_nije_zajebo);      
        }

        let najgora_evaluacija_za_protivnika: f32 = vrednost_mata(!ja_volim_vise);
        let eval = self.izracunaj_rekursivno_jedenje_figura_za_kandidate(najgora_evaluacija_za_protivnika, ja_volim_vise, materijal_proslog_poteza, materijal_pretproslog_poteza);
      
        let evaluacija: f32;
        if eval.nije_greska_otici_u_ovu_varijaciju{
            let nekompresirana_tabla = eval.pozicija.zavrsna_pozicija.to_nekompresirana_tabla();
            evaluacija = eval.pozicija.zavrsna_pozicija.nerekursivno_evaluiraj_poziciju(&nekompresirana_tabla);
        } else {
            evaluacija = self.nerekursivno_evaluiraj_poziciju(&self.to_nekompresirana_tabla());
        }

        vrati_evaluaciju_poteza(vrednost_koju_protivnik_ima_u_dzepu, evaluacija, ja_volim_vise, self.copy())
}






pub fn izracunaj_rekursivno_kandidate_prvi_talas(&self, vrednost_koju_protivnik_ima_u_dzepu: f32, ja_volim_vise:  bool,
broj_rekursija: u8, trenutna_rekursija: u8, materijal_proslog_poteza: f32, materijal_pretproslog_poteza: f32) -> 
Evaluacija_poteza<Tabla>{
    
    let materijalno_stanje: f32 = self.materijalna_prednost_onog_ko_je_na_potezu();
    if trenutna_rekursija >= broj_rekursija{
        return self.evaluiraj_gledajuci_poteze_jedenja_prvi_talas(vrednost_koju_protivnik_ima_u_dzepu, materijalno_stanje, materijal_proslog_poteza, materijal_pretproslog_poteza, ja_volim_vise)
    }

    let legalni_potezi: Vec<Potez_bits> = self.svi_legalni_potezi();
    let evaluacija_gotove_partije = self.vrati_evaluaciju_gotove_partije(vrednost_koju_protivnik_ima_u_dzepu, &legalni_potezi, ja_volim_vise);
    if evaluacija_gotove_partije.is_some(){
        return evaluacija_gotove_partije.unwrap();
    }

    let mut najbolja_opcija_za_sad: f32 = vrednost_mata(ja_volim_vise);
    let mut najbolja_tabla : Pozicija<Tabla> = Pozicija::new(self.copy(), vrednost_mata(ja_volim_vise));

    for legalan_potez in legalni_potezi {
        let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(&legalan_potez);
        
        let eval_poteza = tabla_nakon_poteza.izracunaj_rekursivno_kandidate_prvi_talas(najbolja_opcija_za_sad, !ja_volim_vise, broj_rekursija, trenutna_rekursija+1, materijalno_stanje, materijal_proslog_poteza);
        if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, eval_poteza.pozicija.evaluacija, ja_volim_vise){
            return Evaluacija_poteza::new(eval_poteza.pozicija,false);
        }
        if eval_poteza.nije_greska_otici_u_ovu_varijaciju {
                 najbolja_opcija_za_sad = eval_poteza.pozicija.evaluacija;
                 najbolja_tabla = eval_poteza.pozicija;
        } 
    }
    
    iz_pozicije_u_evaluaciju_poteza(najbolja_tabla, vrednost_koju_protivnik_ima_u_dzepu, ja_volim_vise)

}

fn izracunaj_rekursivno_jedenje_figura_za_kandidate(&self, 
    vrednost_koju_protivnik_ima_u_dzepu: f32, ja_sam_beli:  bool,
    materijal_proslog_poteza: f32, materijal_pretproslog_poteza: f32) -> 
    Evaluacija_poteza<Tabla>{

    let materijalno_stanje: f32 = self.materijalna_prednost_onog_ko_je_na_potezu();
    let mali_broj: f32 = 0.125;
    if materijalno_stanje + mali_broj > materijal_pretproslog_poteza{
        return vrati_evaluaciju_poteza(vrednost_koju_protivnik_ima_u_dzepu, self.beli_minus_crni_materijal(), ja_sam_beli, self.copy());
    }

    let legalni_potezi: Vec<Potez_bits> = self.svi_legalni_potezi();
    let evaluacija_gotove_partije = self.vrati_evaluaciju_gotove_partije(vrednost_koju_protivnik_ima_u_dzepu, &legalni_potezi, ja_sam_beli);
    if evaluacija_gotove_partije.is_some(){
        return evaluacija_gotove_partije.unwrap();
    }

    let potezi_jedenja: Vec<Potez_bits> = self.samo_potezi_koji_jedu_figure(&legalni_potezi);
    if potezi_jedenja.len() == 0 {
        return vrati_evaluaciju_poteza(vrednost_koju_protivnik_ima_u_dzepu, self.beli_minus_crni_materijal(), ja_sam_beli, self.copy());
    }

    let evaluacija_koju_imam_u_dzepu_ako_ne_jedem_nista:f32 = self.beli_minus_crni_materijal();
    let mut najbolja_opcija_za_sad: f32 = evaluacija_koju_imam_u_dzepu_ako_ne_jedem_nista;
    let mut najbolji_potez = Pozicija::new(self.copy(), evaluacija_koju_imam_u_dzepu_ako_ne_jedem_nista);

    for potez in potezi_jedenja {
        let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(&potez);

        let eval_poteza = tabla_nakon_poteza.izracunaj_rekursivno_jedenje_figura_za_kandidate(najbolja_opcija_za_sad, !ja_sam_beli, materijalno_stanje, materijal_proslog_poteza);
        if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, eval_poteza.pozicija.evaluacija, ja_sam_beli){
            return Evaluacija_poteza::new(eval_poteza.pozicija, false);
        }
        if eval_poteza.nije_greska_otici_u_ovu_varijaciju{
            najbolja_opcija_za_sad = eval_poteza.pozicija.evaluacija;
            najbolji_potez = eval_poteza.pozicija;
        }
    }

    iz_pozicije_u_evaluaciju_poteza(najbolji_potez, vrednost_koju_protivnik_ima_u_dzepu, ja_sam_beli)

}






fn vrati_evaluaciju_gotove_partije(&self, vrednost_koju_protivnik_ima_u_dzepu: f32, legalni_potezi: &[Potez_bits], ja_sam_beli:  bool) -> 
Option<Evaluacija_poteza<Tabla>>{

    let broj_legalnih_poteza: usize = legalni_potezi.len();
    if broj_legalnih_poteza == 0 {
        if self.igrac_je_u_sahu(&self.to_nekompresirana_tabla()) {
            let evaluacija_mata = vrati_evaluaciju_poteza(vrednost_koju_protivnik_ima_u_dzepu, vrednost_mata(ja_sam_beli), ja_sam_beli, self.copy());
            return Some(evaluacija_mata)
        } else {
            let evaluacija_pata = vrati_evaluaciju_poteza(vrednost_koju_protivnik_ima_u_dzepu, 0.0, ja_sam_beli, self.copy());
            return Some(evaluacija_pata)
          }
    }

    if self.pre_koliko_poteza_je_50_move_rule_pomeren() >= 50 {
        let evaluacija_nereseno = vrati_evaluaciju_poteza(vrednost_koju_protivnik_ima_u_dzepu, 0.0, ja_sam_beli, self.copy());   
        return Some(evaluacija_nereseno)
    }

    None
}



pub fn izracunaj_rekursivno_kandidate_drugi_talas(&self, vrednost_koju_protivnik_ima_u_dzepu: f32, ja_volim_vise:  bool,
    broj_rekursija: u8, trenutna_rekursija: u8, materijal_proslog_poteza: f32, materijal_pretproslog_poteza: f32) -> 
    Evaluacija_poteza<Tabla>{
        
        let materijalno_stanje: f32 = self.materijalna_prednost_onog_ko_je_na_potezu();
        if trenutna_rekursija >= broj_rekursija{
            return self.evaluiraj_gledajuci_poteze_jedenja_nezahtevan_racun(vrednost_koju_protivnik_ima_u_dzepu, materijalno_stanje, materijal_proslog_poteza, materijal_pretproslog_poteza, ja_volim_vise)
        }
    
        let legalni_potezi: Vec<Potez_bits> = self.svi_legalni_potezi();
        let evaluacija_gotove_partije = self.vrati_evaluaciju_gotove_partije(vrednost_koju_protivnik_ima_u_dzepu, &legalni_potezi, ja_volim_vise);
        if evaluacija_gotove_partije.is_some(){
            return evaluacija_gotove_partije.unwrap();
        }
    
        let mut najbolja_opcija_za_sad: f32 = vrednost_mata(ja_volim_vise);
        let mut najbolja_tabla : Pozicija<Tabla> = Pozicija::new(self.copy(), vrednost_mata(ja_volim_vise));
    
        for legalan_potez in legalni_potezi {
            let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(&legalan_potez);
            
            let eval_poteza = tabla_nakon_poteza.izracunaj_rekursivno_kandidate_drugi_talas(najbolja_opcija_za_sad, !ja_volim_vise, broj_rekursija, trenutna_rekursija+1, materijalno_stanje, materijal_proslog_poteza);
            if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, eval_poteza.pozicija.evaluacija, ja_volim_vise){
                return Evaluacija_poteza::new(eval_poteza.pozicija,false);
            }
            if eval_poteza.nije_greska_otici_u_ovu_varijaciju {
                     najbolja_opcija_za_sad = eval_poteza.pozicija.evaluacija;
                     najbolja_tabla = eval_poteza.pozicija;
            } 
        }
        
        iz_pozicije_u_evaluaciju_poteza(najbolja_tabla, vrednost_koju_protivnik_ima_u_dzepu, ja_volim_vise)
    
    }
    

    fn evaluiraj_gledajuci_poteze_jedenja_nezahtevan_racun(&self, vrednost_koju_protivnik_ima_u_dzepu: f32,
        materijalno_stanje: f32, materijal_proslog_poteza:f32, materijal_pretproslog_poteza: f32, ja_volim_vise:bool) ->
         Evaluacija_poteza<Tabla>{
                
            let mali_broj: f32 = 0.125;
                if materijalno_stanje + mali_broj > materijal_pretproslog_poteza {
                    let lagana_nerekursivna_evaluacija: f32 = self.nerekursivno_i_nezahtevno_evaluiraj_poziciju(&self.to_nekompresirana_tabla());
                    return vrati_evaluaciju_poteza(vrednost_koju_protivnik_ima_u_dzepu, lagana_nerekursivna_evaluacija, ja_volim_vise, self.copy());    
                }
        
                let najgora_evaluacija_za_protivnika: f32 = vrednost_mata(!ja_volim_vise);
                let eval_poteza = self.izracunaj_rekursivno_jedenje_figura_za_kandidate(najgora_evaluacija_za_protivnika, ja_volim_vise, materijal_proslog_poteza, materijal_pretproslog_poteza);
              
                let evaluacija: f32;
                if eval_poteza.nije_greska_otici_u_ovu_varijaciju{
                    let nekompresirana_tabla = eval_poteza.pozicija.zavrsna_pozicija.to_nekompresirana_tabla();
                    evaluacija = eval_poteza.pozicija.zavrsna_pozicija.nerekursivno_i_nezahtevno_evaluiraj_poziciju(&nekompresirana_tabla);
                } else {
                    evaluacija = self.nerekursivno_i_nezahtevno_evaluiraj_poziciju(&self.to_nekompresirana_tabla());
                }
        
                vrati_evaluaciju_poteza(vrednost_koju_protivnik_ima_u_dzepu, evaluacija, ja_volim_vise, self.copy())
        }

}






fn vrati_evaluaciju_poteza(vrednost_koju_protivnik_ima_u_dzepu: f32, evaluacija_posle_mog_poteza: f32, ja_sam_beli: bool,
tabla: Tabla) 
-> Evaluacija_poteza<Tabla>{

    let nije_greska_otici_u_ovu_varijaciju: bool = !protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, evaluacija_posle_mog_poteza, ja_sam_beli);
    let pozicija: Pozicija<Tabla> = Pozicija::new(tabla, evaluacija_posle_mog_poteza);
    Evaluacija_poteza{pozicija, nije_greska_otici_u_ovu_varijaciju}
}






fn protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu: f32, evaluacija_posle_mog_poteza: f32, ja_sam_beli: bool) -> bool{
    if ja_sam_beli {
        return evaluacija_posle_mog_poteza > vrednost_koju_protivnik_ima_u_dzepu
    } else {
        return evaluacija_posle_mog_poteza < vrednost_koju_protivnik_ima_u_dzepu;
    }
}




pub fn iz_pozicije_u_evaluaciju_poteza(pozicija: Pozicija<Tabla>, vrednost_koju_protivnik_ima_u_dzepu: f32, ja_sam_beli: bool)
    -> Evaluacija_poteza<Tabla>{
        let protivnik_se_nije_zajebo: bool = !protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu,pozicija.evaluacija, ja_sam_beli);
        Evaluacija_poteza::new(pozicija, protivnik_se_nije_zajebo)
}