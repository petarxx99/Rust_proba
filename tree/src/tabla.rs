use std::fs::File;



pub enum Ko_je_na_potezu{
    BELI = 0, CRNI = 1
}

pub enum Boja {
    BELA = 0, CRNA = 1
}

pub enum Promocija {
    KRALJICA, TOP, LOVAC, KONJ
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

    fn promovisi_pijuna(figure: &mut[u8; 16], redni_broj_pijuna: usize, promocija: Promocija){
 /* Ovako bit koji je predstavljao pijuna zna da nije vise pijun. */       
        figure[redni_broj_pijuna] = Tabla::obradi_bit_pijuna_za_promociju(figure[redni_broj_pijuna]);

        let mut promocija_bitovi: u8 = 0;
        match promocija {
            Promocija::KRALJICA => {promocija_bitovi = PROMOVISANA_KRALJICA;},
            Promocija::TOP => {promocija_bitovi = PROMOVISAN_TOP;},
            Promocija::LOVAC => {promocija_bitovi = PROMOVISAN_LOVAC;},
            Promocija::KONJ => {promocija_bitovi = PROMOVISAN_KONJ;}
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

}

pub struct Potez{
    pub start_file: u8,
    pub start_rank: u8,

    pub file_destinacije: u8,
    pub rank_destinacije: u8,
    pub promocija: Option<Promocija>,
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
            let (rank, file) = Tabla::broj_to_rank_file(figure[i]);

            match figura  {
                Some(f) => {
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

   

    pub fn broj_to_rank_file(broj: u8) -> (u8, u8){
        let rank = (broj>>3) + 1;
        let file = broj % 8;
        (rank, file)
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
                                Tabla::promovisi_pijuna(napravljene_figure, i, Promocija::KRALJICA);
                            },
                            Figura::TOP => {
                                Tabla::promovisi_pijuna(napravljene_figure, i, Promocija::TOP);
                            },
                            Figura::LOVAC => {
                                Tabla::promovisi_pijuna(napravljene_figure, i, Promocija::LOVAC);
                            },
                            Figura::KONJ => {
                                Tabla::promovisi_pijuna(napravljene_figure, i, Promocija::KONJ);
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
