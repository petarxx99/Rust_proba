use std::fmt::Display;

use super::{Rokada, Promocija, Tabla, KRALJ, File_rank, DESNI_TOP, F_FILE, LEVI_TOP, D_FILE, Figura, kretanje_figura::figure::abs, A_PIJUN};
mod provera_legalnosti;
mod legalni_potezi;
mod obrada_legalnih_poteza;

pub struct Potez_info {
    pub rokada_onemogucena: Rokada,
    pub file_pijuna_pomerenog_2_polja: Option<u32>,
    pub pijun_pomeren_ili_figura_pojedena: bool,
    pub beli_je_odigrao_potez: bool,
}
impl Potez_info {
    pub fn new() -> Potez_info{
        Potez_info { 
            rokada_onemogucena: Rokada::new_sve_rokade_moguce(),
            file_pijuna_pomerenog_2_polja: None, 
            pijun_pomeren_ili_figura_pojedena: false,
            beli_je_odigrao_potez: true
         }
    }

    pub fn zapisi_info_ako_je_pomeren_pijun(&mut self, figure: &[u8;16], potez: &Potez_bits) {
        if Tabla::proveri_da_li_je_pomeren_pijun(figure, potez.broj_figure as usize) {
            self.pijun_pomeren_ili_figura_pojedena = true;
            let (start_rank, start_file) = crate::broj_to_rank_file(figure[potez.broj_figure as usize]);
           
            if (start_rank as i32 - potez.rank as i32 == 2) || (potez.rank as i32 - start_rank as i32 == 2) {
                self.file_pijuna_pomerenog_2_polja = Some(potez.file as u32);
            }
        }
    }

    fn updejtuj_bitfield_nakon_odigranog_poteza(&self, mut bitfield: i32) -> i32{
        bitfield = Tabla::obrni_ko_je_na_potezu(bitfield);
        bitfield = Tabla::onemoguci_rokadu(bitfield, &self.rokada_onemogucena);
        if self.pijun_pomeren_ili_figura_pojedena {
            bitfield = Tabla::resetuj_50_move_rule(bitfield);
        } else {
            bitfield = Tabla::povecaj_50_move_rule_brojac_za_1_unsafe(bitfield);
        }

        bitfield = Tabla::resetuj_fajl_pijuna_koji_se_pomerio_dva_polja(bitfield);
        match self.file_pijuna_pomerenog_2_polja {
            None => {},
            Some(file) => {
                bitfield = Tabla::dodaj_fajl_pijuna_koji_se_pomerio_2_polja(bitfield, file as i32);
            }
        }

        bitfield
    }


    fn updejtuj_figure_nakon_odigranog_poteza(&mut self, mut bele_figure: &mut [u8; 16], mut crne_figure: &mut [u8;16], potez: &Potez_bits, beli_je_odigrao_potez: bool, 
        fajl_en_passant_pijuna: Option<u8>, tabla: &Tabla){
        let figure_koje_su_odigrale_potez: &mut [u8;16];
        let figure_protiv_kojih_je_odigran_potez: &mut [u8;16];

        if beli_je_odigrao_potez {
            figure_koje_su_odigrale_potez = &mut bele_figure;
            figure_protiv_kojih_je_odigran_potez = &mut crne_figure;
        } else {
            figure_koje_su_odigrale_potez = &mut crne_figure;
            figure_protiv_kojih_je_odigran_potez = &mut bele_figure;
        }
        /* Ova linija mora da ide pre updejta, jer zelim poziciju figure pre nego sto se pomerila. */
        let (start_rank, start_file) = crate::broj_to_rank_file(figure_koje_su_odigrale_potez[potez.broj_figure as usize]);
        self.updejtuj_figure_koje_su_odigrale_potez(figure_koje_su_odigrale_potez, potez, tabla);
        let figura_pojedena: bool = self.updejtuj_figure_protiv_kojih_je_odigran_potez(figure_protiv_kojih_je_odigran_potez, potez);

        /* Ako se pijun pomerio ukoso, a nije stao na polje na kom se nalazi neka protivnicka figura,
        to je jedino moguce prilikom en passant. */
       let potez_je_en_passant: bool = abs(start_file as i32 - potez.file as i32) == 1 
       && !figura_pojedena
       && Tabla::figura_je_pijun(&figure_koje_su_odigrale_potez, potez.broj_figure as usize);
       if potez_je_en_passant {
        match fajl_en_passant_pijuna{
            None => {}, /* Pretpostavka je da fajlovi idu od 1 do 8, pijuni se nalaze od 8. do 15. mesta u nizu. */
            Some(_file_en_passant_pojedenog_pijuna) => {
                Tabla::prati_polozaj_kralja(figure_protiv_kojih_je_odigran_potez, 7 + _file_en_passant_pojedenog_pijuna as usize);
            }
        }
       }
       
    }

/* Posebno obradjujem slucaj kad se pomera kralj, jer kralj ima istu lokaciju kao figure koje
su sklonjene sa table. */
    fn updejtuj_figure_koje_su_odigrale_potez(&mut self, figure: & mut[u8; 16], potez: &Potez_bits, tabla: &Tabla){
        if potez.broj_figure == KRALJ as u8{
            self.rokada_onemogucena.pomeren_kralj(self.beli_je_odigrao_potez);
            Tabla::pomeri_kralja(figure, potez.file, potez.rank);
            return;
        }

        self.zapisi_info_ako_je_pomeren_pijun(figure, potez);
        self.zapisi_info_za_rokadu(potez, tabla);

        Tabla::updejtuj_polozaj_figure_unsafe(figure, potez.broj_figure as usize,
             &File_rank::new(potez.file, potez.rank));

        let promocija: &Promocija = &potez.promocija;
        match promocija {
                &Promocija::None => {},
                _promocija => {
                     Tabla::promovisi_pijuna(figure, potez.broj_figure as usize, _promocija);
                }
        }       
    } 
    /*https://rust-lang.github.io/rfcs/2005-match-ergonomics.html
    https://stackoverflow.com/questions/36590549/what-is-the-syntax-to-match-on-a-reference-to-an-enum */

