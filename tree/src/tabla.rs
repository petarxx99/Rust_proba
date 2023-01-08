
pub struct Rokada {
    bela_kraljicina_rokada_vise_nije_moguca: bool,
    bela_kraljeva_rokada_vise_nije_moguca: bool,
    
    crna_kraljicina_rokada_vise_nije_moguca: bool,
    crna_kraljeva_rokada_vise_nije_moguca: bool,
}

pub struct Tabla  {
    bele_figure: [u8; 16],
    crne_figure: [u8; 16],
    sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu: i32,
}


impl Tabla {

    /* sopstvena_evaluacija se cuva u najmanje vrednom bajtu ove gigantske promenljive*/
    pub fn sopstvena_evaluacija(&self) -> i8 {
        let sopstvena_evaluacija = self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu
        &
        ((1 << 8) -1); /* 0b11111111 */

        sopstvena_evaluacija as i8
    }

    /* bitovi za rokadu se nalaze u donja 4 bita drugog bajta (tj. od 9. to 12. bita).
    Ako je bit 1, rokada je onemogucena, ako je bit 2, rokada je omogucena. */
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


    /* Gornja 4 bita drugog bajta cuvaju ovu informaciju. 13.bit (5. bit drugog bajta) cuva informaciju
     da li je pijun uopste pomeren dva polja u prethodnom potezu.*/
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

    /* Ova informacija se cuva u prvih 6 bitova 3. bajta (tj. od 17. do 22. bita) */
    pub fn pre_koliko_poteza_je_50_move_rule_pomeren(&self) -> u8 {
        let prvih_6_bitova = (1<<6) -1;

        let mut pre_koliko_poteza = self.sopstvena_evaluacija_2rokada_en_passant_3pre_koliko_poteza_je_pijun_pojeden_4ko_je_na_potezu 
        >> 16;
        pre_koliko_poteza &= prvih_6_bitova;
        pre_koliko_poteza as u8
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


 