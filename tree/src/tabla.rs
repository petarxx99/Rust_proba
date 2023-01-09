use std::{fs::File, env::VarError};



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

pub enum Figura {
    KRALJ=0, KRALJICA=1, TOP=2, LOVAC=3, KONJ=4, PIJUN=5
}

impl Figura {
    pub fn to_u8(&self) -> u8{
        match self {
            Figura::KRALJ => 0,
            Figura::KRALJICA => 1,
            Figura::TOP => 2,
            Figura::LOVAC => 3,
            Figura::KONJ => 4,
            Figura::PIJUN => 5
        }
    }

    pub fn from_u8(broj: u8) -> Figura {
        if broj == 0 {
            return Figura::KRALJ;
        }
        if broj ==1 {
            return Figura::KRALJICA;
        }
        if broj == 2 {
            return Figura::TOP;
        }
        if broj == 3 {
            return Figura::LOVAC;
        }
        if broj == 4 {
            return Figura::KONJ;
        }

        Figura::PIJUN
    }

    pub fn copy(&self) -> Figura {
        match *self {
            Self::KRALJICA => Self::KRALJICA,
            Self::KONJ => Self::KONJ,
            Self::TOP => Self::TOP,
            Self::LOVAC => Self::LOVAC,
            Self::KRALJ => Self::KRALJ,
            Self::PIJUN => Self::PIJUN,
        }
    }
}

fn map_to_figure(broj: usize) -> Option<Figura> {
    if broj == KRALJ {
        return Some(Figura::KRALJ);
    }
    if broj == KRALJICA {
        return Some(Figura::KRALJICA);
    }
    if broj == LEVI_TOP || broj == DESNI_TOP {
        return Some(Figura::TOP);
    }
    if broj == LEVI_LOVAC || broj == DESNI_LOVAC {
        return Some(Figura::LOVAC);
    }
    if broj == LEVI_KONJ || broj == DESNI_KONJ {
        return Some(Figura::KONJ);
    }

    None
}

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


pub struct Normalna_tabla {
    pub bele_figure: Vec<Option<Figura_info>>,
    pub crne_figure: Vec<Option<Figura_info>>,
    pub rokada_onemogucena: Rokada,
    pub beli_je_na_potezu: bool,
    pub fajl_pijuna_koji_se_pomerio_dva_polja_u_proslom_potezu: Option<u8>,
    pub pre_koliko_poteza_je_odlozen_50_move_rule: u8,
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

    /* Vraca None, ako se figura iz ovog bita ne nalazi na tabli, ako je figura pojedena. */
    fn koja_figura_se_nalazi_u_bitu(figure: &[u8; 16], broj_figure: usize) -> Option<Figura> {

/* Ako figura nije kralj, ali je na poziciji svog kralja, to znaci da je figura sklonjena sa table.
Na taj nacin skladistim informaciju da je figura sklonjena sa table, kako bih ustedeo memorijski prostor. */
        if broj_figure != KRALJ && figure[broj_figure] == figure[KRALJ]{
                return None;
        }

/* Ako je redni broj figure 8 ili vise, to znaci da je u pitanju pijun, ili figura koja je bila pijun. */
        if broj_figure >=8 {
            if ! Tabla::pijun_je_promovisan(figure[broj_figure]){
                return Some(Figura::PIJUN);
            } 

/* Sada obradjujem slucaj figure koja je bila pijun, ali je promovisana. */
            return Some(Tabla::u_sta_je_pijun_promovisan(&figure, broj_figure));
        }

        /* takozvani happy path */
        map_to_figure(broj_figure)
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
        let bela_kraljicina_rokada_vise_nije_moguca = self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        & (1 << 8) == 1;
        let bela_kraljeva_rokada_vise_nije_moguca = self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        & (1<<9) == 1;
        let crna_kraljicina_rokada_vise_nije_moguca = self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        & (1<<10) == 1;
        let crna_kraljeva_rokada_vise_nije_moguca = self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        & (1<<11) == 1; 

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
        bitfield |= 1 << 12;

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
            bitfield |= bit_broj_23
        }
        bitfield & (!bit_broj_23)
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
        
