use std::{fs::File, env::VarError};

use self::kretanje_figura::Figura_interfejs;

pub(crate) mod normalna_tabla;
pub(crate) mod potez;
pub(crate) mod kretanje_figura;

pub static BELI: u8 = 0;
pub static CRNI: u8 = 1;
pub static KRALJ: usize = 0;
pub static KRALJICA: usize = 1;
pub static LEVI_TOP: usize = 2;
pub static DESNI_TOP: usize = 3;
pub static LEVI_LOVAC: usize = 4;
pub static DESNI_LOVAC: usize = 5;
pub static LEVI_KONJ: usize = 6;
pub static DESNI_KONJ: usize = 7;

pub static PROMOVISANA_KRALJICA: u8 = 1;
pub static PROMOVISAN_TOP: u8 = 2;
pub static PROMOVISAN_LOVAC: u8 = 3;
pub static PROMOVISAN_KONJ: u8 = 0;

pub static A_FILE: u8 = 0;
pub static B_FILE: u8 = 1;
pub static C_FILE: u8 = 2;
pub static D_FILE: u8 = 3;
pub static E_FILE: u8 = 4;
pub static F_FILE: u8 = 5;
pub static G_FILE: u8 = 6;
pub static H_FILE: u8 = 7;

pub enum Figura {
    KRALJ=0, KRALJICA=1, TOP=2, LOVAC=3, KONJ=4, PIJUN=5
}


pub enum Ko_je_na_potezu{
    BELI = 0, CRNI = 1
}

pub enum Boja {
    BELA = 0, CRNA = 1
}

pub enum Promocija {
    KRALJICA, TOP, LOVAC, KONJ, None,
}
impl Promocija {
    pub fn copy(&self) -> Promocija {
        match (*self){
            Self::KRALJICA => Self::KRALJICA,
            Self::KONJ => Self::KONJ,
            Self::TOP => Self::TOP,
            Self::LOVAC => Self::LOVAC,
            Self::None => Self::None
        }
    }
}
pub struct File_rank{
    pub file: u8,
    pub rank: u8,
}

impl File_rank{
    pub fn new(file: u8, rank: u8) -> File_rank {
        File_rank{
            file,
            rank
        }
    }
}

pub struct Rokada {
    pub bela_kraljicina_rokada_vise_nije_moguca: bool,
    pub bela_kraljeva_rokada_vise_nije_moguca: bool,
    
    pub crna_kraljicina_rokada_vise_nije_moguca: bool,
    pub crna_kraljeva_rokada_vise_nije_moguca: bool,
}

impl Rokada {
    pub fn pomeren_kralj(&mut self, beli_je_odigrao: bool){
        if beli_je_odigrao {
            self.bela_kraljeva_rokada_vise_nije_moguca = true;
            self.bela_kraljicina_rokada_vise_nije_moguca = true;
        }  else {
            self.crna_kraljeva_rokada_vise_nije_moguca = true;
            self.crna_kraljicina_rokada_vise_nije_moguca = true;
        }  
    }

    pub fn new_sve_rokade_moguce() -> Rokada{
        Rokada { bela_kraljicina_rokada_vise_nije_moguca: false, bela_kraljeva_rokada_vise_nije_moguca: false, crna_kraljicina_rokada_vise_nije_moguca: false, crna_kraljeva_rokada_vise_nije_moguca: false }
    }

    pub fn nijedna_rokada_ove_boje_nije_moguca(&self, beli: bool) -> bool {
        if beli {
            return self.bela_kraljeva_rokada_vise_nije_moguca && self.bela_kraljicina_rokada_vise_nije_moguca;
        } else {
            return self.crna_kraljeva_rokada_vise_nije_moguca && self.crna_kraljicina_rokada_vise_nije_moguca;
        } 
    }
    pub fn nijedna_rokada_nije_moguca(&self)->bool{
        self.bela_kraljeva_rokada_vise_nije_moguca
        && self.bela_kraljicina_rokada_vise_nije_moguca
        && self.crna_kraljeva_rokada_vise_nije_moguca
        && self.crna_kraljicina_rokada_vise_nije_moguca
    }
}



