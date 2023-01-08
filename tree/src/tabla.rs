pub enum Ko_je_na_potezu{
    BELI = 0, CRNI = 1
}


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
    Ako je bit 1, rokada je onemogucena, ako je bit 0, rokada je omogucena. */
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

        if bit_broj_23 == 0 {
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
            Ko_je_na_potezu::BELI => {bit_za_enkodovanje = 0;},
            Ko_je_na_potezu::CRNI => {bit_za_enkodovanje = 1;}
        }

        /* Broj koji se nalazio u bit_za_enkodovanje ce biti u 23. bitu bitfielda. */
        bit_za_enkodovanje <<= 22;
        bitfield | bit_za_enkodovanje
    }

}

impl Tabla {
    pub fn pocetna_pozicija() -> Tabla {
        Tabla {
            bele_figure: Tabla::pocetna_pozicija_belih_figura(),
            crne_figure: Tabla::pocetna_pozicija_crnih_figura(),
            sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu:
            Tabla::pocetni_bit_field()
        }
    }

    /* Prvih 8 bajtova cuvaju informacije o tome gde se figure nalaze. 
Prvih 5 bajtova cuvaju informaciju o tome gde se nalaze na tabli, 6. bajt cuva informaciju o tome
da li se uopste nalaze na tabli. 7. i 8. bajt su na raspolaganju pijunu koji je 8 mesta ispred njih
 u nizu. Tih 8 bajtova cuvaju informaciju o tome u sta se pijun pretvorio (da li je postao kraljica,
top, lovac, konj, itd.). Ukoliko je pijun i dalje pijun, onda su ova poslednja dva bajta nebitna.
 Sto se tice pijuna, oni se nalaze od 8 do 15 mesta u nizu. Oni koriste prvih 6 bajtova za poziciju na tabli,
 7. bajt odredjuje da li su promovisani, ili ne, a 8. bajt ostaje neiskoriscen. */

    fn pocetna_pozicija_belih_figura() -> [u8; 16] {
        let mut niz = [0 as u8; 16];
        niz
    }

    fn pocetna_pozicija_crnih_figura() -> [u8; 16] {
        let mut niz = [0 as u8; 16];
        return niz;
    }

    fn pocetni_bit_field() -> i32 {
        0
    }
}


 