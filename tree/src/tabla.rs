use std::fs::File;

pub enum Ko_je_na_potezu{
    BELI = 0, CRNI = 1
}

pub enum Promocija {
    KRALJICA, TOP, LOVAC, KONJ
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
    bela_kraljicina_rokada_vise_nije_moguca: bool,
    bela_kraljeva_rokada_vise_nije_moguca: bool,
    
    crna_kraljicina_rokada_vise_nije_moguca: bool,
    crna_kraljeva_rokada_vise_nije_moguca: bool,
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

impl Tabla {

    /* sopstvena_evaluacija se cuva u najmanje vrednom bajtu ove gigantske promenljive*/
    pub fn sopstvena_evaluacija(&self) -> i8 {
        let sopstvena_evaluacija = self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
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
        bitfield | ((1<<8) -1)
    }

    fn onemoguci_belu_kraljevu_rokadu(bitfield: i32) -> i32{
        bitfield | ((1<<9) -1)
    }

    fn onemoguci_crnu_kraljicinu_rokadu(bitfield: i32) -> i32 {
        bitfield | ((1<<10) -1)
    }

    fn onemoguci_crnu_kraljevu_rokadu(bitfield: i32) -> i32 {
        bitfield | ((1<<11) -1)
    }

    /* Gornja 4 bita drugog bajta cuvaju ovu informaciju. 13.bit (5. bit drugog bajta) cuva informaciju
     da li je pijun uopste pomeren dva polja u prethodnom potezu. Ako je 13. bit 0, onda pijun nije 
     pomeren 2 polja u prethodnom potezu.*/
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
        bitfield & !bitovi_za_reset
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
        let bit_broj_23 = self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        &
        (1<<22);

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

    /* Prvih 8 bajtova cuvaju informacije o tome gde se figure nalaze. 
Prvih 6 bajtova cuvaju informaciju o tome gde se nalaze na tabli. Informaciju o tome
da li se figura nalazi na tabli cuvam tako sto figure koje su sklonjene sa table imaju istu lokaciju
kao i njihov kralj koji je iste boje.
7. i 8. bajt ostaju na raspolaganju pijunu koji se nalazi 8 ispred u nizu. 
7. i 8. bajt cuvaju informaciju o tome u sta se pijun pretvorio (da li je postao kraljica,
top, lovac, konj, itd.). Ako je pijun i dalje pijun, onda 7. i 8. bajt ne sluze ni cemu.
 Sto se tice pijuna, oni se nalaze od 8 do 15 mesta u nizu. Oni koriste prvih 6 bajtova za poziciju na tabli,
 7. bajt odredjuje da li su promovisani, ili ne, a 8. bajt ostaje neiskoriscen. */

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

    fn broj_to_file_rank(broj: u8) -> File_rank {
        let rank = (broj >> 3) + 1;
        let file = broj % 8;
        File_rank {
            file,
            rank
        }
    }
}

pub struct File_rank{
    file: u8,
    rank: u8
}
 
pub struct Potez{
    start: File_rank,
    destinacija: File_rank,
    promocija: Option<Promocija>,
}