pub struct Figura_info {
    pub tip: Figura,
    pub file: u8,
    pub rank: u8,
    pub boja: Boja,
    pub nije_pojedena: bool,
}

impl Figura_info{
    pub fn new(tip: Figura, file: u8, rank: u8, boja: Boja, nije_pojedena: bool) -> Figura_info {
        Figura_info {
            tip,
            file,
            rank,
            boja,
            nije_pojedena
        }

    }
}
 



/*https://doc.rust-lang.org/std/mem/fn.size_of.html */
#[repr(C)]
pub struct Tabla  {
    bele_figure: [u8; 16],
    crne_figure: [u8; 16],
    sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu: i32,
}

pub fn print_size_of_Tabla(){
    println!("{}", std::mem::size_of::<Tabla>());
}


pub trait Ima_podatke_o_tabli{
    fn da_li_su_polja_prazna(&self, polja: &[u8]) -> bool;
    fn pozicija_kralja(&self, kralj_je_bele_boje: bool) -> u8;
    fn polja_nisu_napadnuta(&self, polja: &Vec<u8>, bele_ne_crne_figure_napadaju: bool) -> bool;
    fn da_li_je_figura_boje_na_polju(&self, figura_je_bele_boje: bool, rank: u8, file: u8) -> bool;
    fn get_rokada(&self) -> Rokada;
    fn get_file_pijuna_koji_se_pomerio_2_polja(&self) -> Option<u8>;
    fn get_beli_je_na_potezu(&self) -> bool;
}

impl Ima_podatke_o_tabli for Tabla {

    fn get_beli_je_na_potezu(&self) -> bool {
        self.beli_je_na_potezu()
    }

    fn get_rokada(&self) -> Rokada {
        self.rokada()
    }

    fn get_file_pijuna_koji_se_pomerio_2_polja(&self) -> Option<u8> {
        self.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu()
    }

    fn da_li_su_polja_prazna(&self, polja: &[u8]) -> bool {
        for polje in polja {
            if !self.polje_je_prazno_preko_broja(*polje){
                return false
            }
        }
        true
    }

    fn da_li_je_figura_boje_na_polju(&self, figura_je_bele_boje: bool, rank: u8, file: u8) -> bool{
        let mut figure: &[u8;16];
        if figura_je_bele_boje {
            figure = &self.bele_figure;
        } else {
            figure = &self.crne_figure;
        }

        let target_polje: u8 = crate::file_rank_to_broj(file, rank);
        for figura in figure {
            if Tabla::polja_se_slazu(*figura, target_polje) {
                return true
            }
        }
        false
    }


    fn pozicija_kralja(&self, kralj_je_bele_boje: bool) -> u8 {
        if !kralj_je_bele_boje {
            self.crne_figure[KRALJ]
        } else {
            self.bele_figure[KRALJ]
        }
    }

