
use crate::tabla::Figura_info;
use crate::tabla::Rokada;
use crate::tabla::Tabla;
use crate::tabla::Figura;


use crate::tabla::Boja;

use crate::tabla::Ko_je_na_potezu;

use crate::tabla::Promocija;

use super::KRALJ;


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

            match figura  {
                Some(f) => {
                    let (rank, file) = crate::broj_to_rank_file(figure[i]);

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
                    napravljene_figure[i] = crate::file_rank_to_broj(f.file, f.rank);
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
                        Figura::KRALJ => {pozicija_kralja = crate::file_rank_to_broj(figura.file, figura.rank);},
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


      /* Vraca None, ako se figura iz ovog bita ne nalazi na tabli, ako je figura pojedena. */
      pub fn koja_figura_se_nalazi_u_bitu(figure: &[u8; 16], broj_figure: usize) -> Option<Figura> {

        /* Ako figura nije kralj, ali je na poziciji svog kralja, to znaci da je figura sklonjena sa table.
        Na taj nacin skladistim informaciju da je figura sklonjena sa table, kako bih ustedeo memorijski prostor. */
                if broj_figure != KRALJ && Tabla::polja_se_slazu(figure[broj_figure], figure[KRALJ]){
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
                Figura::map_redni_broj_to_figure_unsafe(broj_figure)
        }
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

    /* Unsafe zato sto ne uzima u obzir da li je pijun postao kraljica. */
pub fn map_redni_broj_to_figure_unsafe(redni_broj: usize) -> Option<Figura> {
    match redni_broj {
        0 => Some(Figura::KRALJ),
        1 => Some(Figura::KRALJICA),
        2 => Some(Figura::TOP),
        3 => Some(Figura::TOP),
        4 => Some(Figura::LOVAC),
        5 => Some(Figura::LOVAC),
        6 => Some(Figura::KONJ),
        7 => Some(Figura::KONJ),
        broj => {
            if broj < 16 {
                return Some(Figura::PIJUN)
            }
            return None
         }
    }
}


  
}


mod test_normalna_tabla{
    use crate::tabla::{Tabla, KRALJICA};

    use super::Figura;

    #[test]
    fn testiraj_koja_figura_se_nalazi_u_bitu(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let pijun: Figura = Tabla::koja_figura_se_nalazi_u_bitu(&tabla.bele_figure, 9).unwrap();
        let kraljica: Figura = Tabla::koja_figura_se_nalazi_u_bitu(&tabla.crne_figure, KRALJICA).unwrap();

        assert_eq!(pijun.vrednost(), Figura::PIJUN.vrednost());
        assert_eq!(kraljica.vrednost(), Figura::KRALJICA.vrednost());
    }
}
