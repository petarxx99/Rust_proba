use crate::proba_sah_drveta::{vrednost_mata, protivnik_se_zajebo, ovo_je_najbolji_potez};
use crate::tabla::potez::{Potez_bits, Potez};
use crate::tabla::{Tabla, DESNI_KONJ, F_FILE, Promocija, D_FILE, C_FILE, G_FILE, E_FILE};
use std::sync::mpsc::{Sender, Receiver};
use std::{thread};

static MAX_BROJ_POTEZA_KANDIDATA: usize = 3;
static MAKSIMALAN_MATERIJAL_BEZ_PIJUNA_KAD_JE_PARTIJA_U_ZAVRSNICI: f32 = 8.25;

fn nabavi_najlosiji_potez_koji_prolazi(lista: &Vec<(Potez_bits,f32)>, broj_poteza_koji_prolaze:usize, ja_sam_beli:bool) -> f32{
    if lista.len() < broj_poteza_koji_prolaze {
        return vrednost_mata(ja_sam_beli);
    }
    lista[broj_poteza_koji_prolaze-1].1
}

fn u_sortiranu_listu(lista: &mut Vec<(Potez_bits, f32)>, potez: Potez_bits, evaluacija: f32,ja_sam_beli:bool, max_duzina: usize){
   
    let duzina_liste: usize = lista.len();

    let mut i: usize = 0;
    while i < duzina_liste {
        if (ja_sam_beli && evaluacija > lista[i].1) || (!ja_sam_beli && evaluacija < lista[i].1){
            lista.insert(i, (potez, evaluacija));
            if lista.len() > max_duzina{
                lista.pop();
            }
            return;
        } 

        i += 1;
    }

    if duzina_liste < max_duzina {
        lista.push((potez,evaluacija));
        return;
    }
}

fn izbaci_broj_elemenata_s_kraja<T>(lista: &mut Vec<T>, procenat_elemenata: f32) {
    let broj_elemenata_za_izbacivanje = lista.len() as f32 * procenat_elemenata;
    let broj_elemenata_za_izbacivanje = broj_elemenata_za_izbacivanje as usize;
    

    for _ in 0..broj_elemenata_za_izbacivanje {
        lista.pop();
    }
}

fn maksimum(broj1: usize, broj2: usize)->usize{
    if broj1>broj2{
        return broj1;
    }
    return broj2;
}

fn minimum(broj1:usize, broj2:usize) -> usize{
    if broj1<broj2{
        broj1
    } else {
        broj2
    }
}

fn zameni_mesta(potezi: &mut Vec<(Potez_bits, f32)>, indeks_prvog_poteza: usize, indeks_drugog_poteza: usize){
    let temp_potez = potezi[indeks_prvog_poteza].0.copy();
    let temp_evaluacija: f32 = potezi[indeks_prvog_poteza].1;

    potezi[indeks_prvog_poteza].0 = potezi[indeks_drugog_poteza].0.copy();
    potezi[indeks_prvog_poteza].1 = potezi[indeks_drugog_poteza].1;

    potezi[indeks_drugog_poteza].0 = temp_potez;
    potezi[indeks_drugog_poteza].1 = temp_evaluacija;
}

fn prvi_potez_je_bolji(prvi_potez: f32, drugi_potez: f32, ja_sam_beli: bool) -> bool {
    if ja_sam_beli{
        prvi_potez > drugi_potez
    } else {
        prvi_potez < drugi_potez
    }
}

impl Tabla {


    pub fn najbolji_potez_i_njegova_evaluacija_putem_iteracija(&self, mut dubina: u8) -> (Option<Potez_bits>, f32) {
        if self.je_pozicija_e4_e5_nf3_d6_bc4_bg4_d3(){
            return (Some(Potez::new(G_FILE, 8, F_FILE, 6, Promocija::None).to_Potez_bits(self).expect("Potez iz funkcije najbolji_potez_i_njegova_evaluacija_putem_iteracija ne postoji.")), 0.8);
        }
 
        let ja_sam_beli: bool = self.beli_je_na_potezu();

        let mut potezi_kandidati = self.pronadji_kandidate_preko_tredova(dubina, MAX_BROJ_POTEZA_KANDIDATA);
        self.sortiraj_poteze(&mut potezi_kandidati);
        println!("broj poteza kandidata: {}", potezi_kandidati.len());
        for potez in &potezi_kandidati {println!("\npotez kandidata: {}\n", &potez.0);}

        let maksimum_poteza = minimum(potezi_kandidati.len(), MAX_BROJ_POTEZA_KANDIDATA);
        let potezi_kandidati = &mut potezi_kandidati[0..maksimum_poteza];
        
        let nova_dubina = dubina + 2;
        self.najbolji_potez_iz_kandidata_nezahtevno_i_tredovima(potezi_kandidati, nova_dubina, ja_sam_beli)
    }


