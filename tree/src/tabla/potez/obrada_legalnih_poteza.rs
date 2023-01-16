use std::collections::HashSet;

use crate::tabla::{Tabla, KRALJ, self, File_rank, nekompresirana_tabla::{self, Nekompresirana_tabla}};

use super::Potez_bits;

pub enum Partija_zavrsena{
    PARTIJA_TRAJE = 0, PAT=1, MAT=2, FIFTY_MOVE_RULE = 3
}


impl Tabla {
    pub fn potez_je_sah(&self, potez: &Potez_bits) -> bool {
        let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(potez);
        tabla_nakon_poteza.igrac_je_u_sahu(&tabla_nakon_poteza.to_nekompresirana_tabla())
    }

    pub fn igrac_je_u_sahu(&self, nekompresirana_tabla: &Nekompresirana_tabla) -> bool {
        let polje_mog_kralja: File_rank = File_rank::new_iz_broja(self.figure_koje_su_na_potezu()[KRALJ]);
        let protivnicke_figure: &[u8;16] = self.figure_koje_nisu_na_potezu();
        let protivnik_je_beli: bool = !self.beli_je_na_potezu();

        self.figure_napadaju_polje(
            &polje_mog_kralja,
            protivnicke_figure,
            protivnik_je_beli,
            nekompresirana_tabla
        )
    }


    pub fn potez_uzima_figuru(&self, potez: &Potez_bits) -> bool {
        !self.polje_je_prazno(&File_rank{file: potez.file, rank: potez.rank})
    }


    pub fn dostupan_je_mat_u_jednom_potezu(&self) -> bool {
        match self.postoji_potez_posle_kojeg_je_partija_zavrsena(){
            Partija_zavrsena::MAT => true,
            _ => false
        }
    }

    pub fn postoji_potez_posle_kojeg_je_partija_zavrsena(&self) -> Partija_zavrsena {
        let potezi: Vec<Potez_bits> = self.svi_legalni_potezi();
        for potez in potezi {
            let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(&potez);
            match tabla_nakon_poteza.partija_je_zavrsena(&tabla_nakon_poteza.to_nekompresirana_tabla()){
                Partija_zavrsena::PARTIJA_TRAJE => {},
                partija_zavrsena => {return partija_zavrsena;} 
            }
        }
        Partija_zavrsena::PARTIJA_TRAJE
    }

    pub fn protivnik_preti_mat_u_jednom_potezu(&self) -> bool {
        let tabla_da_je_protivnik_na_potezu: Tabla = self.suprotna_boja_je_na_potezu();
        match tabla_da_je_protivnik_na_potezu.postoji_potez_posle_kojeg_je_partija_zavrsena(){
            Partija_zavrsena::MAT => true,
            _ => false
        }
    }

    pub fn partija_je_zavrsena(&self, nekompresirana_tabla: &Nekompresirana_tabla) -> Partija_zavrsena {
        if self.pre_koliko_poteza_je_50_move_rule_pomeren() >= 50 {
            return Partija_zavrsena::FIFTY_MOVE_RULE
        }

        if self.nema_legalnih_poteza(nekompresirana_tabla) {
            if self.igrac_je_u_sahu(nekompresirana_tabla){
                return Partija_zavrsena::MAT
            } else {
                return Partija_zavrsena::PAT
            }
        }
        Partija_zavrsena::PARTIJA_TRAJE
    }

    pub fn koje_moje_figure_su_napadnute(&self) -> Vec<u8> {
        let moje_figure: &[u8;16] = self.figure_koje_su_na_potezu();
        let protivnikove_figure: &[u8;16] = self.figure_koje_nisu_na_potezu();
        let protivnik_je_beli: bool = self.beli_je_na_potezu();
        let mut napadnute_figure: Vec<u8> = Vec::new();
        let nekompresirana_tabla: Nekompresirana_tabla = self.to_nekompresirana_tabla();

        for i in 0..16{
            if Tabla::figura_je_pojedena(moje_figure, i) {
                continue;
            }
            if self.figure_napadaju_polje(&File_rank::new_iz_broja(moje_figure[i]), protivnikove_figure, protivnik_je_beli, &nekompresirana_tabla){
                napadnute_figure.push(i as u8);
            }
        }

        napadnute_figure
    }
  
    pub fn moje_napadnute_figure(&self) -> HashSet<u8>{
        let moje_figure: &[u8;16] = self.figure_koje_su_na_potezu();
        let protivnikove_figure: &[u8;16] = self.figure_koje_nisu_na_potezu();
        let protivnik_je_beli: bool = self.beli_je_na_potezu();
        let mut napadnute_figure: HashSet<u8> = HashSet::new();
        let nekompresirana_tabla: Nekompresirana_tabla = self.to_nekompresirana_tabla();

        for i in 0..16{
            if Tabla::figura_je_pojedena(moje_figure, i) {
                continue;
            }
            if self.figure_napadaju_polje(&File_rank::new_iz_broja(moje_figure[i]), protivnikove_figure, protivnik_je_beli, &nekompresirana_tabla){
                napadnute_figure.insert(i as u8);
            }
        }

        napadnute_figure
    }
}

