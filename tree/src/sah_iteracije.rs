use crate::proba_sah_drveta::{vrednost_mata, protivnik_se_zajebo, ovo_je_najbolji_potez};
use crate::tabla::potez::Potez_bits;
use crate::tabla::{Tabla};
static MAX_BROJ_POTEZA_KANDIDATA: usize = 3;

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

    pub fn najbolji_potez_i_njegova_evaluacija_putem_iteracija(&self, dubina: u8) -> (Option<Potez_bits>, f32) {
        let ja_sam_beli: bool = self.beli_je_na_potezu();
        let protivnik_je_beli: bool = !ja_sam_beli;

        let mut potezi_kandidati = self.pronadji_kandidate_preko_iteracija(dubina);
        self.sortiraj_poteze(&mut potezi_kandidati);
        println!("broj poteza kandidata: {}", potezi_kandidati.len());
        for potez in &potezi_kandidati {println!("\npotez kandidata: {}\n", &potez.0);}

        let maksimum_poteza = minimum(potezi_kandidati.len(), MAX_BROJ_POTEZA_KANDIDATA);
        let potezi_kandidati = &mut potezi_kandidati[0..maksimum_poteza];
        
        let nova_dubina = dubina + 2;
        let mut najbolja_evaluacija: f32 = vrednost_mata(ja_sam_beli);
        let mut najbolji_potez: Option<Potez_bits> = None;

        for (potez, evaluacija) in potezi_kandidati {
            let tabla: Tabla = self.tabla_nakon_poteza_bits(&potez);
            let (vrednost_poteza, _) = tabla.izracunaj_rekursivno_zove_nezahtevne_funkcije(&Some(najbolja_evaluacija), protivnik_je_beli, nova_dubina, 1,  self.materijalna_prednost_onog_ko_je_na_potezu(), vrednost_mata(!self.beli_je_na_potezu()), false);
            *evaluacija = vrednost_poteza;
            if ovo_je_najbolji_potez(najbolja_evaluacija, vrednost_poteza, ja_sam_beli){
                najbolja_evaluacija = vrednost_poteza;
                najbolji_potez = Some(potez.copy());
            }
        }

        (najbolji_potez, najbolja_evaluacija)
    }


    pub fn pronadji_kandidate_preko_iteracija(&self, dubina: u8) -> Vec<(Potez_bits, f32)>{
        let ja_volim_vise: bool = self.beli_je_na_potezu();
        let najgora_evaluacija: f32 = vrednost_mata(ja_volim_vise);

        let mut potezi_evaluacije: Vec<(Potez_bits, f32)> = self.init_potezi_evaluacije();
        let dubina_sa_2: usize = (dubina / 2) as usize;
    
        let mut broj_poteza_koji_prolaze: usize = potezi_evaluacije.len();
        let mut i: usize = 1;
        while i <= dubina_sa_2 {

            let broj_rekursija: u8 = (i*2) as u8;
            let mut potezi_koji_prolaze: Vec<(Potez_bits, f32)> = Vec::new();
            let mut najlosiji_potez_koji_prolazi: f32 = vrednost_mata(ja_volim_vise);

            for (potez, evaluacija) in &potezi_evaluacije{
                let tabla: Tabla = self.tabla_nakon_poteza_bits(potez);
                let (vrednost_poteza, _) = tabla.izracunaj_rekursivno_bez_gledanja_saha(&Some(najlosiji_potez_koji_prolazi), ja_volim_vise, broj_rekursija, 1, 0.0, 0.0, false);
                u_sortiranu_listu(&mut potezi_koji_prolaze, potez.copy(), vrednost_poteza, ja_volim_vise, broj_poteza_koji_prolaze);
                najlosiji_potez_koji_prolazi = nabavi_najlosiji_potez_koji_prolazi(&potezi_koji_prolaze, broj_poteza_koji_prolaze, ja_volim_vise);
            }
            self.sortiraj_poteze(&mut potezi_koji_prolaze);
            broj_poteza_koji_prolaze = maksimum(potezi_koji_prolaze.len() / 2, MAX_BROJ_POTEZA_KANDIDATA);
            skrati_vektor(&mut potezi_koji_prolaze, broj_poteza_koji_prolaze);
            potezi_evaluacije = potezi_koji_prolaze;

            i+=1;
        }


        potezi_evaluacije
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



#[cfg(test)]
mod sah_iteracije_test{
    use crate::tabla::{Tabla, E_PIJUN, E_FILE, potez::Potez_bits, Promocija, C_FILE, A_FILE, B_FILE, LEVI_KONJ, A_PIJUN, B_PIJUN, D_PIJUN, F_PIJUN, F_FILE, G_PIJUN, G_FILE, D_FILE};


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
}