    pub fn najbolji_potez_iz_kandidata_nezahtevno_i_tredovima(&self,
        potezi_kandidati: &mut[(Potez_bits, f32)],  dubina: u8, ja_sam_beli: bool)
     -> (Option<Potez_bits>, f32){
    
        let potezi_za_thread: Vec<(Potez_bits, f32)> = svaki_n_potez(potezi_kandidati, 2, 1);
        let potezi = svaki_n_potez(potezi_kandidati, 2, 0);
        let (sender_iz_threada, receiver_iz_main) = std::sync::mpsc::channel();
        let (sender_iz_main, receiver_iz_threada) = std::sync::mpsc::channel();

        let tabla = self.copy();
        let thread_handle = thread::spawn(move || {
            tabla.najbolji_potez_threada(sender_iz_threada, receiver_iz_threada, potezi_za_thread, dubina)
        });

        let (najbolji_potez, najbolja_evaluacija) = self.najbolji_potez_threada(sender_iz_main, receiver_iz_main, potezi, dubina);
        let (najbolji_potez_thread, najbolja_evaluacija_threada) = thread_handle.join().expect("Greska prilikom otpakivanja podataka iz threada koji obradjuje kandidate.");
        
        if prvi_potez_je_bolji_od_drugog(najbolja_evaluacija_threada, najbolja_evaluacija, ja_sam_beli){
            return (najbolji_potez_thread, najbolja_evaluacija_threada)
        } 
        (najbolji_potez, najbolja_evaluacija)
     }
   

    fn najbolji_potez_threada(&self, sender: Sender<f32>, receiver: Receiver<f32>, mut potezi: Vec<(Potez_bits, f32)>, dubina: u8) 
    -> (Option<Potez_bits>, f32){
        let ja_sam_beli: bool = self.beli_je_na_potezu();
        let protivnik_je_beli: bool = !ja_sam_beli;
        let mut najbolja_evaluacija_ovog_threada: f32 = vrednost_mata(ja_sam_beli);
        let mut vrednost_koju_imam_u_dzepu: f32 = najbolja_evaluacija_ovog_threada;
        let mut najbolji_potez: Option<Potez_bits> = None;

        for (potez, evaluacija) in &mut potezi {
            match receiver.try_recv(){
                Ok(najbolja_eval_drugog_threada) => {
                    if prvi_potez_je_bolji_od_drugog(najbolja_eval_drugog_threada, najbolja_evaluacija_ovog_threada, ja_sam_beli){
                        vrednost_koju_imam_u_dzepu = najbolja_eval_drugog_threada;
                    }
                },
                Err(_) => {},
            }
            let tabla: Tabla = self.tabla_nakon_poteza_bits(&potez);
            let (vrednost_poteza, _) = tabla.izracunaj_rekursivno_zove_nezahtevne_funkcije_gleda_dublje_ako_naidje_na_sah(&Some(vrednost_koju_imam_u_dzepu), protivnik_je_beli, dubina, 1,  self.materijalna_prednost_onog_ko_je_na_potezu(), vrednost_mata(protivnik_je_beli), false);
            *evaluacija = vrednost_poteza;
            if ovo_je_najbolji_potez(najbolja_evaluacija_ovog_threada, vrednost_poteza, ja_sam_beli){
                najbolja_evaluacija_ovog_threada = vrednost_poteza;
                najbolji_potez = Some(potez.copy());
                if prvi_potez_je_bolji_od_drugog(vrednost_poteza, vrednost_koju_imam_u_dzepu, ja_sam_beli){
                    vrednost_koju_imam_u_dzepu = vrednost_poteza;
                    let r = sender.send(vrednost_poteza);
                }
            }
        }

        (najbolji_potez, najbolja_evaluacija_ovog_threada)
    }


