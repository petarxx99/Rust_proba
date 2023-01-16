use crate::tabla::{Ima_podatke_o_tabli, Tabla, kretanje_figura::Figura_interfejs, Figura, KRALJ, Promocija, File_rank, nekompresirana_tabla::{Nekompresirana_tabla, self}};

use super::{Potez_bits, Potez_polje};


impl Tabla {

    /* Ova metoda ocekuje da joj se preda potez takav da figura moze 
    doci do polja destinacije. Podrazumeva se da je taj uslov ispunjen. */
    pub fn potez_je_legalan_podrazumeva_se_da_figura_moze_doci_na_polje(&self, potez: &Potez_polje, nekompresirana_tabla: &Nekompresirana_tabla) -> bool {
        
        /* Ne mogu da pojedem sopstvenu figuru. */
        if nekompresirana_tabla.da_li_je_figura_boje_na_polju(self.beli_je_na_potezu(), potez.end_rank, potez.end_file) {
            return false
        }

        self.nisam_u_sahu_nakon_poteza(potez)
    }

  
 
    pub fn nisam_u_sahu_nakon_poteza(&self, potez: &Potez_polje) -> bool{
            let tabla_nakon_odigranog_poteza: Tabla = self.odigraj_potez_bez_promocije_unsafe(potez);
            let polje_mog_kralja: File_rank = File_rank::new_iz_broja(tabla_nakon_odigranog_poteza.figure_koje_nisu_na_potezu()[KRALJ]);
            let protivnikove_figure: &[u8;16] = tabla_nakon_odigranog_poteza.figure_koje_su_na_potezu();
            let protivnik_ima_bele_figure: bool = tabla_nakon_odigranog_poteza.beli_je_na_potezu();
    
            !tabla_nakon_odigranog_poteza.figure_napadaju_polje(
                &polje_mog_kralja,
                protivnikove_figure, 
                protivnik_ima_bele_figure,   
                &tabla_nakon_odigranog_poteza.to_nekompresirana_tabla()
            )   
    }
 
    

    pub fn figure_napadaju_polje(&self, polje_meta: &File_rank, figure: &[u8;16], boja_figura_je_bela: bool,
    nekompresirana_tabla: &Nekompresirana_tabla) -> bool {

        for i in 0..figure.len(){
            let polje_figure: File_rank = File_rank::new_iz_broja(figure[i]);
            match Figura::iz_niza_u_figure_interfejs(figure, i){
                None => {},
                Some(figura) =>
                 {
                    if (figura.napada_polje)(
                    nekompresirana_tabla,
                    polje_meta,
                    &polje_figure,
                    boja_figura_je_bela){
                        return true
                    }
                }
            }
        }

        false 
    }
        
}





#[cfg(test)]
mod test_provera_legalnosti{
    use crate::tabla::{Tabla, potez::{Potez, Potez_polje}, E_FILE, A_FILE, B_FILE, C_FILE, D_FILE, F_FILE, G_FILE, H_FILE, Promocija};


   
    fn test_potez_je_legalan(tabla: &Tabla, start_file: u8, start_rank: u8, end_file: u8, end_rank: u8) -> bool{
        tabla.potez_je_legalan_podrazumeva_se_da_figura_moze_doci_na_polje(&Potez_polje::new(start_file, start_rank, end_file, end_rank), &tabla.to_nekompresirana_tabla())
    }
    fn odigraj_potez(tabla: &Tabla, start_file: u8, start_rank: u8, end_file: u8, end_rank:u8) -> Tabla {
        tabla.odigraj_validan_potez_bez_promocije(start_file, start_rank, end_file, end_rank)
    }

    #[test]
    fn e4_je_legalan_potez(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        assert_eq!(true, 
            test_potez_je_legalan(&tabla, E_FILE, 2, E_FILE, 4));
    }

    #[test]
    fn d4_je_legalan_potez(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        assert_eq!(true,test_potez_je_legalan(&tabla, D_FILE, 2, D_FILE, 4));
    }

    #[test]
    fn test_promocija_na_h1(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(B_FILE, 1, C_FILE, 3)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 5)
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 5, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(F_FILE, 2, F_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 4, F_FILE, 3)
        .odigraj_validan_potez_bez_promocije(A_FILE,1 , B_FILE, 1)
        .odigraj_validan_potez_bez_promocije(F_FILE, 3, G_FILE, 2)
        .odigraj_validan_potez_bez_promocije(B_FILE, 1, A_FILE, 1);

        assert_eq!(true, Tabla::figura_je_pojedena(&tabla.bele_figure, 14));
        assert_eq!(true, Tabla::polja_se_slazu(tabla.crne_figure[11], crate::file_rank_to_broj(G_FILE, 2)));
        assert_eq!(true,
            test_potez_je_legalan(&tabla, G_FILE, 2, H_FILE, 1));
    }

    #[test]
    fn test_potez_Nd5_nije_moguc_jer_bi_ostavio_kralja_u_sahu(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(B_FILE, 1, C_FILE, 3)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 6)
        .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 4)
        .odigraj_validan_potez_bez_promocije(F_FILE, 8, B_FILE, 4);
        
        assert_eq!(false,
            test_potez_je_legalan(&tabla, C_FILE, 3, D_FILE, 5)
        );
        let tabla_nakon_sto_nema_vise_vezivanja: Tabla = tabla
        .odigraj_validan_potez_bez_promocije(A_FILE, 2, A_FILE, 3)
        .odigraj_validan_potez_bez_promocije(B_FILE, 4, C_FILE, 5);
        
        assert_eq!(true,
            test_potez_je_legalan(&tabla_nakon_sto_nema_vise_vezivanja, C_FILE, 3, D_FILE, 5)
        )
    }

    #[test]
    fn test_ne_mogu_da_odigram_Ra2_jer_ne_mogu_da_jedem_svoju_figuru(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        assert_eq!(
            false,
            test_potez_je_legalan(&tabla, A_FILE, 1, A_FILE, 2)
        );
        let tabla_nakon_sto_pomerim_pijuna: Tabla = tabla
        .odigraj_validan_potez_bez_promocije(A_FILE, 2, A_FILE, 3)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 5);
        assert_eq!(
            true,
            test_potez_je_legalan(&tabla_nakon_sto_pomerim_pijuna, A_FILE, 1, A_FILE, 2)
        );
    }

}