#[cfg(test)]
mod test_obrada_legalnih_poteza{
    use crate::tabla::{Tabla, potez::Potez, F_FILE, B_FILE, D_FILE, E_FILE, Promocija, H_FILE, G_FILE, C_FILE};


    #[test]
    fn test_posle_e4_d5_Bb5_je_sah(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 5);

        let potez_sah: Potez = Potez::new(F_FILE, 1, B_FILE, 5, Promocija::None);
        assert_eq!(true, tabla.potez_je_sah(&potez_sah.to_Potez_bits(&tabla).unwrap()));
    }

    #[test]
    fn test_posle_e4_e5_f4_Qh4_je_sah(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(F_FILE, 2, F_FILE, 4);

        let potez_sah: Potez = Potez::new(D_FILE, 8, H_FILE, 4, Promocija::None);
        assert_eq!(true, tabla.potez_je_sah(&potez_sah.to_Potez_bits(&tabla).unwrap()));

    }

    #[test]
    fn test_posle_e4_e5_d4_Qh4_nije_sah(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 4);

        let potez_sah: Potez = Potez::new(D_FILE, 8, H_FILE, 4, Promocija::None);
        assert_eq!(false, tabla.potez_je_sah(&potez_sah.to_Potez_bits(&tabla).unwrap()));

    }

    #[test]
    fn test_e4_nije_sah(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let potez: Potez = Potez::new(E_FILE, 2, E_FILE, 4, Promocija::None);
        assert_eq!(false, tabla.potez_je_sah(&potez.to_Potez_bits(&tabla).unwrap()));
    }

    /* Testovi da li je figura pojedena. */
    #[test]
    fn posle_e4_d5_exd5_uzima_figuru(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 5);

        let potez: Potez = Potez::new(E_FILE, 4, D_FILE, 5, Promocija::None);
        assert_eq!(true, tabla.potez_uzima_figuru(&potez.to_Potez_bits(&tabla).unwrap()));
    }

    #[test]
    fn posle_e4_e5_Nf3_Nc6_potez_Nxe5_uzima_figuru(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(G_FILE, 1, F_FILE, 3)
        .odigraj_validan_potez_bez_promocije(B_FILE, 8, C_FILE, 6);

        let potez: Potez = Potez::new(F_FILE, 3, E_FILE, 5, Promocija::None);
        assert_eq!(true, tabla.potez_uzima_figuru(&potez.to_Potez_bits(&tabla).unwrap()));
    }

    /* Testovi za da li potez preti mat u jednom potezu. */
    #[test]
    fn posle_e4_e5_Qh5_Nc6_Bc4_Nf6_imam_mat_u_jednom_potezu(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(D_FILE, 1, H_FILE, 5)
        .odigraj_validan_potez_bez_promocije(B_FILE, 8, C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4)
        .odigraj_validan_potez_bez_promocije(G_FILE, 8, F_FILE, 6);

        assert_eq!(true, tabla.dostupan_je_mat_u_jednom_potezu());
    }

    #[test]
    fn posle_e4_e5_Qh5_Nc6_Bc4_Nf6_Qf7_je_kraj(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(D_FILE, 1, H_FILE, 5)
        .odigraj_validan_potez_bez_promocije(B_FILE, 8, C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4)
        .odigraj_validan_potez_bez_promocije(G_FILE, 8, F_FILE, 6)
        .odigraj_validan_potez_bez_promocije(H_FILE, 5, F_FILE, 7);

        assert_eq!(0, tabla.svi_legalni_potezi().len());
        assert_eq!(true, tabla.nema_legalnih_poteza(&tabla.to_nekompresirana_tabla()));
    }

    #[test]
    fn koje_moje_figure_su_napadnute(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 5)
        .odigraj_validan_potez_bez_promocije(G_FILE, 1, F_FILE, 3)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4)
        .odigraj_validan_potez_bez_promocije(C_FILE, 8, G_FILE, 4)
        .odigraj_validan_potez_bez_promocije(H_FILE, 2, H_FILE, 3)
        .odigraj_validan_potez_bez_promocije(B_FILE, 7, B_FILE, 5);
        /* Lovac, e pijun, konj i a pijun su napadnuti. */
        assert_eq!(4, tabla.koje_moje_figure_su_napadnute().len());
    }


    #[test]
    fn posle_e4_e5_Qf4_Nc6_Bc4_protivnik_preti_mat_u_jednom_potezu(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(D_FILE, 1, F_FILE, 3)
        .odigraj_validan_potez_bez_promocije(B_FILE, 8, C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4);

        assert_eq!(true, tabla.protivnik_preti_mat_u_jednom_potezu());
    }

    #[test]
    fn test_moje_napadnute_figure(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 5)
        .odigraj_validan_potez_bez_promocije(G_FILE, 1, F_FILE, 3)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4)
        .odigraj_validan_potez_bez_promocije(C_FILE, 8, G_FILE, 4)
        .odigraj_validan_potez_bez_promocije(H_FILE, 2, H_FILE, 3)
        .odigraj_validan_potez_bez_promocije(B_FILE, 7, B_FILE, 5);
        /* Lovac, e pijun, konj i a pijun su napadnuti. */
        assert_eq!(4, tabla.moje_napadnute_figure().len());
    }
}