    pub fn pronadji_kandidate_preko_iteracija(&self, dubina: u8) -> Vec<(Potez_bits, f32)>{
        let ja_volim_vise: bool = self.beli_je_na_potezu();

        let mut potezi_evaluacije: Vec<(Potez_bits, f32)> = self.init_potezi_evaluacije();
        let dubina_sa_2: usize = (dubina / 2) as usize;
    
        let mut broj_poteza_koji_prolaze: usize = potezi_evaluacije.len();
        let mut i: usize = 1;
        while i <= dubina_sa_2 {
            let broj_rekursija: u8 = (i*2) as u8;
            let potezi_evaluacije_prvi_thread = svaki_n_potez(&potezi_evaluacije, 2, 0);
            let potezi_evaluacije_drugi_thread = svaki_n_potez(&potezi_evaluacije, 2, 1);
           
            let tabla = self.copy();
            let thread_handle = thread::spawn(move || {
                tabla.pronadji_poteze_koji_prolaze(broj_rekursija, potezi_evaluacije_drugi_thread, broj_poteza_koji_prolaze)
            });
            let potezi_koji_prolaze: Vec<(Potez_bits, f32)> = self.pronadji_poteze_koji_prolaze(broj_rekursija, potezi_evaluacije_prvi_thread, broj_poteza_koji_prolaze);
            let potezi_koji_prolaze_iz_drugog_threada: Vec<(Potez_bits, f32)> = thread_handle.join().expect("Greska prilikom pronalazenja poteza kandidata iz drugog threada.");
            let mut potezi_koji_prolaze: Vec<(Potez_bits, f32)> = spoji_2_niza_sortiranih_poteza(potezi_koji_prolaze, potezi_koji_prolaze_iz_drugog_threada, broj_poteza_koji_prolaze, ja_volim_vise);
           
            if i>1{
                broj_poteza_koji_prolaze = maksimum(potezi_koji_prolaze.len() / 2, MAX_BROJ_POTEZA_KANDIDATA);
                skrati_vektor(&mut potezi_koji_prolaze, broj_poteza_koji_prolaze);
            }    
            potezi_evaluacije = potezi_koji_prolaze;

            i+=1;
        }


        potezi_evaluacije
    }

    fn pronadji_poteze_koji_prolaze(&self, dubina: u8, potezi_evaluacije: Vec<(Potez_bits, f32)>,
broj_poteza_koji_prolaze: usize) -> Vec<(Potez_bits, f32)> {
            let ja_sam_beli: bool = self.beli_je_na_potezu();
            let mut potezi_koji_prolaze: Vec<(Potez_bits, f32)> = Vec::new();
            let mut najlosiji_potez_koji_prolazi: f32 = vrednost_mata(ja_sam_beli);

            for (potez, _) in potezi_evaluacije{
                let tabla: Tabla = self.tabla_nakon_poteza_bits(&potez);
                let (vrednost_poteza, _) = tabla.izracunaj_rekursivno_bez_gledanja_saha(&Some(najlosiji_potez_koji_prolazi), !ja_sam_beli, dubina, 1, 0.0, 0.0, false);
                u_sortiranu_listu(&mut potezi_koji_prolaze, potez, vrednost_poteza, ja_sam_beli, broj_poteza_koji_prolaze);
                najlosiji_potez_koji_prolazi = nabavi_najlosiji_potez_koji_prolazi(&potezi_koji_prolaze, broj_poteza_koji_prolaze, ja_sam_beli);
            }
            self.sortiraj_poteze(&mut potezi_koji_prolaze);

            potezi_koji_prolaze
    }

    
    fn pronadji_poteze_koji_prolaze_tredovi_razmenjuju_poruke(&self, dubina: u8, potezi_evaluacije: Vec<(Potez_bits, f32)>,
broj_poteza_koji_prolaze: usize,
sender: Sender<f32>, receiver: Receiver<f32>) -> Vec<(Potez_bits, f32)> {
            let ja_sam_beli: bool = self.beli_je_na_potezu();
            let mut potezi_koji_prolaze: Vec<(Potez_bits, f32)> = Vec::new();
            let mut najlosiji_potez_koji_prolazi: f32 = vrednost_mata(ja_sam_beli);

            for (potez, _) in potezi_evaluacije{
                let tabla: Tabla = self.tabla_nakon_poteza_bits(&potez);
                najlosiji_potez_koji_prolazi = vrati_bolji_potez(&receiver, najlosiji_potez_koji_prolazi, ja_sam_beli);

                let (vrednost_poteza, _) = tabla.izracunaj_rekursivno_bez_gledanja_saha(&Some(najlosiji_potez_koji_prolazi), !ja_sam_beli, dubina, 1, 0.0, 0.0, false);
                u_sortiranu_listu(&mut potezi_koji_prolaze, potez, vrednost_poteza, ja_sam_beli, broj_poteza_koji_prolaze);
                najlosiji_potez_koji_prolazi = nabavi_najlosiji_potez_koji_prolazi(&potezi_koji_prolaze, broj_poteza_koji_prolaze, ja_sam_beli);
                let s = sender.send(najlosiji_potez_koji_prolazi);
            }
            self.sortiraj_poteze(&mut potezi_koji_prolaze);

            potezi_koji_prolaze
    }


  