    fn updejtuj_figure_protiv_kojih_je_odigran_potez(&mut self, figure: &mut[u8;16], potez: &Potez_bits) -> bool{
        let polje_destinacije: u8 = crate::file_rank_to_broj(potez.file, potez.rank);
    
        
        for i in 0..16 {
            if Tabla::polja_se_slazu(polje_destinacije, figure[i]){
                self.pijun_pomeren_ili_figura_pojedena = true;
                Tabla::prati_polozaj_kralja(figure, i);
                return true;
            } 
        }

        false
    }

    fn zapisi_info_za_rokadu(&mut self, potez: &Potez_bits, tabla: &Tabla){
        let zavrsno_polje: u8 = crate::file_rank_to_broj(potez.file, potez.rank);

         if (self.beli_je_odigrao_potez && potez.broj_figure == LEVI_TOP as u8) ||
         (!self.beli_je_odigrao_potez && Tabla::polja_se_slazu(tabla.bele_figure[LEVI_TOP], zavrsno_polje))
         {
            self.rokada_onemogucena.bela_kraljicina_rokada_vise_nije_moguca = true;
         }

         if (self.beli_je_odigrao_potez && potez.broj_figure == DESNI_TOP as u8) || 
         (!self.beli_je_odigrao_potez && Tabla::polja_se_slazu(tabla.bele_figure[DESNI_TOP], zavrsno_polje)){
            self.rokada_onemogucena.bela_kraljeva_rokada_vise_nije_moguca = true;
         }
         if !self.beli_je_odigrao_potez && potez.broj_figure == LEVI_TOP as u8 || 
         (self.beli_je_odigrao_potez && Tabla::polja_se_slazu(tabla.crne_figure[LEVI_TOP], zavrsno_polje)){
            self.rokada_onemogucena.crna_kraljicina_rokada_vise_nije_moguca = true;
         }
         if !self.beli_je_odigrao_potez && potez.broj_figure == DESNI_TOP as u8 || 
         (self.beli_je_odigrao_potez && Tabla::polja_se_slazu(tabla.crne_figure[DESNI_TOP], zavrsno_polje))
         {
            self.rokada_onemogucena.crna_kraljeva_rokada_vise_nije_moguca = true;
         }
       
    }

}

pub struct Potez_polje{
    pub start_file: u8,
    pub start_rank: u8,
    pub end_file: u8,
    pub end_rank: u8,
}

impl Potez_polje{
    pub fn new(start_file: u8, start_rank: u8, end_file: u8, end_rank: u8) -> Potez_polje {
        Potez_polje { start_file, start_rank, end_file, end_rank}
    }
    pub fn new_iz_broja(start_polje: u8, end_file: u8, end_rank: u8) -> Potez_polje {
        let (start_rank, start_file) = crate::broj_to_rank_file(start_polje);
        Potez_polje{start_file, start_rank, end_file, end_rank}
    }
}

pub struct Potez{
    pub start_file: u8,
    pub start_rank: u8,

    pub file_destinacije: u8,
    pub rank_destinacije: u8,
    pub promocija: Promocija,
}

impl Display for Potez {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "start rank: {}, start file: {}\n end rank: {}, end file: {}",
    self.start_rank, self.start_file, self.rank_destinacije, self.file_destinacije)
    }
}

impl Potez{
    pub fn copy(&self) -> Potez{
        Potez{
            start_file: self.start_file,
            start_rank: self.start_rank,
            rank_destinacije: self.rank_destinacije,
            file_destinacije: self.file_destinacije,
            promocija: self.promocija.copy()
        }   
    }
}


#[repr(C)]
pub struct Potez_bits {
    pub broj_figure: u8,
    pub file: u8,
    pub rank: u8,
    pub promocija: Promocija,
}
pub fn print_size_of_Potez_bits(){
    println!("{}", std::mem::size_of::<Potez_bits>());
}
impl PartialEq for Potez_bits{
    fn eq(&self, other: &Self) -> bool {
        if !(self.broj_figure == other.broj_figure && self.file == other.file && self.rank == other.rank) {
            return false
        }
    
        (&self.promocija).copy() as usize == (&other.promocija).copy() as usize
    }
}

