

use crate::tabla::{Tabla, A_FILE, B_FILE, C_FILE, D_FILE, E_FILE, F_FILE, G_FILE, H_FILE};


pub fn benchmark_svi_legalni_potezi_posle_e4_e5_d4_d5(){
    let tabla: Tabla = Tabla::pocetna_pozicija()
    .odigraj_validan_potez_bez_promocije(E_FILE, 2, E_FILE, 4)
    .odigraj_validan_potez_bez_promocije(E_FILE, 7, E_FILE, 5)
    .odigraj_validan_potez_bez_promocije(D_FILE, 2, D_FILE, 4)
    .odigraj_validan_potez_bez_promocije(D_FILE, 7, D_FILE, 5)
    .odigraj_validan_potez_bez_promocije(A_FILE, 1, C_FILE, 5)
    .odigraj_validan_potez_bez_promocije(A_FILE, 7, A_FILE, 6)
    .odigraj_validan_potez_bez_promocije(B_FILE, 1, D_FILE, 5)
    .odigraj_validan_potez_bez_promocije(A_FILE, 6, A_FILE, 5)
    .odigraj_validan_potez_bez_promocije(D_FILE, 1, G_FILE, 4)
    .odigraj_validan_potez_bez_promocije(B_FILE, 7, B_FILE, 6)
    .odigraj_validan_potez_bez_promocije(C_FILE, 1, D_FILE, 2)
    .odigraj_validan_potez_bez_promocije(A_FILE, 8, A_FILE, 7);

    for _ in 0..100000{
        tabla.svi_legalni_potezi();
    }
    
}