        niz[LEVI_TOP] = Tabla::file_rank_to_broj(A_FILE, rank_figura);
        niz[LEVI_KONJ] = Tabla::file_rank_to_broj(B_FILE, rank_figura);
        niz[LEVI_LOVAC] = Tabla::file_rank_to_broj(C_FILE, rank_figura);
        niz[KRALJICA] = Tabla::file_rank_to_broj(D_FILE, rank_figura); 
        niz[KRALJ] = Tabla::file_rank_to_broj(E_FILE, rank_figura);
        niz[DESNI_LOVAC] = Tabla::file_rank_to_broj(F_FILE, rank_figura);
        niz[DESNI_KONJ] = Tabla::file_rank_to_broj(G_FILE, rank_figura);
        niz[DESNI_TOP] = Tabla::file_rank_to_broj(H_FILE, rank_figura);

        for i in 8..16 as usize{
            let file: u8 = ((i-8) + A_FILE as usize) as u8;
            niz[i] = Tabla::file_rank_to_broj(file, rank_pijuna);
        }

        niz
    }


    fn pocetni_bit_field() -> i32 {
        0
    }

    fn file_rank_to_broj(file: u8, rank: u8) -> u8 {
        (rank-1) << 3 + file
    }

    fn polje_kralja(figure: &[u8;16]) -> u8 {
        let polje_kralja:u8 = figure[KRALJ];
        let prvih_6_bitova: u8 = (1<<6) - 1;
        polje_kralja & prvih_6_bitova
    }

    fn file_rank_kralja(figure: &[u8;16]) -> (u8,u8){
        Tabla::broj_to_rank_file(figure[0])
    }
    

    fn updejtuj_polozaj_figure(figure: &mut[u8;16], broj_figure: usize, file_rank: &File_rank) {
        let mut sacuvaj_sedmi_osmi_bit: u8 = figure[broj_figure] >> 6;
        sacuvaj_sedmi_osmi_bit <<= 6;

        figure[broj_figure] = Tabla::file_rank_to_broj(file_rank.file, file_rank.rank);
        figure[broj_figure] |= sacuvaj_sedmi_osmi_bit;
    }

    fn prati_polozaj_kralja(figure: &mut[u8;16], broj_figure: usize){

        let (rank, file) = Tabla::broj_to_rank_file(figure[broj_figure]);
        Tabla::updejtuj_polozaj_figure(figure, broj_figure, &File_rank{file, rank});
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
}

pub struct Potez{
    pub start_file: u8,
    pub start_rank: u8,

    pub file_destinacije: u8,
    pub rank_destinacije: u8,
    pub promocija: Promocija,
}

struct Potez_private {
    broj_figure: usize,
    file: u8,
    rank: u8,
    promocija: Promocija,
}

impl Potez {
    fn to_potez_private(&self, tabla: &Tabla) -> Option<Potez_private> {
        let mut figure: &[u8;16];
  /* Ako tabla cuva informaciju da je beli na potezu, to znaci da je crni odigrao potez i obrnuto. */      
        if tabla.beli_je_na_potezu() {
            figure = &tabla.crne_figure;
        } else {
            figure = &tabla.bele_figure;
        }

        
   /* Kralja za svaki slucaj treniram kao specijalni slucaj, jer figure koje su sklonjene sa table imaju istu lokaciju kao kralj.
   Ovaj deo koda je nepotreban, ali za slucaj da u buducnosti promenim redosled figura ovaj deo koda bi ucinio da takva promena ne proizvede bagove.*/     
        if Tabla::to_je_file_rank_polja(figure[KRALJ], self.start_file, self.start_rank){
            return Some(Potez_private{broj_figure: KRALJ, file: self.start_file, rank: self.start_rank, promocija: self.promocija.copy()})
        }

        for broj_figure in 0..figure.len() {
            if Tabla::to_je_file_rank_polja(figure[broj_figure], self.start_file, self.start_rank) {
                return Some(Potez_private { broj_figure, file: self.start_file, rank: self.start_rank, promocija: self.promocija.copy()})
            }
        }
        None
    }
}