impl Display for Potez_bits {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {        
        write!(formatter, "potez: \nredni broj figure: {}, rank: {}, file: {}", self.broj_figure, self.rank, self.file)
    }
}

impl Potez_bits {
    pub fn copy(&self) -> Potez_bits{
        Potez_bits {
            broj_figure: self.broj_figure,
            file: self.file,
            rank: self.rank,
            promocija: self.promocija.copy()
        }
    }

    pub fn new(broj_figure: u8, file: u8, rank: u8, promocija: Promocija) -> Potez_bits{
        Potez_bits {
            broj_figure, rank, file, promocija
        }
    }

    pub fn to_Potez_polje(&self, figure: &[u8;16]) -> Potez_polje{
        let (start_rank, start_file) = crate::broj_to_rank_file(figure[self.broj_figure as usize]);
        Potez_polje { start_file, start_rank, end_file: self.file, end_rank:self.rank }
    }

    pub fn to_Potez(&self, figure: &[u8;16]) -> Potez {

        let (start_rank, start_file) = crate::broj_to_rank_file(figure[self.broj_figure as usize]);
        Potez::new(start_file, start_rank, self.file, self.rank, self.promocija.copy())
    }

    
}

impl Potez {

    pub fn new(start_file: u8, start_rank: u8, file_destinacije: u8, rank_destinacije: u8, promocija: Promocija) -> Potez{
        Potez{start_file, start_rank, file_destinacije, rank_destinacije, promocija}
    }

    pub fn to_Potez_bits(&self, tabla: &Tabla) -> Option<Potez_bits> {
        let mut figure: &[u8;16];
     /* Potez se kombinuje sa tablom (tj. pozicijom) nad kojom se potez igra, tako da je boja igraca
     koji odigrao potez ista ona koju ima tabla nad kojom potez treba da se odigra. */
        if tabla.beli_je_na_potezu() {
            figure = &tabla.bele_figure;
        } else {
            figure = &tabla.crne_figure;
        }

        
   /* Kralja treniram kao specijalni slucaj, jer figure koje su sklonjene sa table imaju istu lokaciju kao kralj.
*/     
        if Tabla::to_je_file_rank_polja(figure[KRALJ], self.start_file, self.start_rank){
            return Some(Potez_bits{broj_figure: KRALJ as u8, file: self.file_destinacije, rank: self.rank_destinacije, promocija: self.promocija.copy()})
        }

        for broj_figure in 0..figure.len() {
            if Tabla::to_je_file_rank_polja(figure[broj_figure], self.start_file, self.start_rank) {
                return Some(Potez_bits { broj_figure: broj_figure as u8, file: self.file_destinacije, rank: self.rank_destinacije, promocija: self.promocija.copy()})
            }
        }
        None
    }

    pub fn to_Potez_polje(&self)->Potez_polje{
        Potez_polje { start_file: self.start_file, start_rank: self.start_rank, end_file: self.file_destinacije, end_rank:self.rank_destinacije }
    }
}


impl Tabla {
    pub fn odigraj_potez_bez_promocije_unsafe(&self, potez: &Potez_polje) -> Tabla{
        self.tabla_nakon_validnog_poteza(&Potez{start_file: potez.start_file, start_rank: potez.start_rank, file_destinacije: potez.end_file, rank_destinacije: potez.end_rank, promocija: Promocija::None})
    }

    pub fn odigraj_validan_potez_bez_promocije(&self, start_file: u8, start_rank: u8, file_destinacije: u8, rank_destinacije: u8) -> Tabla {
        self.tabla_nakon_validnog_poteza(&Potez{start_file, start_rank, file_destinacije, rank_destinacije, promocija: Promocija::None})
    }

    pub fn tabla_nakon_validnog_poteza(&self, potez: &Potez) -> Tabla{
        let Potez_bits: Potez_bits = potez.to_Potez_bits(self).expect("Uneli ste nevalidan potez.");
        let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(&Potez_bits);
        tabla_nakon_poteza
    }

    pub fn tabla_nakon_poteza_bits(&self, potez: &Potez_bits) -> Tabla{
        let mut bele_figure: [u8; 16] = self.bele_figure.clone();
        let mut crne_figure: [u8; 16] = self.crne_figure.clone();
        let mut bitfield: i32 = self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu;
       
        let mut potez_info: Potez_info = Potez_info::new();
        potez_info.beli_je_odigrao_potez = self.beli_je_na_potezu();

        potez_info.updejtuj_figure_nakon_odigranog_poteza(& mut bele_figure, &mut crne_figure, potez, potez_info.beli_je_odigrao_potez, self.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu(), self);
        bitfield = potez_info.updejtuj_bitfield_nakon_odigranog_poteza(bitfield);
    
        Tabla {
            bele_figure,
            crne_figure,
            sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu: bitfield,
        }
    }


    fn prati_polozaj_kralja(figure: &mut[u8;16], broj_figure: usize){

        let (rank, file) = crate::broj_to_rank_file(figure[KRALJ]);
        Tabla::updejtuj_polozaj_figure_unsafe(figure, broj_figure, &File_rank{file, rank});
    }