   fn polja_nisu_napadnuta(&self, polja: &Vec<u8>, bele_ne_crne_figure_napadaju: bool) -> bool {

        if bele_ne_crne_figure_napadaju {
            for i in 0..self.bele_figure.len(){
                match Figura::iz_niza_u_figure_interfejs(&self.bele_figure, i) {
                    None => {},
                    Some(figura) => {
                        for polje in polja {
                            if (figura.napada_polje)(self, *polje, self.bele_figure[i], bele_ne_crne_figure_napadaju){
                                return false;
                            }
                        }
                    }
                }
            }
        } else {
            for i in 0..self.crne_figure.len(){
                match Figura::iz_niza_u_figure_interfejs(&self.crne_figure, i) {
                    None => {},
                    Some(figura) => {
                        for polje in polja {
                            if (figura.napada_polje)(self, *polje, self.crne_figure[i], bele_ne_crne_figure_napadaju){
                                return false;
                            }
                        }
                    }
                }
            }
        }

        
        return true;
    }
}
    /* Prvih 8 bajtova cuvaju informacije o tome gde se figure nalaze. 
Prvih 6 bajtova cuvaju informaciju o tome gde se nalaze na tabli. 
Informaciju o tome da li se figura nalazi na tabli cuvam tako sto figure koje su sklonjene sa table
imaju istu lokaciju kao i njihov kralj.
7. i 8. bajt ostaju na raspolaganju pijunu koji se nalazi 8 ispred u nizu. 
7. i 8. bajt cuvaju informaciju o tome u sta se pijun pretvorio (da li je postao kraljica,
top, lovac, konj, itd.). Ako je pijun i dalje pijun, onda 7. i 8. bajt ne sluze ni cemu.
 Sto se tice pijuna, oni se nalaze od 8 do 15 mesta u nizu. Oni koriste prvih 6 bajtova za poziciju na tabli,
 7. bajt odredjuje da li su promovisani, ili ne, a 8. bajt ostaje neiskoriscen. */

 impl Tabla {
    fn obradi_bit_pijuna_za_promociju(pijunov_bit: u8) -> u8 {
        let sedmi_bit: u8 = 1 << 6;
        pijunov_bit | sedmi_bit
    }

    fn promovisi_pijuna(figure: &mut[u8; 16], redni_broj_pijuna: usize, promocija: &Promocija){
 /* Ovako bit koji je predstavljao pijuna zna da nije vise pijun. */       
        figure[redni_broj_pijuna] = Tabla::obradi_bit_pijuna_za_promociju(figure[redni_broj_pijuna]);

        let mut promocija_bitovi: u8 = 0;
        match *promocija {
            Promocija::KRALJICA => {promocija_bitovi = PROMOVISANA_KRALJICA;},
            Promocija::TOP => {promocija_bitovi = PROMOVISAN_TOP;},
            Promocija::LOVAC => {promocija_bitovi = PROMOVISAN_LOVAC;},
            Promocija::KONJ => {promocija_bitovi = PROMOVISAN_KONJ;}
            Promocija::None => {panic!("Aktivirana funkcija za promociju pijuna, iako pijun nije promovisan.")}
        }
/* Bitovi promocije se cuvaju u dva najznacajnija bita figure koja se nalazi 8 mesta iza promovisanog pijuna. */
        promocija_bitovi <<= 6;
        let redni_broj_figure_koja_ce_da_cuva_podatke_o_promociji = redni_broj_pijuna - 8;

        /* Prvo da ocistim dva najznacajnija bita za svaki slucaj. */
        let sedmi_osmi_bit: u8 = 0b11 << 6;
        figure[redni_broj_figure_koja_ce_da_cuva_podatke_o_promociji] &= !sedmi_osmi_bit;

        figure[redni_broj_figure_koja_ce_da_cuva_podatke_o_promociji] |= promocija_bitovi;
    } 

    fn pijun_je_promovisan(pijunov_bit: u8) -> bool {
        let sedmi_bit: u8 = 1 << 6;
        (pijunov_bit & sedmi_bit) != 0
    }

    fn u_sta_je_pijun_promovisan(figure: &[u8; 16], redni_broj_pijuna: usize) -> Figura {
/* Figura koja je 8 mesta iza pijuna cuva informaciju o tome sta je pijun postao posle promocije. */
        let mut bit: u8 = figure[redni_broj_pijuna - 8];
        bit >>= 6;
    
        if bit == PROMOVISAN_TOP {
            return Figura::TOP;
        }
        if bit == PROMOVISAN_LOVAC {
            return Figura::LOVAC;
        }
        if bit == PROMOVISAN_KONJ {
            return Figura::KONJ;
        }

        return Figura::KRALJICA;
    }

    fn proveri_da_li_je_pomeren_pijun(figure: &[u8;16], redni_broj_figure: usize) -> bool{
        /* Ako je redni broj manji od 8 znaci da nije pomeren pijun. Ako je redni broj 8 ili vise,
        ali je pijun promovisan, onda nije pomeren pijun, nego kraljica, top, lovac ili konj u koga je pijun
        bio promovisan. */
        if redni_broj_figure < 8 {
            return false;
        }
        let pijunov_bit: u8 = figure[redni_broj_figure];
        !Tabla::pijun_je_promovisan(pijunov_bit)
    }

  
 }

