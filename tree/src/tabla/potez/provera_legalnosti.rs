use crate::tabla::{Ima_podatke_o_tabli, Tabla, kretanje_figura::Figura_interfejs, Figura, KRALJ, Promocija};

use super::Potez_bits;


impl Tabla {

    /* Ova metoda ocekuje da joj se preda potez takav da figura moze 
    doci do polja destinacije. Podrazumeva se da je taj uslov ispunjen. */
    fn potez_je_legalan_podrazumeva_se_da_figura_moze_doci_na_polje(&self, potez: &Potez_bits) -> bool {
        
        /* Ne mogu da pojedem sopstvenu figuru. */
        if self.da_li_je_figura_boje_na_polju(self.beli_je_na_potezu(), potez.rank, potez.file) {
            return false
        }

        if self.pijun_je_dosao_do_kraja_a_nije_promovisan(potez) {
            return false
        }

        self.nisam_u_sahu_nakon_poteza(potez)
    }

    fn pijun_je_dosao_do_kraja_a_nije_promovisan(&self, potez: &Potez_bits) -> bool{
     /* Ako pijun nije stigao do prvog ili osmog ranka, znaci da promocije nije ni bilo. */   
        if potez.rank != 1 && potez.rank != 8 {
            return false
        }

/* Prvih 8 figura nisu pijuni. */
        let broj_prvog_pijuna: u8 = 8;
        if potez.broj_figure < broj_prvog_pijuna {
            return false
        }
        /* Ostaje nam slucaj da je pomeren pijun i da je pomeren na prvi ili osmi rank. 
        To je slucaj koji nas zanima. Nije legalno da pijun dodje do kraja, a ne bude promovisan. */
        
        match &potez.promocija {
            &Promocija::None => true,
            _ => false
        }

    }

    fn nisam_u_sahu_nakon_poteza(&self, potez: &Potez_bits) -> bool{
        let polje_mog_kralja: u8 = self.figure_koje_su_na_potezu()[KRALJ];
        let tabla_nakon_odigranog_poteza: Tabla = self.tabla_nakon_poteza_bits(potez);
        let protivnikove_figure: &[u8;16] = tabla_nakon_odigranog_poteza.figure_koje_su_na_potezu();

        for i in 0..protivnikove_figure.len(){
            let figura_optional:Option<Figura_interfejs<Tabla>> = Figura::iz_niza_u_figure_interfejs(protivnikove_figure, i);
            let polje_protivnikove_figure: u8 = protivnikove_figure[i];
            match figura_optional {
                None => {},
                Some(protivnikova_figura) =>
                 {
                    let figura_napada_mog_kralja: bool = (protivnikova_figura.napada_polje)(
                    &tabla_nakon_odigranog_poteza,
                    polje_mog_kralja,
                    polje_protivnikove_figure,
                    tabla_nakon_odigranog_poteza.beli_je_na_potezu());

                    if figura_napada_mog_kralja {
                        return false
                    }
                }
            }
        }

        true    
    }
        
}



#[cfg(test)]
mod test_provera_legalnosti{
    use crate::tabla::{Tabla, potez::Potez, E_FILE, A_FILE, B_FILE, C_FILE, D_FILE, F_FILE, G_FILE, H_FILE, Promocija};


    fn potez_je_legalan(tabla: &Tabla, potez: &Potez) -> bool{
        tabla.potez_je_legalan_podrazumeva_se_da_figura_moze_doci_na_polje(&potez.to_Potez_bits(&tabla).unwrap())
    }
    fn potez_bez_promocije_je_legalan(tabla: &Tabla, start_file: u8, start_rank: u8, end_file: u8, end_rank: u8) -> bool{
        potez_je_legalan(tabla, &Potez::new(start_file, start_rank, end_file, end_rank, Promocija::None))
    }
    fn odigraj_potez(tabla: &Tabla, start_file: u8, start_rank: u8, end_file: u8, end_rank:u8) -> Tabla {
        tabla.odigraj_validan_potez_bez_promocije(start_file, start_rank, end_file, end_rank)
    }

    #[test]
    fn e4_je_legalan_potez(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        assert_eq!(true, 
            potez_bez_promocije_je_legalan(&tabla, E_FILE, 2, E_FILE, 4));
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
            potez_je_legalan(&tabla, &Potez::new(G_FILE, 2, H_FILE, 1, Promocija::KRALJICA)));
        assert_eq!(false,
        potez_bez_promocije_je_legalan(&tabla, G_FILE, 2, H_FILE, 1));
    }

    #[test]
    fn test_potez_Nd5_nije_moguc_jer_bi_ostavio_kralja_u_sahu(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(B_FILE, 1, C_FILE, 3)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 6)
        .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 4)
        .odigraj_validan_potez_bez_promocije(F_FILE, 8, B_FILE, 4);
        
        assert_eq!(false,
            potez_bez_promocije_je_legalan(&tabla, C_FILE, 3, D_FILE, 5)
        );
        let tabla_nakon_sto_nema_vise_vezivanja: Tabla = tabla
        .odigraj_validan_potez_bez_promocije(A_FILE, 2, A_FILE, 3)
        .odigraj_validan_potez_bez_promocije(B_FILE, 4, C_FILE, 5);
        
        assert_eq!(true,
            potez_bez_promocije_je_legalan(&tabla_nakon_sto_nema_vise_vezivanja, C_FILE, 3, D_FILE, 5)
        )
    }

    #[test]
    fn test_ne_mogu_da_odigram_Ra2_jer_ne_mogu_da_jedem_svoju_figuru(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        assert_eq!(
            false,
            potez_bez_promocije_je_legalan(&tabla, A_FILE, 1, A_FILE, 2)
        );
        let tabla_nakon_sto_pomerim_pijuna: Tabla = tabla
        .odigraj_validan_potez_bez_promocije(A_FILE, 2, A_FILE, 3)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 5);
        assert_eq!(
            true,
            potez_bez_promocije_je_legalan(&tabla_nakon_sto_pomerim_pijuna, A_FILE, 1, A_FILE, 2)
        );
    }

}