    fn pomeri_kralja(figure: & mut[u8;16], file: u8, rank: u8){
        let polozaj_kralja: u8 = figure[KRALJ];

        Tabla::updejtuj_polozaj_figure_unsafe(figure, KRALJ,
             &File_rank{file, rank});

      /* Sledi kod koji se brine za rokadu. */       
        let (start_rank, start_file) = crate::broj_to_rank_file(polozaj_kralja);

        if file as i32 - start_file as i32 == 2 { /* kraljeva rokada */
            Tabla::updejtuj_polozaj_figure_unsafe(figure, DESNI_TOP, &File_rank{file: F_FILE, rank});
        } else if start_file as i32 - file as i32 == 2{ /* kraljicina rokada */
            Tabla::updejtuj_polozaj_figure_unsafe(figure, LEVI_TOP, &File_rank{file: D_FILE, rank});
        }

/* Pojedene figure prate poziciju kralja. */
        for i in 0..figure.len() {
            if Tabla::polja_se_slazu(polozaj_kralja, figure[i]){
                Tabla::prati_polozaj_kralja(figure, i);
            }
        }

    }

    pub fn figura_je_pojedena(figure: &[u8;16], redni_broj_figure: usize) -> bool {
        if redni_broj_figure == KRALJ {
            return false;
        }
        Tabla::polja_se_slazu(figure[KRALJ], figure[redni_broj_figure])
    }
}


pub static MAX_EVALUACIJA: i8 = 99;
pub static MIN_EVALUACIJA: i8 = -99;

impl Figura {
    pub fn vrednost(&self) -> f32 {
        match *self {
            Figura::KRALJ => 0.0,
            Figura::KRALJICA => 9.0,
            Figura::TOP => 5.0,
            Figura::LOVAC => 3.5,
            Figura::KONJ => 3.0,
            Figura::PIJUN => 1.0
        }
    }
}

impl Tabla {
    pub fn ukupna_vrednost_nepojedenih_figura(figure: &[u8;16]) -> u8 {
        let mut vrednost_nepojedenih_figura: f32 = 0.0;
     /* Mesta 1,2,3,4,5,6,7 cuvaju figure koje nisu pijuni (broj 0 je kralj, zato ga preskacem.) */   
        for i in 1..8 {
            if !Tabla::figura_je_pojedena(figure, i){
                let figura: Figura = Figura::map_redni_broj_to_figure_unsafe(i).unwrap();
                vrednost_nepojedenih_figura += figura.vrednost();
            }
        }
    /* Sada obradjujem pijune i figure koje su nastale promocijom pijuna. */    
        for i in 8..16 {
            if Tabla::figura_je_pojedena(figure, i) {
                continue;
            }
            if Tabla::pijun_je_promovisan(figure[i]){
                let promovisana_figura: Figura = Tabla::u_sta_je_pijun_promovisan(figure, i);
                vrednost_nepojedenih_figura += promovisana_figura.vrednost();
            } else {
                vrednost_nepojedenih_figura += Figura::PIJUN.vrednost();
            }
        }

        let rezultat = vrednost_nepojedenih_figura as u8;
        rezultat
    } 

    pub fn broj_nepojedenih_figura(figure: &[u8;16]) -> u8 {
        let mut broj_nepojedenih_figura=1;
        for i in 1..16 {
            if !Tabla::figura_je_pojedena(figure, i){
                broj_nepojedenih_figura += 1;
            }
        }
        broj_nepojedenih_figura
    }

    /* Ova funkcija zavisi od toga da je kralj na prvom mestu u nizu figura. Zasto?
    Zato sto figure koje su sklonjene sa table imaju istu lokaciju kao i kralj. Ako proveravam polje
    na kom se nalazi kralj i prvo proverim neku pojedenu figuru, funkcija bi pogresno vratila tu 
    pojedenu figuru umesto kralja. */
    pub fn koja_figura_se_figura_nalazi_na_polju(file_rank_polja: &File_rank, figure: &[u8;16]) 
    -> Option<Figura> {
        for i in 0..8{           
            /* Ako se figura nalazi na polju koje trazim onda vracam tu figuru. */
            if Tabla::polja_se_slazu(figure[i], crate::file_rank_to_broj(file_rank_polja.file, file_rank_polja.rank)){
                return Some(Figura::map_redni_broj_to_figure_unsafe(i).unwrap());
            }
        }

        for i in 8..16 {
            if !Tabla::polja_se_slazu(figure[i], crate::file_rank_to_broj(file_rank_polja.file, file_rank_polja.rank)){
                continue; /* Ako se ova figura ne nalazi na polju koje trazim onda preskacem iteraciju. */
            }

            if !Tabla::pijun_je_promovisan(figure[i]){
                return Some(Figura::PIJUN);
            }
            return Some(Tabla::u_sta_je_pijun_promovisan(figure, i));
        }
        None
    }

    pub fn polje_je_prazno_preko_broja(&self, polje: u8) -> bool {
        let (rank, file) = crate::broj_to_rank_file(polje);
        self.polje_je_prazno(&File_rank{file, rank})
    }


    pub fn polje_je_prazno(&self, file_rank_polja: &File_rank) -> bool {
        match Tabla::koja_figura_se_figura_nalazi_na_polju(file_rank_polja, &self.bele_figure) {
            None => {},
            Some(_) => {return false;}
        }
       
        match Tabla::koja_figura_se_figura_nalazi_na_polju(file_rank_polja, &self.crne_figure){
            None => true,
            Some(_) => false
        }
    }
}