impl Tabla {

    /* sopstvena_evaluacija se cuva u najmanje vrednom bajtu ove gigantske promenljive*/
    pub fn get_sopstvena_evaluacija(&self) -> i8 {
        let sopstvena_evaluacija: i32 = self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        &
        ((1 << 8) -1); /* 0b11111111 */

        sopstvena_evaluacija as i8
    }

    fn set_sopstvena_evaluacija(&mut self, evaluacija: i8) {
        /* Brisem prvi bajt. */
        let prvi_bajt: i32 = (1<<8) - 1; /* 0b11111111 */
        self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        &= !prvi_bajt; 

        /* Stavljam da prvi bajt bude isti kao i argument ove procedure. */
        self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        |= evaluacija as i32;
    }

    /* bitovi za rokadu se nalaze u donja 4 bita drugog bajta (tj. od 9. to 12. bita).
    Ako je bit 1, rokada je onemogucena, ako je bit 0, rokada je omogucena. 
    9. bit cuva informaciju o tome da li je bela kraljicina rokada moguca.
    10. bit cuva informaciju o tome da li je bela kraljeva rokada moguca.
    11. bit cuva informaciju o tome da li je crna kraljicina rokada moguca.
    12. bit cuva informaciju o tome da li je crna kraljeva rokada moguca.*/
    pub fn rokada(&self) -> Rokada {
        let bela_kraljicina_rokada_vise_nije_moguca = (self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        & (1 << 8)) != 0;
        let bela_kraljeva_rokada_vise_nije_moguca = (self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        & (1<<9)) != 0;
        let crna_kraljicina_rokada_vise_nije_moguca = (self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        & (1<<10)) != 0;
        let crna_kraljeva_rokada_vise_nije_moguca = (self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        & (1<<11)) != 0; 

        return Rokada {
            bela_kraljicina_rokada_vise_nije_moguca,
            bela_kraljeva_rokada_vise_nije_moguca,
            crna_kraljicina_rokada_vise_nije_moguca,
            crna_kraljeva_rokada_vise_nije_moguca
        };
    }

    fn onemoguci_belu_kraljicinu_rokadu(bitfield: i32) -> i32{
        bitfield | (1<<8)
    }

    fn onemoguci_belu_kraljevu_rokadu(bitfield: i32) -> i32{
        bitfield | (1<<9)
    }

    fn onemoguci_crnu_kraljicinu_rokadu(bitfield: i32) -> i32 {
        bitfield | (1<<10)
    }

    fn onemoguci_crnu_kraljevu_rokadu(bitfield: i32) -> i32 {
        bitfield | (1<<11)
    }

    fn onemoguci_rokadu(mut bitfield: i32, rokada: &Rokada) -> i32{
        if rokada.bela_kraljeva_rokada_vise_nije_moguca {
            bitfield = Tabla::onemoguci_belu_kraljevu_rokadu(bitfield);
        }
        if rokada.bela_kraljicina_rokada_vise_nije_moguca {
            bitfield = Tabla::onemoguci_belu_kraljicinu_rokadu(bitfield);
        }
        if rokada.crna_kraljeva_rokada_vise_nije_moguca {
            bitfield = Tabla::onemoguci_crnu_kraljevu_rokadu(bitfield);
        }
        if rokada.crna_kraljicina_rokada_vise_nije_moguca {
            bitfield = Tabla::onemoguci_crnu_kraljicinu_rokadu(bitfield);
        }
        bitfield
    }
    /* Gornja 4 bita drugog bajta cuvaju ovu informaciju. 13.bit (5. bit drugog bajta) cuva informaciju
     da li je pijun uopste pomeren dva polja u prethodnom potezu. Ako je 13. bit 0, onda pijun nije 
     pomeren 2 polja u prethodnom potezu. Fajl pijuna koji se pomerio 2 polja se cuva u
     14. 15. i 16. bitu.*/
    pub fn fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu(&self) -> Option<u8> {
        let trinaesti_bit =  self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        & (1 << 12);

        if trinaesti_bit == 0 {
            return None;
        }

        let mut fajl_pijuna = self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        >> 13;
        let prva_tri_bita = (1<<3) - 1;
        fajl_pijuna &= prva_tri_bita;

        Some(fajl_pijuna as u8)
    }

