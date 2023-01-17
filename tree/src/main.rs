use tabla::{Tabla, E_FILE, D_FILE};

use crate::tabla::H_FILE;
use crate::komunikacija::{Komunikator, Konzola_sah};

mod drvo_eval;
mod tabla;
mod proba_sah_drveta;
mod komunikacija;

pub fn file_rank_to_broj(file: u8, rank: u8) -> u8 {
    ((rank-1) << 3) + file-1
}
pub fn broj_to_rank_file(mut broj: u8) -> (u8, u8){
    let prvih_6_bitova: u8 = (1<<6) - 1;
    broj &= prvih_6_bitova;

    let broj_sa_8: u8 = broj >> 3;
    let rank = broj_sa_8 + 1;
    let file = broj - (broj_sa_8 << 3) + 1;
    (rank, file)
}
fn min_max_broj(broj1: u8, broj2: u8) -> (u8, u8) {
    if broj1 < broj2 {
        (broj1, broj2)
    } else {
        (broj2, broj1)
    }
}


fn main() {
    
   /*  drvo_eval::proba();
    crate::tabla::potez::print_size_of_Potez_bits();
    let tabla: Tabla = Tabla::pocetna_pozicija(); 
    tabla.svi_legalni_potezi(); */
   // proba();
   //odigraj_partiju(true, 4);
   partije();
    //odigraj_partiju2(true, 2);
}

fn partije(){
    let mut beli_ili_crni: String = String::new();
    println!("1 za belog, 2 za crnog");
    std::io::stdin().read_line(&mut beli_ili_crni).expect("Greska");
    if beli_ili_crni.trim().starts_with("2"){
        odigraj_partiju(false, 4);
    } else {
        odigraj_partiju(true, 4);
    }
}


fn proba(){
    let tabla: Tabla = Tabla::pocetna_pozicija()
        .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
        .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
        .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 4)
        .odigraj_validan_potez_bez_promocije(D_FILE, 8, H_FILE, 4);
        let (potez, eval) = tabla.najbolji_potez_i_njegova_evaluacija(4);    
        println!("najbolji potez posle e4,e5,d4,Qh4:\n {}\n njegova evaluacija: {}", potez.unwrap().to_Potez(&tabla.figure_koje_su_na_potezu()), eval);

    }


fn odigraj_partiju(kompjuter_je_beli: bool, dubina_pretrage: u8){
    Tabla::pocni_partiju(Konzola_sah::new(), kompjuter_je_beli, dubina_pretrage)
}

fn odigraj_partiju2(kompjuter_je_beli: bool, dubina_pretrage: u8) {
    Tabla::pocni_partiju2(Konzola_sah::new(), kompjuter_je_beli, dubina_pretrage)
}

#[cfg(test)]
mod main_test{
    use crate::{broj_to_rank_file, file_rank_to_broj};

    #[test]
    fn test_63_rank_8_file_8(){
        let (rank, file) = broj_to_rank_file(63);
        assert_eq!(8, rank);
        assert_eq!(8, file);
    }

    #[test]
    fn test_41_rank_6_file_2(){
        let (rank, file) = broj_to_rank_file(41);
        assert_eq!(6, rank);
        assert_eq!(2, file);
    }

    #[test]
    fn test_3_rank_1_file_4(){
        let (rank, file) = broj_to_rank_file(3);
        assert_eq!(1, rank);
        assert_eq!(4, file);
    }

    #[test]
    fn test_10_rank_2_file_3(){
        let (rank, file) = broj_to_rank_file(10);
        assert_eq!(2, rank);
        assert_eq!(3, file);
    }


    #[test]
    fn rank_3_file_4_daje_19(){
        let polje: u8 = file_rank_to_broj(4, 3);
        assert_eq!(19, polje);
    }
}



