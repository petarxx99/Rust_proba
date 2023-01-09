
use crate::tabla::Figura_info;
use crate::tabla::Rokada;
use crate::tabla::Tabla;
use crate::tabla::Figura;


use crate::tabla::Boja;

use crate::tabla::Ko_je_na_potezu;

use crate::tabla::Promocija;


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