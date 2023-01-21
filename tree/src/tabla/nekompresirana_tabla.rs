use super::{Ima_podatke_o_tabli, File_rank, Rokada, KRALJ, Tabla, H_FILE, A_FILE};

static PRAZNO_POLJE: u8 = 0;
static BELA_FIGURA: u8 = 1;
static CRNA_FIGURA: u8 = 2;

pub struct Nekompresirana_tabla{
    pub polja_table: [[u8;9]; 9],
    pub pozicija_belog_kralja: File_rank,
    pub pozicija_crnog_kralja: File_rank,
    pub rokada: Rokada,
    pub file_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu: Option<u8>,
    pub beli_je_na_potezu: bool,
    tabla: Tabla,
}



impl Ima_podatke_o_tabli for Nekompresirana_tabla{

    fn pozicija_kralja(&self, kralj_je_bele_boje: bool) -> File_rank {
        if kralj_je_bele_boje {
            return self.pozicija_belog_kralja.copy()
        } 
        self.pozicija_crnog_kralja.copy()
    }

    fn polja_nisu_napadnuta(&self, polja: &Vec<File_rank>, bele_ne_crne_figure_napadaju: bool) -> bool {
        self.tabla.polja_nisu_napadnuta(polja, bele_ne_crne_figure_napadaju)
    }

    fn da_li_su_polja_prazna(&self, polja: &[File_rank]) -> bool {
        for polje in polja {
            if self.polja_table[polje.rank as usize][polje.file as usize] != PRAZNO_POLJE {
                return false
            }
        }
        true
    }

    fn da_li_je_polje_prazno(&self, polje: &File_rank) -> bool {
        self.polja_table[polje.rank as usize][polje.file as usize] == PRAZNO_POLJE
    }

    fn da_li_je_figura_boje_na_polju(&self, figura_je_bele_boje: bool, rank: u8, file: u8) -> bool {
        if file_rank_je_nelegalan(&File_rank{file, rank}){
            return false
        }
        if figura_je_bele_boje {
            if self.polja_table[rank as usize][file as usize] == BELA_FIGURA {
                return true
            } else {
                return false
            }
        } else {
            if self.polja_table[rank as usize][file as usize] == CRNA_FIGURA {
                return true
            } else {
                return false
            }
        }
    }

    fn get_rokada(&self) -> Rokada {
        self.rokada.copy()
    }

    fn get_file_pijuna_koji_se_pomerio_2_polja(&self) -> Option<u8> {
        match &self.file_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu{
            &None => None,
            &Some(_file) => Some(_file)
        }
    }

    fn get_beli_je_na_potezu(&self) -> bool {
        self.beli_je_na_potezu
    }
}

fn file_rank_je_nelegalan(polje: &File_rank) -> bool{
    if polje.file < A_FILE || polje.file > H_FILE {
        return true
    }
    if polje.rank < 1 || polje.rank > 8 {
        return true
    }
    false
}

impl Tabla{
    pub fn to_nekompresirana_tabla(&self) -> Nekompresirana_tabla{
        let bele_figure: &[u8;16] = &self.bele_figure;
        let crne_figure: &[u8;16] = &self.crne_figure;
        let pozicija_belog_kralja = File_rank::new_iz_broja(bele_figure[KRALJ]);
        let pozicija_crnog_kralja: File_rank = File_rank::new_iz_broja(crne_figure[KRALJ]);

        let file_za_en_passant = match self.get_file_pijuna_koji_se_pomerio_2_polja(){
            None => None,
            Some(_file) => Some(_file)
        };

        Nekompresirana_tabla { 
            polja_table: self.polja_table(),
            pozicija_belog_kralja,
            pozicija_crnog_kralja,
            rokada: self.rokada(),
            file_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu: file_za_en_passant,
            beli_je_na_potezu: self.beli_je_na_potezu(), 
            tabla: self.copy() }
    }

