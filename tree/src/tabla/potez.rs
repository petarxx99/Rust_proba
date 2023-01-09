use super::{Rokada, Promocija, Tabla, KRALJ, File_rank};




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
     /* Potez se kombinuje sa tablom (tj. pozicijom) nad kojom se potez igra, tako da je boja igraca
     koji odigrao potez ista ona koju ima tabla nad kojom potez treba da se odigra. */
        if tabla.beli_je_na_potezu() {
            figure = &tabla.bele_figure;
        } else {
            figure = &tabla.crne_figure;
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
            bitfield = Tabla::resetuj_50_move_rule(bitfield);
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


    fn prati_polozaj_kralja(figure: &mut[u8;16], broj_figure: usize){

        let (rank, file) = Tabla::broj_to_rank_file(figure[broj_figure]);
        Tabla::updejtuj_polozaj_figure(figure, broj_figure, &File_rank{file, rank});
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