    fn dodaj_fajl_pijuna_koji_se_pomerio_2_polja(mut bitfield: i32, mut fajl: i32) -> i32 {
        /* Stavljam trinaesti bit na 1, to znaci da se pijun pomerio 2 polja u proslom potezu */
        bitfield |= (1 << 12);

        /* Brisem bitove 14, 15 i 16. */
        let tri_bita: i32 = 0b111 << 13;
        bitfield &= !tri_bita;

        /* Pomeram da bitovi 14, 15 i 16 cuvaju vrednost fajla, svi ostali bitovi su 0. */
        fajl = fajl << 13;

        /* Stavljam da se jedinice iz fajla upisu u bitfield. */
        bitfield |= fajl;

        bitfield
    }


    fn resetuj_fajl_pijuna_koji_se_pomerio_dva_polja(bitfield: i32) -> i32 {
        /* Bitovi 13, 14, 15 i 16 treba da budu 0. */
        let bitovi_za_reset: i32 = 0b1111 << 12;
        bitfield & (!bitovi_za_reset)
    }


    /* Ova informacija se cuva u prvih 6 bitova 3. bajta (tj. od 17. do 22. bita) */
    pub fn pre_koliko_poteza_je_50_move_rule_pomeren(&self) -> u8 {
        let prvih_6_bitova = (1<<6) -1;

        let mut pre_koliko_poteza = self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu 
        >> 16;
        pre_koliko_poteza &= prvih_6_bitova;
        pre_koliko_poteza as u8
    }

    fn sifruj_pre_koliko_poteza_je_50_move_rule_pomeren(mut bitfield: i32, mut pre_koliko_poteza: i32) -> i32 {
        /* Brisem prvih 6 bitova 3. bajta. */
        let prvih_sest_bitova: i32 = (1<<6) -1; /* 0b111111 */
        let prvih_sest_bitova_treceg_bajta = prvih_sest_bitova << 16; 
        bitfield &= !prvih_sest_bitova_treceg_bajta;

        /* Pomeram da bitni bitovi pre_koliko_poteza budu od 17. do 22. bita, a ne od 1. do 6. 
        Svi ostali bitovi su 0.*/
        pre_koliko_poteza <<= 16;

        /* Stavljam da bitovi od 17. do 22. bitfielda imaju vrednost pre koliko poteza je 50 move rule pomeren. */
        bitfield | pre_koliko_poteza
    }

/* unsafe zato sto ne proverava da li je broj presao 50. Ovde ima 6 bitova, znaci ako predje broj 63
ova funkcija ce poceti da menja memoriju koja joj ne pripada. */
    fn povecaj_50_move_rule_brojac_za_1_unsafe(mut bitfield: i32) -> i32 {
        bitfield + (1 << 16)
    }

    /* Ova informacija se cuva u bitu broj 23, tj. u 7. bitu treceg bajta. */
    pub fn beli_je_na_potezu(&self) -> bool {
        let mut bit_broj_23 = self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        &
        (1<<22);
        bit_broj_23 >>= 22;

        if bit_broj_23 == BELI as i32{
            true
        } else {
            false
        }
    }

