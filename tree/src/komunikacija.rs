use crate::tabla::{potez::Potez, Promocija};
use std::{io::{self, Error, Read, Write}, u8, net::{TcpListener, TcpStream}, thread};

use self::enkoder_poteza::Enkoder_poteza;
pub(crate) mod enkoder_poteza;


pub trait Komunikator{
    fn posalji_primi_potez(&mut self, potez: Option<Potez>) -> Potez;
    fn primi_potez(&mut self) -> Potez; 
}


pub struct Konzola_sah {

}
impl Konzola_sah {
    pub fn new() -> Konzola_sah{
        Konzola_sah{}
    }

    pub fn preuzmi_potez(&self) -> Potez {
        let mut pocetni_rank: String = String::new();
        println!("Upisite rank polja sa kog pomerate figuru: ");
        std::io::stdin().read_line(&mut pocetni_rank).expect("Greska");
        let pocetni_rank_br: u8 = pocetni_rank.trim().parse().expect("Niste upisali broj");

        let mut pocetni_file: String = String::new();
        println!("Upisite file polja sa kog pomerate figuru: ");
        std::io::stdin().read_line(&mut pocetni_file).expect("Greska.");
        let pocetni_file_br: u8 = pocetni_file.trim().parse().expect("Niste upisali broj");

        let mut zavrsni_rank: String = String::new();
        println!("Upisite rank destinacije: ");
        std::io::stdin().read_line(&mut zavrsni_rank).expect("Greska.");
        let krajnji_rank_br: u8 = zavrsni_rank.trim().parse().expect("Niste upisali broj");

        let mut zavrsni_file: String = String::new();
        println!("Upisite file destinacije: ");
        std::io::stdin().read_line(&mut zavrsni_file).expect("Greska");
        let krajnji_file_br: u8 = zavrsni_file.trim().parse().expect("Niste upisali broj");

        let mut promocija_string: String = String::new();
        println!("Upisite 0 za promociju kraljice, 1 za topa, 2 za lovca, 3 za konja, bilo koji drugi broj ako nema promocije.");
        std::io::stdin().read_line(&mut promocija_string).expect("Greska");
        let promocija_id: i64 = promocija_string.trim().parse().expect("Greska.");

        let mut promocija: Promocija = Promocija::None;

        if promocija_id == 0 {
            promocija = Promocija::KRALJICA;
        }
        if promocija_id == 1 {
            promocija = Promocija::TOP;
        }
        if promocija_id == 2 {
            promocija = Promocija::LOVAC;
        }
        if promocija_id ==3 {
            promocija = Promocija::KONJ;
        }

        println!("Sacekajte odgovor. Wait for response...");
        Potez::new(pocetni_file_br, pocetni_rank_br, krajnji_file_br, krajnji_rank_br, promocija)
    }
}

impl Komunikator for Konzola_sah {
    fn posalji_primi_potez(&mut self, potez: Option<Potez>) -> Potez {
        match potez {
            None => {println!("Odigrajte potez.");},
            Some(p) => {println!("Kompjuter je odigrao potez {}", p);}
        }
        self.preuzmi_potez()
    }

	fn primi_potez(&mut self) -> Potez {
		self.preuzmi_potez()
	}
	
}


pub enum Mozda_potez{
    Potez(Potez),
    None
}



pub struct Socket_komunikator{
    moj_port: u32,
    drugi_port: u32,
    ip_adresa_drugog_igraca: String,
    puna_moja_adresa: String,
    puna_adresa_drugog: String,
    cekam_poruku: bool,
    protivnikov_potez: Mozda_potez,
    enkoder_poteza: Box<dyn Enkoder_poteza>,
}

impl Socket_komunikator{
    pub fn new_localhost(moj_port: u32, drugi_port: u32, enkoder_poteza: Box<dyn Enkoder_poteza>) -> Socket_komunikator{
        Socket_komunikator{moj_port, drugi_port,
            ip_adresa_drugog_igraca: "127.0.0.1".to_owned(),
            puna_moja_adresa: String::from("127.0.0.1:".to_owned() + &moj_port.to_string()),
            puna_adresa_drugog: String::from("127.0.0.1:".to_owned() + &drugi_port.to_string()), 
            cekam_poruku: false,
            protivnikov_potez: Mozda_potez::None,
            enkoder_poteza,
        }
    }

    fn posalji_potez(&self, potez: &Potez){
        let bajtovi_poteza: Vec<u8> = self.enkoder_poteza.enkoduj_potez(&potez);

        let mut stream: TcpStream = TcpStream::connect(&self.puna_adresa_drugog).expect("Greska pri konekciji prilikom slanja.");
        stream.write(&bajtovi_poteza).expect("Greska pri slanju bajtovia poteza.");
        
    }

    /* Ovako bih napisao ako hocu da pisem server i obradjujem sve poruke klijenata.
        for stream in tcp_listener.incoming(){
            match stream{
                Err(e) => {eprintln!("greska stream-a. {}", e);},
                Ok(_stream) => {
                    self.handle_client(_stream);
                } 
            }
        } */
        /*
        let stream = tcp_listener.accept();
        match stream {
            Err(e) => eprintln!("Dogodila se greska prilikom citanja stream-a. {}", e),
            Ok((_stream, _socket_addr)) => {self.handle_client(_stream);}
        }
        */
    fn preuzmi_poruku(&mut self)  {
        println!("Cekam da stigne poruka na socket.");
        self.cekam_poruku = true;

        let tcp_listener: TcpListener = TcpListener::bind(&self.puna_moja_adresa).expect("Greska pri konekciji socket-a.");
        let stream = tcp_listener.accept();
        match stream {
            Err(e) => eprintln!("Dogodila se greska prilikom citanja stream-a. {}", e),
            Ok((_stream, _socket_addr)) => {self.handle_client(_stream);}
        }
    }

    fn handle_client(&mut self, mut stream: TcpStream) -> Result<(), std::io::Error>{
        println!("Stigla poruka na socket.");
        let mut buffer: [u8;256] = [0 as u8; 256];
        let broj_bajtova: usize = stream.read(&mut buffer)?;

        let protivnikov_potez: Potez = self.enkoder_poteza.desifruj_potez(&buffer);
        println!("Protivnikov potez: {}", &protivnikov_potez);
        self.protivnikov_potez = Mozda_potez::Potez(protivnikov_potez);
        self.cekam_poruku = false;
        Ok(())
    }


}

impl Komunikator for Socket_komunikator{
    fn posalji_primi_potez(&mut self, potez: Option<Potez>) -> Potez {
        self.posalji_potez(&potez.expect("Nijedan potez nije prosledjen soket komunikatoru poteza."));
        self.preuzmi_poruku();
        
        match &self.protivnikov_potez {
            Mozda_potez::None => panic!("Poruka je primljena, ali poteza nema."),
            Mozda_potez::Potez(_potez) => _potez.copy(),
            /* Ne radi ako napisem &Mozda_potez.  */
        }
    }

    /*while self.cekam_poruku {
            std::thread::sleep(std::time::Duration::from_millis(200));
        } */
    fn primi_potez(&mut self) -> Potez {
        self.preuzmi_poruku();
    
        match &self.protivnikov_potez {
            Mozda_potez::None => panic!("Poruka je primljena, ali poteza nema."),
            Mozda_potez::Potez(_potez) => _potez.copy(),
            /* Ne radi ako napisem &Mozda_potez.  */
        }
    }
}