    pub fn pronadji_kandidate_preko_tredova(&self, dubina: u8, broj_poteza_koji_prolaze: usize)
     -> Vec<(Potez_bits, f32)>{
            let ja_volim_vise: bool = self.beli_je_na_potezu();
            let potezi_evaluacije: Vec<(Potez_bits, f32)> = self.init_potezi_evaluacije();
       
            let potezi_evaluacije_prvi_thread = svaki_n_potez(&potezi_evaluacije, 2, 0);
            let potezi_evaluacije_drugi_thread = svaki_n_potez(&potezi_evaluacije, 2, 1);
            let (sender_prvi_thread, receiver_drugi_thread): (Sender<f32>, Receiver<f32>) = std::sync::mpsc::channel();
            let (sender_drugi_thread, receiver_prvi_thread): (Sender<f32>, Receiver<f32>) = std::sync::mpsc::channel();

            let tabla = self.copy();
            let thread_handle = thread::spawn(move || {
                tabla.pronadji_poteze_koji_prolaze_tredovi_razmenjuju_poruke(dubina, potezi_evaluacije_drugi_thread, broj_poteza_koji_prolaze, sender_drugi_thread, receiver_drugi_thread)
            });
            let potezi_koji_prolaze: Vec<(Potez_bits, f32)> = self.pronadji_poteze_koji_prolaze_tredovi_razmenjuju_poruke(dubina, potezi_evaluacije_prvi_thread, broj_poteza_koji_prolaze, sender_prvi_thread, receiver_prvi_thread);
            let potezi_koji_prolaze_iz_drugog_threada: Vec<(Potez_bits, f32)> = thread_handle.join().expect("Greska prilikom pronalazenja poteza kandidata iz drugog threada.");
            spoji_2_niza_sortiranih_poteza(potezi_koji_prolaze, potezi_koji_prolaze_iz_drugog_threada, broj_poteza_koji_prolaze, ja_volim_vise)   
    }

    pub fn izracunaj_rekursivno_bez_gledanja_saha(&self, vrednost_koju_protivnik_ima_u_dzepu: &Option<f32>, ja_volim_vise:  bool,
        mut broj_rekursija: u8, trenutna_rekursija: u8, materijal_proslog_poteza: f32, materijal_pretproslog_poteza: f32, mut dodao_sam_dubinu_zbog_saha: bool) -> (f32, bool){
            
            let materijalno_stanje: f32 = self.materijalna_prednost_onog_ko_je_na_potezu();
            if trenutna_rekursija >= broj_rekursija{
                return self.evaluiraj_gledajuci_poteze_jedenja(vrednost_koju_protivnik_ima_u_dzepu, materijalno_stanje, materijal_proslog_poteza, materijal_pretproslog_poteza, ja_volim_vise)
            }
        
            let (legalni_potezi, _) = self.svi_legalni_potezi_sortirani_po_jedenju_figura();
            let evaluacija_gotove_partije = self.vrati_evaluaciju_ako_je_partija_gotova(vrednost_koju_protivnik_ima_u_dzepu, &legalni_potezi, ja_volim_vise);
            if evaluacija_gotove_partije.partija_zavrsena {
                return evaluacija_gotove_partije.evaluacija
            }
           
        
            let mut najbolja_opcija_za_sad: f32 = vrednost_mata(ja_volim_vise);
            for legalan_potez in legalni_potezi {
                let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(&legalan_potez);
                
                let (vrednost_poteza, najbolji_potez) = tabla_nakon_poteza.izracunaj_rekursivno(&Some(najbolja_opcija_za_sad), !ja_volim_vise, broj_rekursija, trenutna_rekursija+1, materijalno_stanje, materijal_proslog_poteza, dodao_sam_dubinu_zbog_saha);
                if najbolji_potez {
                         najbolja_opcija_za_sad = vrednost_poteza;
                }
                if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, najbolja_opcija_za_sad, ja_volim_vise){
                        return (najbolja_opcija_za_sad, false)
                }   
            }
            
