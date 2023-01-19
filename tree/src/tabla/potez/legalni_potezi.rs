use crate::tabla::{Tabla, kretanje_figura::Figura_interfejs, Figura, Rokada, Promocija, Ima_podatke_o_tabli, File_rank, nekompresirana_tabla::Nekompresirana_tabla};

use super::{Potez_bits, Potez, Potez_polje};



impl Tabla {

    pub fn table_nakon_svih_legalnih_poteza(&self) -> Vec<Tabla>{
        let mut table_nakon_legalnih_poteza: Vec<Tabla> = Vec::new();

        let svi_legalni_potezi: Vec<Potez_bits> = self.svi_legalni_potezi();
        for legalan_potez in svi_legalni_potezi {
            let tabla: Tabla = self.tabla_nakon_poteza_bits(&legalan_potez);
            table_nakon_legalnih_poteza.push(tabla);
        }

        table_nakon_legalnih_poteza
    }
    
    fn potez_je_legalan(&self, nekompresirana_tabla: &Nekompresirana_tabla, figure: &[u8;16], 
        broj_figure: u8, polje_destinacije: &File_rank, potez: &Potez_polje, 
        figura: &Figura_interfejs<Nekompresirana_tabla>, beli_je_na_potezu: bool) -> bool {

        let trenutno_polje_figure: File_rank = File_rank::new_iz_broja(figure[broj_figure as usize]);

        if !(figura.figura_moze_doci_na_polje)(
                nekompresirana_tabla,
                polje_destinacije,
                &trenutno_polje_figure,
                beli_je_na_potezu
        ){
                return false
        }

        self.potez_je_legalan_podrazumeva_se_da_figura_moze_doci_na_polje(potez, nekompresirana_tabla)
    }

    pub fn svi_legalni_potezi(&self) -> Vec<Potez_bits>{
        let mut legalni_potezi: Vec<Potez_bits> = Vec::new();
        let figure: &[u8;16] = self.figure_koje_su_na_potezu();       
        let rokada: &Rokada = &self.rokada();
        let fajl_en_passant_pijuna: Option<u8> = self.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu();
        let beli_je_na_potezu: bool = self.beli_je_na_potezu();
        let nekompresirana_tabla: Nekompresirana_tabla = self.to_nekompresirana_tabla();

        for i in 0..figure.len(){
            let figura_option: Option<Figura_interfejs<Nekompresirana_tabla>> = Figura::iz_niza_u_figure_interfejs(figure, i);
            match figura_option {
                None => {},
                Some(figura) => {
                    let trenutno_polje_figure: File_rank = File_rank::new_iz_broja(figure[i]);
                    let potezi_figure: Vec<File_rank> = (&figura.potezi_figure)(&nekompresirana_tabla, &trenutno_polje_figure, rokada, &fajl_en_passant_pijuna, beli_je_na_potezu);

                    for polje in potezi_figure{                  
                        let potez: Potez_bits = Potez_bits{broj_figure: i as u8, file: polje.file, rank: polje.rank, promocija: Promocija::None};
                        let potez_polje: Potez_polje = Potez_polje::new(trenutno_polje_figure.file, trenutno_polje_figure.rank, polje.file, polje.rank);
                        if self.nisam_u_sahu_nakon_poteza(&potez_polje) {
                            Tabla::ubaci_poteze_u_listu(&mut legalni_potezi, potez, figure, i, polje.rank);                            
                        }   
                    }
                }    
            }
        }


        legalni_potezi
    }


    
    fn ubaci_poteze_u_listu(legalni_potezi: &mut Vec<Potez_bits>, potez: Potez_bits, figure: &[u8;16], broj_figure: usize, rank_destinacije: u8){
        let mut potez: Potez_bits = potez;

        if !Tabla::pijun_je_dosao_do_kraja_table(figure, broj_figure, rank_destinacije){
            potez.promocija = Promocija::None;
            legalni_potezi.push(potez);
         } else {
            potez.promocija = Promocija::KRALJICA;
            legalni_potezi.push(potez.copy());
            potez.promocija = Promocija::TOP;
            legalni_potezi.push(potez.copy());
            potez.promocija = Promocija::LOVAC;
            legalni_potezi.push(potez.copy());
            potez.promocija = Promocija::KONJ;
            legalni_potezi.push(potez);
        }
    }

    fn pijun_je_dosao_do_kraja_table(figure: &[u8;16], broj_figure: usize, rank: u8) -> bool {
        if !Tabla::figura_je_pijun(figure, broj_figure){
            return false
        }

        rank == 1 || rank == 8
    }