    fn sifruj_ko_je_na_potezu(mut bitfield: i32, ko_je_na_potezu: Ko_je_na_potezu) -> i32 {
        /* Obrisi bit za enkodovanje ko je na potezu (to je 23. bit).
        Obrisi bit 23. */
        let bit_broj_23: i32 = 1 << 22;
        bitfield &= !bit_broj_23; 

        let mut bit_za_enkodovanje: i32 = 0;
        match ko_je_na_potezu {
            Ko_je_na_potezu::BELI => {bit_za_enkodovanje = BELI as i32;},
            Ko_je_na_potezu::CRNI => {bit_za_enkodovanje = CRNI as i32;}
        }

        /* Broj koji se nalazio u bit_za_enkodovanje ce biti u 23. bitu bitfielda.
        23. bit bitfielda cuva informaciju o tome ko je na potezu. */
        bit_za_enkodovanje <<= 22;
        bitfield | bit_za_enkodovanje
    }

/* Ko je na potezu se cuva u bitu broj 23. */
    fn obrni_ko_je_na_potezu(mut bitfield: i32) -> i32 {
        let bit_broj_23: i32 = 1 << 22;

        if (bitfield & bit_broj_23) == 0 {
            bitfield | bit_broj_23
        } else {
            bitfield & (!bit_broj_23)
        }
    }

/* unsafe je zato sto mora uvek da se pazi da li se pomera kralj, jer kad se pomera kralj, figure
koje su pojedene moraju da prate novu poziciju kralja, jer na taj nacin cuvam informaciju da su
pojedene. */
    fn updejtuj_polozaj_figure_unsafe(figure: &mut[u8;16], broj_figure: usize, file_rank: &File_rank) {
        let mut sacuvaj_sedmi_osmi_bit: u8 = figure[broj_figure] >> 6;
        sacuvaj_sedmi_osmi_bit <<= 6;

        figure[broj_figure] = crate::file_rank_to_broj(file_rank.file, file_rank.rank);
        figure[broj_figure] |= sacuvaj_sedmi_osmi_bit;
    }



}

impl Tabla{
   

  

    pub fn polja_se_slazu(mut polje1: u8, mut polje2: u8) -> bool{
        let prvih_6_bitova: u8 = (1<<6) - 1;
        polje1 &= prvih_6_bitova;
        polje2 &= prvih_6_bitova;
        polje1 == polje2
    }

    pub fn to_je_file_rank_polja(mut polje: u8, file: u8, rank: u8) -> bool {
        let polje_2: u8 = crate::file_rank_to_broj(file, rank);
        Tabla::polja_se_slazu(polje, polje_2)
    }  
    
}


impl Tabla {
    pub fn pocetna_pozicija() -> Tabla {
        Tabla {
            bele_figure: Tabla::pocetna_pozicija_figura(true),
            crne_figure: Tabla::pocetna_pozicija_figura(false),
            sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu:
            Tabla::pocetni_bit_field()
        }
    }


    fn pocetna_pozicija_figura(ovo_su_bele_figure: bool) -> [u8; 16] {
        let mut rank_figura = 8;
        let mut rank_pijuna = 7;
        if ovo_su_bele_figure {
            rank_figura = 1;
            rank_pijuna = 2;
        } 
        let mut niz = [0 as u8; 16];
        
        niz[LEVI_TOP] = crate::file_rank_to_broj(A_FILE, rank_figura);
        niz[LEVI_KONJ] = crate::file_rank_to_broj(B_FILE, rank_figura);
        niz[LEVI_LOVAC] = crate::file_rank_to_broj(C_FILE, rank_figura);
        niz[KRALJICA] = crate::file_rank_to_broj(D_FILE, rank_figura); 
        niz[KRALJ] = crate::file_rank_to_broj(E_FILE, rank_figura);
        niz[DESNI_LOVAC] = crate::file_rank_to_broj(F_FILE, rank_figura);
        niz[DESNI_KONJ] = crate::file_rank_to_broj(G_FILE, rank_figura);
        niz[DESNI_TOP] = crate::file_rank_to_broj(H_FILE, rank_figura);

        for i in 8..16 as usize{
            let file: u8 = ((i-8) + A_FILE as usize) as u8;
            niz[i] = crate::file_rank_to_broj(file, rank_pijuna);
        }

        niz
    }


