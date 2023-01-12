use tabla::Tabla;


mod drvo_eval;
mod tabla;

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


fn main() {
    drvo_eval::proba();
    crate::tabla::potez::print_size_of_Potez_bits();
    let tabla: Tabla = Tabla::pocetna_pozicija();
    tabla.svi_legalni_potezi();
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