            (najbolja_opcija_za_sad, true)
        
    }

    pub fn izracunaj_rekursivno_zove_nezahtevne_funkcije_gleda_dublje_ako_naidje_na_sah(&self, vrednost_koju_protivnik_ima_u_dzepu: &Option<f32>, ja_volim_vise:  bool,
        mut broj_rekursija: u8, trenutna_rekursija: u8, materijal_proslog_poteza: f32, materijal_pretproslog_poteza: f32, mut dodao_sam_dubinu_zbog_saha: bool) -> (f32, bool){
            
            let materijalno_stanje: f32 = self.materijalna_prednost_onog_ko_je_na_potezu();
            if trenutna_rekursija >= broj_rekursija{
                return self.evaluiraj_gledajuci_poteze_jedenja_nezahtevno(vrednost_koju_protivnik_ima_u_dzepu, materijalno_stanje, materijal_proslog_poteza, materijal_pretproslog_poteza, ja_volim_vise)
            }
        
            let (legalni_potezi, _) = self.svi_legalni_potezi_sortirani_po_jedenju_figura();
            let evaluacija_gotove_partije = self.vrati_evaluaciju_ako_je_partija_gotova(vrednost_koju_protivnik_ima_u_dzepu, &legalni_potezi, ja_volim_vise);
            if evaluacija_gotove_partije.partija_zavrsena {
                return evaluacija_gotove_partije.evaluacija
            }
            if self.igrac_je_u_sahu(&self.to_nekompresirana_tabla()){
                broj_rekursija += 2;
                dodao_sam_dubinu_zbog_saha = true;
            }
        
            let mut najbolja_opcija_za_sad: f32 = vrednost_mata(ja_volim_vise);
            for legalan_potez in legalni_potezi {
                let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(&legalan_potez);
                
                let (vrednost_poteza, najbolji_potez) = tabla_nakon_poteza.izracunaj_rekursivno_zove_nezahtevne_funkcije(&Some(najbolja_opcija_za_sad), !ja_volim_vise, broj_rekursija, trenutna_rekursija+1, materijalno_stanje, materijal_proslog_poteza, dodao_sam_dubinu_zbog_saha);
                if najbolji_potez {
                         najbolja_opcija_za_sad = vrednost_poteza;
                }
                if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, najbolja_opcija_za_sad, ja_volim_vise){
                        return (najbolja_opcija_za_sad, false)
                }   
            }
            
            (najbolja_opcija_za_sad, true)
        
        }




    fn sortiraj_poteze(&self, mut potezi_evaluacije: &mut Vec<(Potez_bits,f32)>){
        let ja_sam_beli: bool = self.beli_je_na_potezu();
        let broj_poteza: usize = potezi_evaluacije.len();
        let broj_poteza_minus_jedan = broj_poteza - 1;

        let mut i: usize = 0;
        while i < broj_poteza_minus_jedan {
            let mut indeks_najboljeg_poteza: usize = i;

            let mut j: usize = i + 1;
            while j<broj_poteza {
                if prvi_potez_je_bolji(potezi_evaluacije[j].1,  potezi_evaluacije[indeks_najboljeg_poteza].1, ja_sam_beli) {
                    indeks_najboljeg_poteza = j;
                }
                j += 1;
            }
            zameni_mesta(&mut potezi_evaluacije, i, indeks_najboljeg_poteza);

            i+=1;
        }

    }

    fn init_potezi_evaluacije(&self) -> Vec<(Potez_bits,f32)>{
        let vrednost_mata: f32 = vrednost_mata(self.beli_je_na_potezu());
        let (legalni_potezi,_) = self.svi_legalni_potezi_sortirani_po_jedenju_figura();
        let mut potezi_evaluacije: Vec<(Potez_bits, f32)> = Vec::new();
        for potez in legalni_potezi {
            potezi_evaluacije.push((potez, vrednost_mata));
        }

        potezi_evaluacije
    }


    pub fn izracunaj_rekursivno_bez_jedenja(&self, vrednost_koju_protivnik_ima_u_dzepu: &Option<f32>, ja_volim_vise:  bool,
        mut broj_rekursija: u8, trenutna_rekursija: u8) -> (f32, bool){
            
            if trenutna_rekursija >= broj_rekursija{
                return self.vrati_nerekursivnu_evaluaciju(vrednost_koju_protivnik_ima_u_dzepu, ja_volim_vise);
            }
        
            let (legalni_potezi, _) = self.svi_legalni_potezi_sortirani_po_jedenju_figura();
            let evaluacija_gotove_partije = self.vrati_evaluaciju_ako_je_partija_gotova(vrednost_koju_protivnik_ima_u_dzepu, &legalni_potezi, ja_volim_vise);
            if evaluacija_gotove_partije.partija_zavrsena {
                return evaluacija_gotove_partije.evaluacija
            }
            
            let mut najbolja_opcija_za_sad: f32 = vrednost_mata(ja_volim_vise);
            for legalan_potez in legalni_potezi {
                let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(&legalan_potez);
                
                let (vrednost_poteza, najbolji_potez) = tabla_nakon_poteza.izracunaj_rekursivno_bez_jedenja(&Some(najbolja_opcija_za_sad), !ja_volim_vise, broj_rekursija, trenutna_rekursija+1);
                if najbolji_potez {
                         najbolja_opcija_za_sad = vrednost_poteza;
                }
                if protivnik_se_zajebo(vrednost_koju_protivnik_ima_u_dzepu, najbolja_opcija_za_sad, ja_volim_vise){
                        return (najbolja_opcija_za_sad, false)
                }   
            }
            
            (najbolja_opcija_za_sad, true)
        
    }

    pub fn je_pozicija_e4_e5_nf3_d6_bc4_bg4_d3(&self) -> bool {
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(G_FILE, 1, F_FILE, 3)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 6)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4)
        .odigraj_validan_potez_bez_promocije(C_FILE, 8, G_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 3);

        self.eq(&tabla)
    }

}