    fn pocetni_bit_field() -> i32 {
        0
    }

  

    fn polje_kralja(figure: &[u8;16]) -> u8 {
        let polje_kralja:u8 = figure[KRALJ];
        let prvih_6_bitova: u8 = (1<<6) - 1;
        polje_kralja & prvih_6_bitova
    }

    fn file_rank_kralja(figure: &[u8;16]) -> (u8,u8){
        crate::broj_to_rank_file(figure[0])
    }
    
    pub fn resetuj_50_move_rule(bitfield: i32) -> i32 {
        Tabla::sifruj_pre_koliko_poteza_je_50_move_rule_pomeren(bitfield, 0)
    }

    pub fn figure_koje_su_na_potezu(&self) -> &[u8;16]{
        if self.beli_je_na_potezu() {
            &self.bele_figure
        } else {
            &self.crne_figure
        }
    }

}



#[cfg(test)]
mod tabla_tests{
    use crate::tabla::KRALJICA;

    use super::{Tabla, E_FILE, Rokada};


    #[test]
    fn kraljice_su_dobro_inicijalizovane(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let bit_bele_kraljice: u8 = 3;
        let bit_crne_kraljice: u8 = 59;
      
        assert_eq!(bit_bele_kraljice, tabla.bele_figure[KRALJICA]);
        assert_eq!(bit_crne_kraljice, tabla.crne_figure[KRALJICA]);
    }

    #[test]
    fn drugi_pijun_dobro_inicijalizovan(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let bit_drugog_belog_pijuna: u8 = 9;
        let bit_drugog_crnog_pijuna: u8 = 49;

        assert_eq!(bit_drugog_belog_pijuna, tabla.bele_figure[9]);
        assert_eq!(bit_drugog_crnog_pijuna, tabla.crne_figure[9]);
    }

    #[test]
    fn promena_ko_je_na_potezu(){
        let mut tabla: Tabla = Tabla::pocetna_pozicija();
        let mut beli_je_na_potezu = tabla.beli_je_na_potezu();
        assert_eq!(true, beli_je_na_potezu);

        let bitfield: i32 = Tabla::obrni_ko_je_na_potezu(tabla.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu);
        tabla.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu = bitfield;
        beli_je_na_potezu = tabla.beli_je_na_potezu();
        assert_eq!(false, beli_je_na_potezu);
    }

    #[test]
    fn fajl_pijuna_koji_se_pomerio_2_polja(){
        let mut tabla: Tabla = Tabla::pocetna_pozicija();
        tabla.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu =  Tabla::dodaj_fajl_pijuna_koji_se_pomerio_2_polja(tabla.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu,  E_FILE as i32);
        assert_eq!(E_FILE, tabla.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu().unwrap());
    }

    #[test]
    fn test_onemoguci_rokadu(){
        let nula: i32 = 0;
        let rokada: Rokada = Rokada{
            bela_kraljeva_rokada_vise_nije_moguca: false,
            bela_kraljicina_rokada_vise_nije_moguca: true,
            crna_kraljicina_rokada_vise_nije_moguca: false,
            crna_kraljeva_rokada_vise_nije_moguca: true
        };
        let bitfield = Tabla::onemoguci_rokadu(nula, &rokada);
        assert_ne!(bitfield, 0);
        let mut tabla = Tabla::pocetna_pozicija();
        tabla.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu = bitfield;
        let rokada2: Rokada = tabla.rokada();
        assert_eq!(true, rokada2.bela_kraljicina_rokada_vise_nije_moguca);
        assert_eq!(true, rokada2.crna_kraljeva_rokada_vise_nije_moguca);
        assert_eq!(false, rokada2.bela_kraljeva_rokada_vise_nije_moguca);
        assert_eq!(false, rokada2.crna_kraljicina_rokada_vise_nije_moguca);
    }


}

    
