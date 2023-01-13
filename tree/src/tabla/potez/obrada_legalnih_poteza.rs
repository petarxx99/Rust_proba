use crate::tabla::{Tabla, KRALJ};

use super::Potez_bits;




impl Tabla {
    pub fn potez_je_sah(&self, potez: &Potez_bits) -> bool {
        let ja_sam_beo: bool = self.beli_je_na_potezu();
        let tabla_nakon_poteza: Tabla = self.tabla_nakon_poteza_bits(potez);
        let protivnicki_kralj: u8 = tabla_nakon_poteza.figure_koje_su_na_potezu()[KRALJ];
        let moje_figure: &[u8;16] = tabla_nakon_poteza.figure_koje_nisu_na_potezu();

        tabla_nakon_poteza.figure_napadaju_polje(
            protivnicki_kralj, 
            moje_figure,
            ja_sam_beo)
    }


}

#[cfg(test)]
mod test_obrada_legalnih_poteza{
    use crate::tabla::{Tabla, potez::Potez, F_FILE, B_FILE, D_FILE, E_FILE, Promocija, H_FILE};


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


}