    pub fn nema_legalnih_poteza(&self, nekompresirana_tabla: &Nekompresirana_tabla) -> bool{
        let figure: &[u8;16] = self.figure_koje_su_na_potezu();       
        let rokada: &Rokada = &self.rokada();
        let fajl_en_passant_pijuna: Option<u8> = self.fajl_pijuna_koji_se_pomerio_2_polja_u_proslom_potezu();
        let beli_je_na_potezu: bool = self.beli_je_na_potezu();

        for i in 0..figure.len(){
            let figura_option: Option<Figura_interfejs<Nekompresirana_tabla>> = Figura::iz_niza_u_figure_interfejs(figure, i);
            match figura_option {
                None => {},
                Some(figura) => {
                    let trenutno_polje_figure: File_rank = File_rank::new_iz_broja(figure[i]);
                    let polja_prirodnog_kretanja: Vec<File_rank> = (&figura.potezi_figure)(&nekompresirana_tabla, &trenutno_polje_figure, rokada, &fajl_en_passant_pijuna, beli_je_na_potezu);

                    if polja_prirodnog_kretanja.len() > 0 {
                        return false
                    }
                }    
            }
        }

        true
    }

}




#[cfg(test)]
mod test_legalni_potezi{
    use crate::tabla::{Tabla, potez::{Potez_bits, Potez, Potez_polje}, Promocija, G_FILE, F_FILE, E_FILE, D_FILE, C_FILE, B_FILE, H_FILE, A_FILE, DESNI_LOVAC};



    #[test]
    fn pocetna_pozicija_ima_20_legalnih_poteza(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        assert_eq!(20, tabla.svi_legalni_potezi().len());
    }
    