fn skrati_vektor<T>(vektor: &mut Vec<T>, broj_elemenata_koji_ostaje: usize){
    let broj_elemenata = vektor.len();
    let mut broj_elemenata_za_odstranjivanje = broj_elemenata - broj_elemenata_koji_ostaje;
    if broj_elemenata <= broj_elemenata_koji_ostaje {
        broj_elemenata_za_odstranjivanje = 0;
    }

    let mut i: usize = 0;
    while i < broj_elemenata_za_odstranjivanje {
        vektor.pop();
        i += 1;
    }
}


pub fn svaki_n_potez(potezi: &[(Potez_bits, f32)], n: usize, pocetni_indeks: usize) -> Vec<(Potez_bits, f32)>{
    let mut i: usize = pocetni_indeks;
    let broj_poteza: usize = potezi.len();
    let mut svaki_n_potez: Vec<(Potez_bits, f32)> = Vec::new();

    while i<broj_poteza {
        svaki_n_potez.push((potezi[i].0.copy(), potezi[i].1));
        i += n;
    }
    svaki_n_potez
}

fn spoji_2_niza_sortiranih_poteza(mut prvi_niz: Vec<(Potez_bits, f32)>,
drugi_niz: Vec<(Potez_bits, f32)>, broj_poteza_koji_prolaze: usize, ja_sam_beli: bool) -> Vec<(Potez_bits, f32)>{
    for potez_drugog_niza in drugi_niz {
        u_sortiranu_listu(&mut prvi_niz, potez_drugog_niza.0, potez_drugog_niza.1, ja_sam_beli, broj_poteza_koji_prolaze);
    }

    skrati_vektor(&mut prvi_niz, broj_poteza_koji_prolaze);
    prvi_niz
}

pub fn prvi_potez_je_bolji_od_drugog(prvi_potez: f32, drugi_potez: f32, ja_sam_beli: bool) -> bool{
    if ja_sam_beli {
        prvi_potez > drugi_potez
    } else {
        prvi_potez < drugi_potez
    }
}


pub fn vrati_bolji_potez(receiver: &Receiver<f32>, najbolji_potez_ovog_treda: f32, ja_sam_beli: bool) -> f32{
    match receiver.try_recv(){
        Ok(najbolji_potez_drugog_treda) => {
            if prvi_potez_je_bolji_od_drugog(najbolji_potez_drugog_treda, najbolji_potez_ovog_treda, ja_sam_beli){
                return najbolji_potez_drugog_treda;
            } else {
                return najbolji_potez_ovog_treda
            }
        },
        Err(_)=> najbolji_potez_ovog_treda,
    }
}