#[cfg(test)]
mod potez_tests{
    use crate::tabla::{Tabla, E_FILE, B_FILE, C_FILE, F_FILE, LEVI_KONJ, DESNI_LOVAC, Promocija, G_FILE, DESNI_TOP, File_rank, A_FILE, D_FILE, H_FILE, DESNI_KONJ, LEVI_TOP, F_PIJUN, LEVI_LOVAC, KRALJ};

    use super::{Potez, Potez_bits, Potez_info};
    use crate::tabla::{Figura};

    #[test]
    fn info_o_pijunu(){
        let potez: Potez_bits = Potez_bits{broj_figure: 12, file: E_FILE, rank: 4, promocija: Promocija::None};
        let mut potez_info: Potez_info = Potez_info::new();
        let mut tabla: Tabla = Tabla::pocetna_pozicija();
        potez_info.zapisi_info_ako_je_pomeren_pijun(&tabla.bele_figure, &potez);
        assert_eq!(potez_info.file_pijuna_pomerenog_2_polja.unwrap(), E_FILE as u32);

        let bitfield: i32 = potez_info.updejtuj_bitfield_nakon_odigranog_poteza(tabla.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu);
        tabla.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu = bitfield;
        assert_eq!(tabla.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu().unwrap(), E_FILE);
    }

    #[test]
    fn testiraj_tabla_nakon_poteza_bits(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let Potez_bits: Potez_bits = Potez_bits{broj_figure: 14, file: F_FILE, rank: 4, promocija: Promocija::None};
        let tabla2: Tabla = tabla.tabla_nakon_poteza_bits(&Potez_bits);
        assert_eq!(F_FILE, tabla2.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu().unwrap());
        assert_eq!(false, tabla2.beli_je_na_potezu());

        let tabla3: Tabla = tabla2.tabla_nakon_poteza_bits(&Potez_bits { broj_figure: LEVI_KONJ as u8, file: B_FILE, rank: 3, promocija: Promocija::None });
        assert_eq!(1, tabla3.pre_koliko_poteza_je_50_move_rule_pomeren());
        assert_eq!(true, tabla3.beli_je_na_potezu());    
    }

    #[test]
    fn to_Potez_bits(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let potez: Potez = Potez::new(E_FILE, 2, E_FILE, 4, Promocija::None);
        let Potez_bits: Potez_bits = potez.to_Potez_bits(&tabla).unwrap();
        assert_eq!(12, Potez_bits.broj_figure);
        assert_eq!(E_FILE, Potez_bits.file);
        assert_eq!(4, Potez_bits.rank);
    }

    #[test]
    fn desni_beli_top_se_pomeri_kao_da_je_rokada_ako_se_kralj_pomeri_za_2_fajla_udesno(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let potez: Potez = Potez::new(E_FILE, 1, G_FILE, 1, Promocija::None);
        let tabla_nakon_poteza = tabla.tabla_nakon_validnog_poteza(&potez);
        let (_, file_topa) = crate::broj_to_rank_file(tabla_nakon_poteza.bele_figure[DESNI_TOP]);
        assert_eq!(F_FILE, file_topa);
    }