impl Tabla {
    pub fn to_Normalna_tabla(&self) -> Normalna_tabla {
        let bele_figure: Vec<Option<Figura_info>> = Tabla::napravi_figure(&self.bele_figure, true);
        let crne_figure: Vec<Option<Figura_info>> = Tabla::napravi_figure(&self.crne_figure, false);
        let rokada_onemogucena: Rokada = self.rokada();
        let beli_je_na_potezu: bool = self.beli_je_na_potezu();
        let fajl_pijuna_koji_se_pomerio_dva_polja_u_proslom_potezu = self.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu();
        let pre_koliko_poteza_je_odlozen_50_move_rule = self.pre_koliko_poteza_je_50_move_rule_pomeren();

        Normalna_tabla {
            bele_figure,
            crne_figure,
            rokada_onemogucena,
            beli_je_na_potezu,
            fajl_pijuna_koji_se_pomerio_dva_polja_u_proslom_potezu,
            pre_koliko_poteza_je_odlozen_50_move_rule
        }
    }

    fn napravi_figure(figure: &[u8; 16], bela_boja: bool) -> Vec<Option<Figura_info>>{
        let mut nove_figure: Vec<Option<Figura_info>> = Vec::new();

        for i in 0..figure.len() {
            let figura: Option<Figura> =  Tabla::koja_figura_se_nalazi_u_bitu(figure, i);

            match figura  {
                Some(f) => {
                    let (rank, file) = Tabla::broj_to_rank_file(figure[i]);

                    if bela_boja{
                        nove_figure.push(Some(Figura_info::new(f, file, rank, Boja::BELA, true)));
                    } else {
                        nove_figure.push(Some(Figura_info::new(f, file, rank, Boja::CRNA, true)));
                    }
                },
                None => {nove_figure.push(None);}
            }
        }

        nove_figure
    }

   

    pub fn broj_to_rank_file(mut broj: u8) -> (u8, u8){
        let prvih_6_bitova: u8 = (1<<6) - 1;
        broj &= prvih_6_bitova;

        let rank = (broj>>3) + 1;
        let file = broj % 8;
        (rank, file)
    }

    pub fn polja_se_slazu(mut polje1: u8, mut polje2: u8) -> bool{
        let prvih_6_bitova: u8 = (1<<6) - 1;
        polje1 &= prvih_6_bitova;
        polje2 &= prvih_6_bitova;
        polje1 == polje2
    }

    pub fn to_je_file_rank_polja(mut polje: u8, file: u8, rank: u8) -> bool {
        let polje_2: u8 = Tabla::file_rank_to_broj(file, rank);
        Tabla::polja_se_slazu(polje, polje_2)
    }



    pub fn from_normalna_tabla(normalna_tabla: &Normalna_tabla) -> Tabla {

        let mut bele_figure: [u8; 16] = Tabla::napravi_figure_iz_normalne_table(&normalna_tabla.bele_figure);
        let mut crne_figure: [u8; 16] = Tabla::napravi_figure_iz_normalne_table(&normalna_tabla.crne_figure);
        let mut bitfield: i32 = Tabla::napravi_bitfield_iz_normalne_table(normalna_tabla);

        Tabla {
            bele_figure,
            crne_figure,
            sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu: bitfield,
        }
    }

    fn napravi_figure_iz_normalne_table(figure: &Vec<Option<Figura_info>>) -> [u8; 16]{
        let pozicija_kralja: u8 = Tabla::pozicija_kralja_iz_normalne_table(figure);
        let mut nove_figure: [u8; 16] = [pozicija_kralja; 16];
        Tabla::ubaci_figure_iz_normalne_table(figure, & mut nove_figure);       
        
        nove_figure
    }