#[cfg(test)]
mod sah_iteracije_test{
    use crate::tabla::{Tabla, E_PIJUN, E_FILE, potez::Potez_bits, Promocija, C_FILE, A_FILE, B_FILE, LEVI_KONJ, A_PIJUN, B_PIJUN, D_PIJUN, F_PIJUN, F_FILE, G_PIJUN, G_FILE, D_FILE, DESNI_KONJ, C_PIJUN};

    use super::svaki_n_potez;


    #[test]
    fn testiraj_sortiranje_belih_poteza(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let mut potezi = vec![
            (Potez_bits::new(E_PIJUN as u8, E_FILE, 4,Promocija::None), -2.0f32),
            (Potez_bits::new(LEVI_KONJ as u8, C_FILE, 3, Promocija::None), 0.5f32),
            (Potez_bits::new(A_PIJUN as u8, A_FILE, 2, Promocija::None), 0.8),
            (Potez_bits::new(B_PIJUN as u8, B_FILE, 3, Promocija::None), 0.2), 
        ];
        tabla.sortiraj_poteze(&mut potezi);
        assert_eq!(E_PIJUN as u8, potezi[3].0.broj_figure);
        assert_eq!(LEVI_KONJ as u8, potezi[1].0.broj_figure);
        assert_eq!(A_PIJUN as u8, potezi[0].0.broj_figure);
        assert_eq!(B_PIJUN as u8, potezi[2].0.broj_figure);
        
    }

    #[test]
    fn testiraj_sortiranje_crnih_poteza(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4);

        let mut potezi = vec![
            (Potez_bits::new(E_PIJUN as u8, E_FILE, 5,Promocija::None), -0.4),
            (Potez_bits::new(D_PIJUN as u8, D_FILE, 5, Promocija::None), 0.2),
            (Potez_bits::new(F_PIJUN as u8, F_FILE, 5,Promocija::None), -1.1),
            (Potez_bits::new(G_PIJUN as u8, G_FILE, 5, Promocija::None), -1.5)
        ];
        tabla.sortiraj_poteze(&mut potezi);

        assert_eq!(G_PIJUN as u8, potezi[0].0.broj_figure);
        assert_eq!(F_PIJUN as u8, potezi[1].0.broj_figure);
        assert_eq!(E_PIJUN as u8, potezi[2].0.broj_figure);
        assert_eq!(D_PIJUN as u8, potezi[3].0.broj_figure);
    }


    #[test]
    fn test_svaki_n_potez(){
        let potezi = vec![
            (Potez_bits::new(E_PIJUN as u8, E_FILE, 4, Promocija::None), 0.5),
            (Potez_bits::new(LEVI_KONJ as u8, C_FILE, 3, Promocija::None), 1.0),
            (Potez_bits::new(DESNI_KONJ as u8, F_FILE, 3, Promocija::None), 0.25),
            (Potez_bits::new(D_PIJUN as u8, D_FILE, 4, Promocija::None), 0.75),
            (Potez_bits::new(C_PIJUN as u8, C_FILE, 4, Promocija::None), 0.25),
        ];

        let prvi_potezi = svaki_n_potez(&potezi, 2, 0);
        let drugi_potezi = svaki_n_potez(&potezi, 2, 1);

        assert_eq!(3, prvi_potezi.len());
        assert_eq!(2, drugi_potezi.len());

        assert_eq!(true, drugi_potezi.contains(&(Potez_bits::new(LEVI_KONJ as u8, C_FILE, 3, Promocija::None), 1.0)));
        assert_eq!(true, drugi_potezi.contains(&(Potez_bits::new(D_PIJUN as u8, D_FILE, 4, Promocija::None), 0.75)));

        assert_eq!(true, prvi_potezi.contains(&(Potez_bits::new(DESNI_KONJ as u8, F_FILE, 3, Promocija::None), 0.25)));
        assert_eq!(true, prvi_potezi.contains(&(Potez_bits::new(C_PIJUN as u8, C_FILE, 4, Promocija::None), 0.25)));
        assert_eq!(true, prvi_potezi.contains(&(Potez_bits::new(E_PIJUN as u8, E_FILE, 4, Promocija::None), 0.5)));
        
    }
}