    #[test]
    fn odigraj_e4_Nc6_Bb5_Bc5_Bxc6_e6_a3_Ke7(){
        let tabla1: Tabla = Tabla::pocetna_pozicija();
        let potez_e4: Potez = Potez::new(E_FILE, 2, E_FILE, 4, Promocija::None);
        let tabla2: Tabla = tabla1.tabla_nakon_validnog_poteza(&potez_e4);

        assert_eq!(false, tabla2.beli_je_na_potezu());
        assert_eq!(E_FILE, tabla2.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu().unwrap());
        assert_eq!(0, tabla2.pre_koliko_poteza_je_50_move_rule_pomeren());

        let potez_Nc6 = Potez::new(B_FILE, 8, C_FILE, 6, Promocija::None);
        let tabla3: Tabla = tabla2.tabla_nakon_validnog_poteza(&potez_Nc6);
        assert_eq!(true, tabla3.beli_je_na_potezu());
        assert_eq!(None, tabla3.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu());
        assert_eq!(1, tabla3.pre_koliko_poteza_je_50_move_rule_pomeren());

        let potez_Bb5: Potez = Potez::new(F_FILE, 1, B_FILE, 5, Promocija::None);
        let tabla4: Tabla = tabla3.tabla_nakon_validnog_poteza(&potez_Bb5);
        assert_eq!(false, tabla4.beli_je_na_potezu());
        assert_eq!(None, tabla4.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu());
        assert_eq!(2, tabla4.pre_koliko_poteza_je_50_move_rule_pomeren());

        assert_eq!(28, tabla4.bele_figure[12]); /* e pijun je na e4. */
        assert_eq!(42, tabla4.crne_figure[LEVI_KONJ]); /* crni konj na c6 */
        assert_eq!(33, tabla4.bele_figure[DESNI_LOVAC]); /* beli lovac na b5 */
        

        let potez_Bc5: Potez = Potez::new(F_FILE, 8, C_FILE, 5, Promocija::None);
        let potez_Bxc6: Potez = Potez::new(B_FILE, 5, C_FILE, 6, Promocija::None);
        let potez_e6: Potez = Potez::new(E_FILE, 7, E_FILE, 6, Promocija::None);
        let potez_a3: Potez = Potez::new(A_FILE, 2, A_FILE, 3, Promocija::None);
        let potez_Ke7: Potez = Potez::new(E_FILE, 8, E_FILE, 7, Promocija::None);
        let tabla_kraj: Tabla = tabla4.tabla_nakon_validnog_poteza(&potez_Bc5).tabla_nakon_validnog_poteza(&potez_Bxc6)
        .tabla_nakon_validnog_poteza(&potez_e6).tabla_nakon_validnog_poteza(&potez_a3).tabla_nakon_validnog_poteza(&potez_Ke7);
 
        assert_eq!(1, tabla_kraj.pre_koliko_poteza_je_50_move_rule_pomeren());
        assert_eq!(true, tabla_kraj.beli_je_na_potezu());
        assert_eq!(true, Tabla::figura_je_pojedena(&tabla_kraj.crne_figure, LEVI_KONJ));
        assert_eq!(15, Tabla::broj_nepojedenih_figura(&tabla_kraj.crne_figure));
        assert_eq!(40, Tabla::ukupna_vrednost_nepojedenih_figura(&tabla_kraj.bele_figure));
        assert_eq!(37, Tabla::ukupna_vrednost_nepojedenih_figura(&tabla_kraj.crne_figure));
        assert_eq!(Figura::LOVAC.vrednost(), (Tabla::koja_figura_se_figura_nalazi_na_polju(&File_rank{file: C_FILE, rank:5}, &tabla_kraj.crne_figure)).unwrap().vrednost());
        assert_eq!(Figura::PIJUN.vrednost(), (Tabla::koja_figura_se_figura_nalazi_na_polju(&File_rank{file: E_FILE, rank: 4}, &tabla_kraj.bele_figure)).unwrap().vrednost());
    }

    fn potezi_ka_promociji_e4_d5_exd5_c6_dxc6_Nf6_cxb7_h6_bxa8Queen_Nc6() -> Tabla {
        let e4: Potez = Potez::new(E_FILE, 2, E_FILE, 4, Promocija::None);
        let d5: Potez = Potez::new(D_FILE, 7, D_FILE, 5, Promocija::None);
        let exd5: Potez = Potez::new(E_FILE, 4, D_FILE, 5, Promocija::None);
        let c6: Potez = Potez::new(C_FILE, 7, C_FILE, 6, Promocija::None);
        let dxc6: Potez = Potez::new(D_FILE, 5, C_FILE, 6, Promocija::None);
        let Nf6: Potez = Potez::new(G_FILE, 8, F_FILE, 6, Promocija::None);
        let cxb7: Potez = Potez::new(C_FILE, 6, B_FILE, 7, Promocija::None);
        let h6: Potez = Potez::new(A_FILE, 7, A_FILE, 6, Promocija::None);
        let bxa8Queen: Potez = Potez::new(B_FILE, 7, A_FILE, 8, Promocija::KRALJICA);
        let Nc6: Potez = Potez::new(B_FILE, 8, C_FILE, 6, Promocija::None);

        let tabla: Tabla = Tabla::pocetna_pozicija();
        tabla.tabla_nakon_validnog_poteza(&e4).tabla_nakon_validnog_poteza(&d5).
        tabla_nakon_validnog_poteza(&exd5).tabla_nakon_validnog_poteza(&c6)
        .tabla_nakon_validnog_poteza(&dxc6)
        .tabla_nakon_validnog_poteza(&Nf6)
        .tabla_nakon_validnog_poteza(&cxb7)
        .tabla_nakon_validnog_poteza(&h6)
        .tabla_nakon_validnog_poteza(&bxa8Queen)
        .tabla_nakon_validnog_poteza(&Nc6)
    }
    #[test]
    fn testiraj_promociju(){
       let tabla: Tabla = potezi_ka_promociji_e4_d5_exd5_c6_dxc6_Nf6_cxb7_h6_bxa8Queen_Nc6();
       assert_eq!(Figura::KRALJICA.vrednost(), Tabla::koja_figura_se_nalazi_u_bitu(&tabla.bele_figure, 12).unwrap().vrednost());
       assert_eq!(Figura::KRALJICA.vrednost(), Tabla::koja_figura_se_figura_nalazi_na_polju(&File_rank{file:A_FILE, rank:8}, &tabla.bele_figure).unwrap().vrednost());
    }

    #[test]
    fn testiraj_ukupna_vrednost_nepojedenih_figura(){
        let tabla: Tabla = potezi_ka_promociji_e4_d5_exd5_c6_dxc6_Nf6_cxb7_h6_bxa8Queen_Nc6();
        assert_eq!(48, Tabla::ukupna_vrednost_nepojedenih_figura(&tabla.bele_figure));
        assert_eq!(32, Tabla::ukupna_vrednost_nepojedenih_figura(&tabla.crne_figure));
    }
    