    #[test]
    fn protivnik_ima_20_poteza_na_pocetku(){
        let tabla: Tabla = Tabla::pocetna_pozicija().odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4);
        assert_eq!(20, tabla.svi_legalni_potezi().len());
    }

    #[test]
    fn Nf3_je_legalan_pocetni_potez(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let Nf3: Potez = Potez::new(G_FILE, 1, F_FILE, 3, Promocija::None);
        assert_eq!(true, tabla.svi_legalni_potezi().contains(&Nf3.to_Potez_bits(&tabla).unwrap()));
    }

    #[test]
    fn e4_je_legalan_pocetni_potez(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let e4: Potez = Potez::new(E_FILE, 2, E_FILE, 4, Promocija::None);
        assert_eq!(true, tabla.svi_legalni_potezi().contains(&e4.to_Potez_bits(&tabla).unwrap()));
    }


    #[test]
    fn na_pocetku_Bf4_nije_legalno_ali_posle_d4_jeste(){
        let tabla: Tabla = Tabla::pocetna_pozicija();
        let Bf4: Potez = Potez::new(C_FILE, 1, F_FILE, 4, Promocija::None);
        assert_eq!(false, tabla.svi_legalni_potezi().contains(&Bf4.to_Potez_bits(&tabla).unwrap()));
        
        let tabla_nakon_d4: Tabla = tabla
        .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 5);

        assert_eq!(true, tabla_nakon_d4.svi_legalni_potezi().contains(&Bf4.to_Potez_bits(&tabla_nakon_d4).unwrap()));
    }

    #[test]
    fn ima_43_legalna_poteza_posle_e4_e5_Nf3_Nc6_Bc4_Bc5_d4_Nf6(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(G_FILE, 1, F_FILE, 3)
        .odigraj_validan_potez_bez_promocije(B_FILE, 8, C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4)
        .odigraj_validan_potez_bez_promocije(F_FILE, 8, C_FILE, 5)
        .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 4)
        .odigraj_validan_potez_bez_promocije(G_FILE, 8, F_FILE, 6);
        assert_eq!(43, tabla.svi_legalni_potezi().len());
    }



    #[test] 
    fn ima_53_legalnih_poteza_posle_e4_d5_exd5_c6_dxc6_Nf6_cxb7_e6_Nf3_Bc5_Bc4_Qc7_Nc3_Qd8_d3_Qd7_Bd2_Qd8_Qe7_Qc7(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 5)
        .odigraj_validan_potez_bez_promocije(E_FILE, 4, D_FILE, 5)
        .odigraj_validan_potez_bez_promocije(C_FILE, 7, C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(D_FILE, 5, C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(G_FILE, 8, F_FILE, 6)
        .odigraj_validan_potez_bez_promocije(C_FILE, 6, B_FILE, 7)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 6)
        .odigraj_validan_potez_bez_promocije(G_FILE, 1, F_FILE, 3)
        .odigraj_validan_potez_bez_promocije(F_FILE, 8, C_FILE, 5)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 8, C_FILE, 7)
        .odigraj_validan_potez_bez_promocije(B_FILE, 1, C_FILE, 3)
        .odigraj_validan_potez_bez_promocije(C_FILE, 7, D_FILE, 8)
        .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 3)
        .odigraj_validan_potez_bez_promocije(D_FILE, 8, C_FILE, 7)
        .odigraj_validan_potez_bez_promocije(C_FILE, 1, D_FILE, 2)
        .odigraj_validan_potez_bez_promocije(C_FILE, 7, D_FILE, 8)
        .odigraj_validan_potez_bez_promocije(D_FILE, 1, E_FILE, 2)
        .odigraj_validan_potez_bez_promocije(D_FILE, 8, C_FILE, 7);

        assert_eq!(53, tabla.svi_legalni_potezi().len());
    }

    #[test]
    fn treba_da_ima_23_legalna_poteza_posle_c4_e5_c5_b5_zbog_en_passant(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(C_FILE, 2, C_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(C_FILE, 4, C_FILE, 5)
        .odigraj_validan_potez_bez_promocije(B_FILE, 7, B_FILE, 5);

        assert_eq!(23, tabla.svi_legalni_potezi().len());
    }

    #[test]
    fn posle_e4_e5_Qh5_Nc6_Bc4_Nf6_Qxf7_nema_legalnih_poteza(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(D_FILE, 1, H_FILE, 5)
        .odigraj_validan_potez_bez_promocije(B_FILE, 8, C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4)
        .odigraj_validan_potez_bez_promocije(G_FILE, 8, F_FILE, 6)
        .odigraj_validan_potez_bez_promocije(H_FILE, 5, F_FILE, 7);

        let legalni_potezi: Vec<Potez_bits> = tabla.svi_legalni_potezi();
        assert_eq!(0, legalni_potezi.len());
    }


    /* Testiranje table_posle_svih_legalnih_poteza */
    #[test]
    fn test_table_posle_svih_legalnih_poteza_treba_da_ima_43_poteza(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(G_FILE, 1, F_FILE, 3)
        .odigraj_validan_potez_bez_promocije(B_FILE, 8, C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4)
        .odigraj_validan_potez_bez_promocije(F_FILE, 8, C_FILE, 5)
        .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 4)
        .odigraj_validan_potez_bez_promocije(G_FILE, 8, F_FILE, 6);
        assert_eq!(43, tabla.table_nakon_svih_legalnih_poteza().len());
    }



    /* Testiranje funkcije nema_legalnih_poteza */
    #[test]
    fn testiraj_nema_legalnih_poteza_mat_u_4_poteza(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(D_FILE, 1, H_FILE, 5)
        .odigraj_validan_potez_bez_promocije(B_FILE, 8, C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4)
        .odigraj_validan_potez_bez_promocije(G_FILE, 8, F_FILE, 6)
        .odigraj_validan_potez_bez_promocije(H_FILE, 5, F_FILE, 7);

        assert_eq!(true, tabla.nema_legalnih_poteza(&tabla.to_nekompresirana_tabla()));
    }


    #[test]
    fn test_21_potez_posle_e4_h5_d4_Rh7_Nf3_Rh1_Bc4(){
        
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(A_FILE, 7, A_FILE, 5)
        .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 4)
        .odigraj_validan_potez_bez_promocije(A_FILE, 8, A_FILE, 7)
        .odigraj_validan_potez_bez_promocije(G_FILE, 1, F_FILE, 3)
        .odigraj_validan_potez_bez_promocije(A_FILE, 7, A_FILE, 8)
        .odigraj_validan_potez_bez_promocije(F_FILE, 1, C_FILE, 4);

        
        assert_eq!(21, tabla.svi_legalni_potezi().len());
        assert_eq!(false, tabla.svi_legalni_potezi().contains(&Potez::new(A_FILE, 5, B_FILE, 4, Promocija::None).to_Potez_bits(&tabla).unwrap()));
        assert_eq!(true, tabla.svi_legalni_potezi().contains(&Potez::new(A_FILE, 5, A_FILE, 4, Promocija::None).to_Potez_bits(&tabla).unwrap()));
    }

    #[test]
    fn test(){
        let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(B_FILE, 1, C_FILE, 3)
        .odigraj_validan_potez_bez_promocije(B_FILE, 8, C_FILE, 6)
        .odigraj_validan_potez_bez_promocije(A_FILE, 1, B_FILE, 1)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(A_FILE, 2, A_FILE, 1)
        .odigraj_validan_potez_bez_promocije(F_FILE, 8, C_FILE, 5)
        .odigraj_validan_potez_bez_promocije(C_FILE, 3, B_FILE, 1)
        .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 6)
        .odigraj_validan_potez_bez_promocije(B_FILE, 1, C_FILE, 3)
        .odigraj_validan_potez_bez_promocije(D_FILE, 8, F_FILE, 6)
        .odigraj_validan_potez_bez_promocije(G_FILE, 1, H_FILE, 3);


        assert_eq!(true, tabla.svi_legalni_potezi().contains(&Potez::new(C_FILE, 8, H_FILE, 3, Promocija::None).to_Potez_bits(&tabla).unwrap()));
        assert_eq!(45, tabla.svi_legalni_potezi().len());
    }
}