    fn ubaci_figure_iz_normalne_table(figure: &Vec<Option<Figura_info>>,napravljene_figure: & mut [u8;16]){
       
        for i in 1..figure.len() {
            let figura_info: &Option<Figura_info> = &figure[i];
            match figura_info {
                Some(f) => {
                    napravljene_figure[i] = Tabla::file_rank_to_broj(f.file, f.rank);
                    if i>=8{
                        match f.tip {
                            Figura::KRALJICA => {  
                                Tabla::promovisi_pijuna(napravljene_figure, i, &Promocija::KRALJICA);
                            },
                            Figura::TOP => {
                                Tabla::promovisi_pijuna(napravljene_figure, i, &Promocija::TOP);
                            },
                            Figura::LOVAC => {
                                Tabla::promovisi_pijuna(napravljene_figure, i, &Promocija::LOVAC);
                            },
                            Figura::KONJ => {
                                Tabla::promovisi_pijuna(napravljene_figure, i, &Promocija::KONJ);
                            },
                            _ => {}
                        }
                    }
                },
                None => {}
            }
            
        }
    }


    
    fn pozicija_kralja_iz_normalne_table(figure: &Vec<Option<Figura_info>>) -> u8 {
        let mut pozicija_kralja: u8 = 0;
        for figura_optional in figure {
            match figura_optional {
                Some(figura) => {  
                    match figura.tip {
                        Figura::KRALJ => {pozicija_kralja = Tabla::file_rank_to_broj(figura.file, figura.rank);},
                         _ => {}
                    }
                },
                None => {}
            }
        }
        pozicija_kralja
    }

    fn napravi_bitfield_iz_normalne_table(normalna_tabla: &Normalna_tabla) -> i32 {
        let mut bitfield: i32 = 0;

        bitfield = Tabla::sifruj_pre_koliko_poteza_je_50_move_rule_pomeren(bitfield, normalna_tabla.pre_koliko_poteza_je_odlozen_50_move_rule as i32); 
        bitfield = Tabla::onemoguci_rokadu(bitfield, &normalna_tabla.rokada_onemogucena);

        if normalna_tabla.beli_je_na_potezu{
            bitfield = Tabla::sifruj_ko_je_na_potezu(bitfield, Ko_je_na_potezu::BELI);
        } else {
            bitfield = Tabla::sifruj_ko_je_na_potezu(bitfield, Ko_je_na_potezu::CRNI);
        }
        

        match normalna_tabla.fajl_pijuna_koji_se_pomerio_dva_polja_u_proslom_potezu {
            Some(fajl) => {bitfield = Tabla::dodaj_fajl_pijuna_koji_se_pomerio_2_polja(bitfield, fajl as i32);},
            None => {}
        }
        bitfield
    }
}



impl Tabla {
    pub fn tabla_nakon_validnog_poteza(&self, potez: &Potez) -> Tabla{
        let potez_private: Potez_private = potez.to_potez_private(self).expect("Uneli ste nevalidan potez.");
        let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_private(&potez_private);
        tabla_nakon_poteza
    }

    fn tabla_nakon_poteza_private(&self, potez: &Potez_private) -> Tabla{
        let mut bele_figure: [u8; 16] = self.bele_figure.clone();
        let mut crne_figure: [u8; 16] = self.crne_figure.clone();
        let mut bitfield: i32 = self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu;
       
        let mut potez_info: Potez_info = Potez_info::new();
        potez_info.beli_je_odigrao_potez = false;

        if self.beli_je_na_potezu() {
            potez_info.beli_je_odigrao_potez = true; /* self se odnosi na bivse stanje table */
        } 

        Tabla::updejtuj_figure_nakon_odigranog_poteza(& mut bele_figure, &mut crne_figure, potez, potez_info.beli_je_odigrao_potez, &mut potez_info);
        bitfield = Tabla::updejtuj_bitfield_nakon_odigranog_poteza(bitfield, &potez_info);
    
        Tabla {
            bele_figure,
            crne_figure,
            sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu: bitfield,
        }
    }