    #[test]
    fn testiraj_broj_nepojedenih_figura1(){
        let tabla: Tabla = potezi_ka_promociji_e4_d5_exd5_c6_dxc6_Nf6_cxb7_h6_bxa8Queen_Nc6();
        assert_eq!(16, Tabla::broj_nepojedenih_figura(&tabla.bele_figure));
        assert_eq!(12, Tabla::broj_nepojedenih_figura(&tabla.crne_figure));
    }

    #[test]
    fn testiraj_broj_nepojedenih_figura2(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let tabla2: Tabla = tabla.odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(F_FILE, 7, F_FILE, 5)
        .odigraj_validan_potez_bez_promocije(E_FILE, 4, F_FILE, 5);

        assert_eq!(15, Tabla::broj_nepojedenih_figura(&tabla2.crne_figure));
        assert_eq!(16, Tabla::broj_nepojedenih_figura(&tabla2.bele_figure));
    }

    #[test]
    fn testiraj_polje_je_prazno(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        assert_eq!(true, tabla.polje_je_prazno(&File_rank{file: G_FILE, rank: 4}));
        assert_eq!(false, tabla.polje_je_prazno(&File_rank{file: E_FILE, rank: 1}));
        assert_eq!(false, tabla.polje_je_prazno(&File_rank{file: H_FILE, rank: 7}));
        assert_eq!(false, tabla.polje_je_prazno(&File_rank{file: B_FILE, rank: 8}));
    }

