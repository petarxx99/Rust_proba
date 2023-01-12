use tabla::Tabla;


mod drvo_eval;
mod tabla;

pub fn file_rank_to_broj(file: u8, rank: u8) -> u8 {
    ((rank-1) << 3) + file-1
}
pub fn broj_to_rank_file(mut broj: u8) -> (u8, u8){
    let prvih_6_bitova: u8 = (1<<6) - 1;
    broj &= prvih_6_bitova;

    let rank = (broj>>3) + 1;
    let file = broj % 8 + 1;
    (rank, file)
}


fn main() {
    drvo_eval::proba();
    crate::tabla::potez::print_size_of_Potez_bits();
    let tabla: Tabla = Tabla::pocetna_pozicija();
    tabla.svi_legalni_potezi();
}