    fn polja_table(&self) -> [[u8;9]; 9] {
        let mut polja_table: [[u8;9];9] = [[PRAZNO_POLJE; 9]; 9];

        for i in 0..16 {
            if !Tabla::figura_je_pojedena(&self.bele_figure, i){
                let (rank, file) = crate::broj_to_rank_file(self.bele_figure[i]);
                polja_table[rank as usize][file as usize] = BELA_FIGURA;
            } 

            if !Tabla::figura_je_pojedena(&self.crne_figure, i) {
                let (rank, file) = crate::broj_to_rank_file(self.crne_figure[i]);
                polja_table[rank as usize][file as usize] = CRNA_FIGURA;
            }
        }
        polja_table
    }
}



pub struct Tabla_pijuna{
    tabla: [[u8;9];9],
}

impl Tabla_pijuna{
    pub fn pijun_bele_boje(&self, rank: u8, file: u8) -> bool {
        self.tabla[rank as usize][file as usize] == BELA_FIGURA
    }

    pub fn pijun_crne_boje(&self, rank: u8, file: u8) -> bool {
        self.tabla[rank as usize][file as usize] == CRNA_FIGURA
    }
}

impl Tabla{
    pub fn to_tabla_pijuna(&self) -> Tabla_pijuna{
        let mut tabla_pijuna = Tabla_pijuna{tabla: [[PRAZNO_POLJE;9];9]};
        for i in 8..16 {
            if !Tabla::figura_je_pojedena(&self.bele_figure, i) {
                if !Tabla::pijun_je_promovisan(self.bele_figure[i]){
                    let (rank,file) :(u8,u8) = crate::broj_to_rank_file(self.bele_figure[i]);
                    tabla_pijuna.tabla[rank as usize][file as usize] = BELA_FIGURA;
                }
            }

            if !Tabla::figura_je_pojedena(&self.crne_figure, i) {
                if !Tabla::pijun_je_promovisan(self.crne_figure[i]){
                    let (rank,file): (u8,u8) = crate::broj_to_rank_file(self.crne_figure[i]);
                    tabla_pijuna.tabla[rank as usize][file as usize] = CRNA_FIGURA;
                }
            }
        }

        tabla_pijuna
    }
}

#[cfg(test)]
mod nekompresovana_tabla_test{
    use crate::tabla::{Tabla, File_rank, C_FILE, A_FILE, B_FILE, F_FILE, Ima_podatke_o_tabli};

    use super::Nekompresirana_tabla;
    



    #[test]
    fn test_tabla_to_nekompresovana_tabla(){
        let tabla_nk: Box<dyn Ima_podatke_o_tabli> = Box::from(Tabla::pocetna_pozicija().to_nekompresirana_tabla());
        assert_eq!(true, tabla_nk.da_li_je_figura_boje_na_polju(true, 2, F_FILE));
        assert_eq!(true, tabla_nk.da_li_je_figura_boje_na_polju(true, 1, B_FILE));
        assert_eq!(true, tabla_nk.da_li_je_figura_boje_na_polju(false, 8, C_FILE));
        assert_eq!(true, tabla_nk.da_li_je_figura_boje_na_polju(false, 7, A_FILE));

        assert_eq!(false, tabla_nk.da_li_je_figura_boje_na_polju(true, 8, C_FILE));
        assert_eq!(false, tabla_nk.da_li_je_figura_boje_na_polju(true, 5, C_FILE));
        let t: Nekompresirana_tabla = Tabla::pocetna_pozicija().to_nekompresirana_tabla();
        for i in 1..9 {
            println!("rank: {}\n", i);
            for j in 1..9 {
                println!("  file: {}, figura: {}", j, t.polja_table[i][j]);
            }
        }
    }

    #[test]
    fn test_polje_je_prazno(){
        let tabla_nk: Box<dyn Ima_podatke_o_tabli> = Box::from(Tabla::pocetna_pozicija().to_nekompresirana_tabla());
        assert_eq!(false, tabla_nk.da_li_je_polje_prazno(&File_rank::new(B_FILE, 2)));
        assert_eq!(false, tabla_nk.da_li_je_polje_prazno(&File_rank::new(C_FILE, 7)));
        assert_eq!(true, tabla_nk.da_li_je_polje_prazno(&File_rank::new(A_FILE, 5)));
        assert_eq!(true, tabla_nk.da_li_je_polje_prazno(&File_rank::new(F_FILE, 3)));
        assert_eq!(false, tabla_nk.da_li_je_polje_prazno(&File_rank::new(A_FILE, 8)));
    }
}