    #[test]
    fn testiraj_pojedene_figure_kad_se_pomeri_kralj(){
        let tabla: Tabla = Tabla::pocetna_pozicija().
        odigraj_validan_potez_bez_promocije(A_FILE, 1, C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7,C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(H_FILE, 2, H_FILE, 3)
        .odigraj_validan_potez_bez_promocije(H_FILE, 7, G_FILE, 1)
        .odigraj_validan_potez_bez_promocije(E_FILE,1, E_FILE, 3);
         assert_eq!(true, Tabla::figura_je_pojedena(&tabla.bele_figure, LEVI_TOP));
         assert_eq!(true, Tabla::figura_je_pojedena(&tabla.bele_figure,DESNI_KONJ));
    }

    #[test]
    fn test_zapisi_info_za_rokadu(){
        let tabla: Tabla = Tabla::pocetna_pozicija().odigraj_validan_potez_bez_promocije(H_FILE, 1, H_FILE, 3); 
        assert_eq!(true, tabla.rokada().bela_kraljeva_rokada_vise_nije_moguca);

        let tabla2: Tabla = tabla.odigraj_validan_potez_bez_promocije(A_FILE, 8, A_FILE, 6);
        assert_eq!(true, tabla2.rokada().crna_kraljicina_rokada_vise_nije_moguca);

        let tabla3: Tabla = tabla2.odigraj_validan_potez_bez_promocije(A_FILE, 1, A_FILE, 3);
        assert_eq!(true, tabla3.rokada().bela_kraljicina_rokada_vise_nije_moguca);

        let tabla4: Tabla = tabla3.odigraj_validan_potez_bez_promocije(H_FILE, 8, H_FILE, 6);
        assert_eq!(true, tabla4.rokada().crna_kraljeva_rokada_vise_nije_moguca);
    }

    #[test]
    fn test2_zapisi_info_za_rokadu(){
        let mut potez_info: Potez_info = Potez_info::new();
        let potez: Potez_bits = Potez_bits{
            broj_figure: DESNI_TOP as u8,
            file: H_FILE,
            rank: 3,
            promocija: Promocija::None,
        };
        potez_info.zapisi_info_za_rokadu(&potez, &Tabla::pocetna_pozicija());
        assert_eq!(true, potez_info.rokada_onemogucena.bela_kraljeva_rokada_vise_nije_moguca);
        
    }

    #[test]
    fn testiraj_koja_figura_se_nalazi_u_polju(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let polje:u8 = crate::file_rank_to_broj(H_FILE, 1);
        Tabla::koja_figura_se_figura_nalazi_na_polju(&File_rank{file: H_FILE, rank:1}, &tabla.bele_figure);
    }
    #[test]
    fn test_rokada(){
        let tabla: Tabla = Tabla::pocetna_pozicija().odigraj_validan_potez_bez_promocije(F_FILE, 1, F_FILE, 3)
        .odigraj_validan_potez_bez_promocije(B_FILE, 8, B_FILE, 6)
        .odigraj_validan_potez_bez_promocije(G_FILE, 1, G_FILE, 3)
        .odigraj_validan_potez_bez_promocije(C_FILE, 8, C_FILE, 8)
  /*promocija */      .odigraj_validan_potez_bez_promocije(E_FILE, 1, G_FILE, 1)
        .odigraj_validan_potez_bez_promocije(D_FILE, 8, D_FILE, 6)
        .odigraj_validan_potez_bez_promocije(A_FILE, 2, A_FILE, 2)
   /* promocija */     .odigraj_validan_potez_bez_promocije(E_FILE, 8, C_FILE, 8);
        assert_eq!(crate::file_rank_to_broj(F_FILE, 1), tabla.bele_figure[DESNI_TOP]);
        assert_eq!(crate::file_rank_to_broj(D_FILE, 8), tabla.crne_figure[LEVI_TOP]);
        assert_eq!(true, tabla.rokada().nijedna_rokada_nije_moguca());
    }

    #[test]
    fn test_to_potez_bits(){
        let potez: Potez = Potez::new(H_FILE, 1, H_FILE, 3, Promocija::None);
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let potez_bits: Potez_bits = potez.to_Potez_bits(&tabla).unwrap();
        assert_eq!(DESNI_TOP as u8, potez_bits.broj_figure);
    }

    #[test]
    fn test_novi_potez(){
        let potez: Potez = Potez::new(H_FILE, 1, H_FILE, 3, Promocija::None);
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let potez_bits: Potez_bits = potez.to_Potez_bits(&tabla).unwrap();
        let tabla2: Tabla = tabla.tabla_nakon_poteza_bits(&potez_bits);

        assert_eq!(true, tabla2.rokada().bela_kraljeva_rokada_vise_nije_moguca);
    }

    #[test]
    fn test_pracenje_rokade(){
        let potez: Potez = Potez::new(H_FILE, 1, H_FILE, 3, Promocija::None);
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let tabla2: Tabla = tabla.tabla_nakon_validnog_poteza(&potez);
        assert_eq!(true, tabla2.rokada().bela_kraljeva_rokada_vise_nije_moguca);
        let tabla3: Tabla = tabla.odigraj_validan_potez_bez_promocije(H_FILE, 1, H_FILE, 8);
        assert_eq!(true, tabla3.rokada().bela_kraljeva_rokada_vise_nije_moguca);   
    }


    #[test]
    fn e_file_je_en_passant_file_nakon_d4_h5_d5_e5(){
         let tabla: Tabla = Tabla::pocetna_pozicija()
         .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 4)
         .odigraj_validan_potez_bez_promocije(H_FILE, 7, H_FILE, 5)
         .odigraj_validan_potez_bez_promocije(D_FILE, 4, D_FILE, 5)
         .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5);
         assert_eq!(E_FILE, tabla.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu().unwrap());
    }

    #[test]
    fn testiraj_f_pijun_je_pojeden_nakon_en_passant(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(A_FILE, 7, A_FILE, 7)
        .odigraj_validan_potez_bez_promocije(E_FILE, 4, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(F_FILE, 7, F_FILE, 5);
        let tabla: Tabla = tabla.odigraj_validan_potez_bez_promocije(E_FILE, 5, F_FILE, 6);
        
        assert_eq!(true, Tabla::figura_je_pojedena(&tabla.crne_figure, F_PIJUN));
    }

   
    #[test]
    fn testiraj_d4_d5_e4_c6_exd5_a5_dxc6_ima_30_legalnih_poteza_kraljica_ne_moze_da_jede_kraljicu(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 5)
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(C_FILE, 7, C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(E_FILE, 4, D_FILE, 5)
        .odigraj_validan_potez_bez_promocije(A_FILE, 7, A_FILE, 5)
        .odigraj_validan_potez_bez_promocije(D_FILE, 5, C_FILE, 6);

        let potezi: Vec<Potez_bits> = tabla.svi_legalni_potezi();
        assert_eq!(30, potezi.len());
        assert_eq!(false, potezi.contains(&Potez::new(D_FILE, 8, D_FILE, 1, Promocija::None).to_Potez_bits(&tabla).unwrap()));
    }

    #[test]
    fn posle_Nc3_d5_e4_d4_Na4_e5_Nf3_Bd6_c3_potez_Bd7_je_legalan(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(B_FILE, 1, C_FILE, 3)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 5)
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 5, D_FILE, 4)
        .odigraj_validan_potez_bez_promocije(C_FILE, 3, A_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(G_FILE, 1, F_FILE, 3)
        .odigraj_validan_potez_bez_promocije(F_FILE, 8, D_FILE, 6)
        .odigraj_validan_potez_bez_promocije(C_FILE, 2, C_FILE, 3);

        assert_eq!(true, Tabla::to_je_file_rank_polja(tabla.crne_figure[LEVI_LOVAC], 3, 8));
        let potezi: Vec<Potez_bits> = tabla.svi_legalni_potezi();
        assert_eq!(true, potezi.contains(&Potez_bits{broj_figure: LEVI_LOVAC as u8, file: 4, rank: 7, promocija: Promocija::None}));
        assert_eq!(true, potezi.contains(&Potez::new(C_FILE, 8, D_FILE, 7, Promocija::None).to_Potez_bits(&tabla).unwrap()));
        assert_eq!(potezi.len(), 38);
    }

    #[test]
    fn nakon_sto_se_pijun_pojede_pijun_prati_kralja(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(F_FILE, 7, F_FILE, 5)
        .odigraj_validan_potez_bez_promocije(E_FILE, 4, F_FILE, 5)
        .odigraj_validan_potez_bez_promocije(E_FILE, 8, F_FILE, 7);

        assert_eq!(true, Tabla::figura_je_pojedena(&tabla.crne_figure, F_PIJUN));
        assert_eq!(tabla.crne_figure[F_PIJUN], tabla.crne_figure[KRALJ]);

    }
}