    fn updejtuj_bitfield_nakon_odigranog_poteza(mut bitfield: i32, potez_info: &Potez_info) -> i32{
        bitfield = Tabla::obrni_ko_je_na_potezu(bitfield);
        bitfield = Tabla::onemoguci_rokadu(bitfield, &potez_info.rokada_onemogucena);
        if potez_info.pijun_pomeren_ili_figura_pojedena {
            bitfield = Tabla::sifruj_pre_koliko_poteza_je_50_move_rule_pomeren(bitfield, 0);
        }
        bitfield = Tabla::resetuj_fajl_pijuna_koji_se_pomerio_dva_polja(bitfield);
        match potez_info.file_pijuna_pomerenog_2_polja {
            None => {},
            Some(file) => {
                bitfield = Tabla::dodaj_fajl_pijuna_koji_se_pomerio_2_polja(bitfield, file as i32);
            }
        }

        bitfield
    }

    fn updejtuj_figure_nakon_odigranog_poteza(bele_figure: &mut [u8; 16], crne_figure: &mut [u8;16], potez: &Potez_private, beli_je_odigrao_potez: bool, potez_info: &mut Potez_info){
        
        if beli_je_odigrao_potez {
            Tabla::updejtuj_figure_koje_su_odigrale_potez(bele_figure, potez, potez_info);
            Tabla::updejtuj_figure_protiv_kojih_je_odigran_potez(crne_figure, potez, potez_info);
        } else {
            Tabla::updejtuj_figure_koje_su_odigrale_potez(crne_figure, potez, potez_info);
            Tabla::updejtuj_figure_protiv_kojih_je_odigran_potez(bele_figure, potez, potez_info);
        }
    }

/* Posebno obradjujem slucaj kad se pomera kralj, jer kralj ima istu lokaciju kao figure koje
su sklonjene sa table. */
    fn updejtuj_figure_koje_su_odigrale_potez(figure: & mut[u8; 16], potez: &Potez_private, potez_info: &mut Potez_info){
        if potez.broj_figure == KRALJ {
            potez_info.rokada_onemogucena.pomeren_kralj(potez_info.beli_je_odigrao_potez);
            Tabla::pomeri_kralja(figure, potez.file, potez.rank);
            return;
        }

        if Tabla::proveri_da_li_je_pomeren_pijun(figure, potez.broj_figure) {
            potez_info.pijun_pomeren_ili_figura_pojedena = true;
        }
        Tabla::updejtuj_polozaj_figure(figure, potez.broj_figure,
             &File_rank::new(potez.file, potez.rank));

        match &potez.promocija {
                Promocija::None => {},
                _promocija => {
                     Tabla::promovisi_pijuna(figure, potez.broj_figure, _promocija);
                }
        }       
    } 

    fn updejtuj_figure_protiv_kojih_je_odigran_potez(figure: &mut[u8;16], potez: &Potez_private, potez_info: &mut Potez_info){
        let polje_destinacije: u8 = Tabla::file_rank_to_broj(potez.file, potez.rank);
        for i in 0..figure.len() {
            if Tabla::polja_se_slazu(polje_destinacije, figure[i]){
                potez_info.pijun_pomeren_ili_figura_pojedena = true;
                Tabla::prati_polozaj_kralja(figure, i);
                return;
            }
        }
    }

    fn pomeri_kralja(figure: & mut[u8;16], file: u8, rank: u8){
        let polozaj_kralja: u8 = figure[KRALJ];

        Tabla::updejtuj_polozaj_figure(figure, KRALJ,
             &File_rank{file, rank});

        for i in 0..figure.len() {
            if Tabla::polja_se_slazu(polozaj_kralja, figure[i]){
                Tabla::prati_polozaj_kralja(figure, i);
            }
        }

    }
    
}