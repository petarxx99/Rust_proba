
use crate::{tabla::{Tabla, potez::Potez_bits}, proba_sah_drveta::{Evaluacija_poteza_jedenja, najgori_eval_poteza_jedenja, vrati_evaluaciju_poteza, vrednost_mata, protivnik_se_zajebo, Evaluacija, vrati_evaluaciju_poteza_jedenja, ovo_je_refutacija_protivnikovog_poteza, ovo_je_najbolji_potez}};

static broj_kandidata: usize = 4;

fn ubaci_u_sortiranu_listu_poteza(lista: &mut Vec<(Potez_bits, f32)>, novi_potez: Potez_bits, vrednost_poteza: f32, ja_sam_beli: bool){
    if lista.len() == 0{
        lista.push((novi_potez, vrednost_poteza));
        return;
    }

    let broj_poteza = lista.len();
    let mut i: usize = 0;
    while i<broj_poteza{
        if (ja_sam_beli && vrednost_poteza > lista[i].1) || (!ja_sam_beli && vrednost_poteza < lista[i].1){
            lista.insert(i, (novi_potez, vrednost_poteza));
            if lista.len() > broj_kandidata {
                lista.pop();
            }
            return;
        }
        i+=1;
    }

}

impl Tabla{

    pub fn najbolji_potez_i_njegova_evaluacija_u_talasima(&self, dubina: u8) -> (Option<Potez_bits>, f32) {
        let ja_sam_beli: bool = self.beli_je_na_potezu();
        let protivnik_je_beli: bool = !ja_sam_beli;

        let mut potezi_kandidati = self.pronadji_kandidate_za_talase(dubina);
        let nova_dubina = dubina + 2;
        let mut najbolja_evaluacija: f32 = vrednost_mata(ja_sam_beli);
        let mut najbolji_potez: Option<Potez_bits> = None;

        for (potez, evaluacija) in & mut potezi_kandidati {
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

    pub fn pronadji_kandidate_za_talase(&self, pocetna_dubina: u8) -> Vec<(Potez_bits,f32)>{
        let protivnik_je_beli: bool = !self.beli_je_na_potezu();
        let ja_sam_beli: bool = self.beli_je_na_potezu();

        let mut najbolji_potezi: Vec<(Potez_bits, f32)> = Vec::new();
        let mut najbolja_evaluacija: f32 = vrednost_mata(ja_sam_beli);

        let (legalni_potezi, _) = self.svi_legalni_potezi_sortirani_po_jedenju_figura();
        for potez in legalni_potezi {
            let tabla: Tabla = self.tabla_nakon_poteza_bits(&potez);
            let (vrednost_poteza, _) = tabla.izracunaj_rekursivno(&Some(najbolja_evaluacija), protivnik_je_beli, pocetna_dubina, 1,  self.materijalna_prednost_onog_ko_je_na_potezu(), vrednost_mata(!self.beli_je_na_potezu()), false);
            if ovo_je_najbolji_potez(najbolja_evaluacija, vrednost_poteza, ja_sam_beli){
                ubaci_u_sortiranu_listu_poteza(&mut najbolji_potezi, potez.copy(), vrednost_poteza, ja_sam_beli);
                najbolja_evaluacija = najbolji_potezi[najbolji_potezi.len() -1].1;
            }
        }

        najbolji_potezi
    }

    pub fn evaluiraj_gledajuci_poteze_jedenja_nezahtevno(&self, vrednost_koju_protivnik_ima_u_dzepu: &Option<f32>,
        materijalno_stanje: f32, materijal_proslog_poteza:f32, materijal_pretproslog_poteza: f32, ja_volim_vise:bool) -> (f32, bool){
                
            let mali_broj: f32 = 0.125;
                if materijalno_stanje + mali_broj > materijal_pretproslog_poteza {
                    return self.vrati_nerekursivnu_i_nezahtevnu_evaluaciju_koja_uzima_u_obzir_da_li_je_mat(vrednost_koju_protivnik_ima_u_dzepu, ja_volim_vise)
                }
                let najgora_evaluacija_za_protivnika: Evaluacija_poteza_jedenja = najgori_eval_poteza_jedenja(!ja_volim_vise);
                let eval = self.izracunaj_rekursivno_samo_jedenje_figura_nezahtevno(&najgora_evaluacija_za_protivnika, ja_volim_vise, materijal_proslog_poteza, materijal_pretproslog_poteza);
                return vrati_evaluaciju_poteza(vrednost_koju_protivnik_ima_u_dzepu, eval.kompletna_evaluacija, ja_volim_vise)
        }
        
        pub fn izracunaj_rekursivno_zove_nezahtevne_funkcije(&self, vrednost_koju_protivnik_ima_u_dzepu: &Option<f32>, ja_volim_vise:  bool,
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
        
        pub fn izracunaj_rekursivno_samo_jedenje_figura_nezahtevno(&self, 
            vrednost_koju_protivnik_ima_u_dzepu: &Evaluacija_poteza_jedenja, ja_sam_beli:  bool, 
            materijal_proslog_poteza: f32, materijal_pretproslog_poteza: f32) -> Evaluacija_poteza_jedenja {
        
            let (beli_materijal, crni_materijal, _,_) = self.evaluacija_materijala(ja_sam_beli);    
            let beli_minus_crni_materijal: f32 = beli_materijal - crni_materijal;
            let materijal: f32 = self.materijalna_prednost_onog_ko_je_na_potezu();
            let nerekursivna_evaluacija: f32 = self.nerekursivno_i_nezahtevno_evaluiraj_poziciju(&self.to_nekompresirana_tabla());
        
            let mali_broj: f32 = 0.125;
            if materijal + mali_broj > materijal_pretproslog_poteza{
                return vrati_evaluaciju_poteza_jedenja(vrednost_koju_protivnik_ima_u_dzepu, beli_minus_crni_materijal, nerekursivna_evaluacija, ja_sam_beli);
            }
        
            let (legalni_potezi, broj_poteza_jedenja) = self.svi_legalni_potezi_sortirani_po_jedenju_figura();
            let evaluacija_gotove_partije: Evaluacija = self.vrati_evaluaciju_ako_je_partija_gotova(&Some(vrednost_koju_protivnik_ima_u_dzepu.kompletna_evaluacija), &legalni_potezi, ja_sam_beli);
            if evaluacija_gotove_partije.partija_zavrsena {
                let (evaluacija, protivnik_se_nije_zajebo):(f32, bool) = evaluacija_gotove_partije.evaluacija;
                return Evaluacija_poteza_jedenja::new(beli_minus_crni_materijal, evaluacija, protivnik_se_nije_zajebo);
            }
        
            let potezi_jedenja = &legalni_potezi[0..broj_poteza_jedenja];
            if broj_poteza_jedenja == 0 {
                return vrati_evaluaciju_poteza_jedenja(vrednost_koju_protivnik_ima_u_dzepu, beli_minus_crni_materijal, nerekursivna_evaluacija, ja_sam_beli);
            }
            
            let evaluacija_koju_imam_u_dzepu_ako_ne_odigram_nista = vrati_evaluaciju_poteza_jedenja(vrednost_koju_protivnik_ima_u_dzepu, beli_minus_crni_materijal, nerekursivna_evaluacija, ja_sam_beli);
            let mut najbolji_potez_do_sad: Evaluacija_poteza_jedenja = evaluacija_koju_imam_u_dzepu_ako_ne_odigram_nista;
            for potez in potezi_jedenja {
                let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(potez);
        
                let eval_poteza: Evaluacija_poteza_jedenja = tabla_nakon_poteza.izracunaj_rekursivno_samo_jedenje_figura_nezahtevno(&najbolji_potez_do_sad, !ja_sam_beli, materijal, materijal_proslog_poteza);
                if eval_poteza.ovo_je_najbolja_varijacija_do_sad {
                    najbolji_potez_do_sad.kompletna_evaluacija = eval_poteza.kompletna_evaluacija;
                    najbolji_potez_do_sad.evaluacija_po_materijalu = eval_poteza.evaluacija_po_materijalu;
                    if ovo_je_refutacija_protivnikovog_poteza(&vrednost_koju_protivnik_ima_u_dzepu, &eval_poteza, ja_sam_beli){
                        najbolji_potez_do_sad.ovo_je_najbolja_varijacija_do_sad = false;
                        return najbolji_potez_do_sad
                    }
                }
            }
        
            najbolji_potez_do_sad
        }
        
        
}