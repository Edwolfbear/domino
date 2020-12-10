#![allow(non_snake_case)]
#![allow(dead_code)]

use std::io;

trait CLI {
    fn exec(&mut self);

    fn input(&self, message : &str) -> String {
        let mut buffer = String::new();

        println!("{}", message);
        io::stdin().read_line(&mut buffer);

        return buffer;
    }

    fn options(&mut self, choosed : &str);

    fn prints(&self, title : &str) {  }

    fn pause(&self, message : &str) { self.input(message); }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Estado { Mano, Mesa, Pozo }

#[derive(Debug, Copy, Clone)]
struct Ficha { ficha : ((u8, u8), (u8, u8)), estado : Estado }
impl Ficha {
    fn new(ficha : ((u8, u8), (u8, u8))) -> Self {
        return Self { ficha : ficha, estado : Estado::Pozo };
    }

    fn mover(&mut self, estado : Estado) {
        self.estado = estado;
    }
}


#[derive(Debug)]
struct Juego { fichas : Vec<Ficha> }
impl Juego {
    fn new() -> Self {
        return Self { fichas : vec![
            Ficha::new(((0,0), (0,0))), Ficha::new(((0,1), (1,0))),
            Ficha::new(((0,2), (2,0))), Ficha::new(((0,3), (3,0))),
            Ficha::new(((0,4), (4,0))), Ficha::new(((0,5), (5,0))),
            Ficha::new(((0,6), (6,0))), Ficha::new(((1,1), (1,1))),
            Ficha::new(((1,2), (2,1))), Ficha::new(((1,3), (3,1))),
            Ficha::new(((1,4), (4,1))), Ficha::new(((1,5), (5,1))),
            Ficha::new(((1,6), (6,1))), Ficha::new(((2,2), (2,2))),
            Ficha::new(((2,3), (3,2))), Ficha::new(((2,4), (4,2))),
            Ficha::new(((2,5), (5,2))), Ficha::new(((2,6), (6,2))),
            Ficha::new(((3,3), (3,3))), Ficha::new(((3,4), (4,3))),
            Ficha::new(((3,5), (5,3))), Ficha::new(((3,6), (6,3))),
            Ficha::new(((4,4), (4,4))), Ficha::new(((4,5), (5,4))),
            Ficha::new(((4,6), (6,4))), Ficha::new(((5,5), (5,5))),
            Ficha::new(((5,6), (6,5))), Ficha::new(((6,6), (6,6))),
        ]};
    }

    fn tomar(&mut self, ficha : (u8, u8)) {
        let index : usize = self.fichas.iter().position(|f| {
            ficha == f.ficha.0 || ficha == f.ficha.1
        }).unwrap();

        self.fichas[index].estado = Estado::Mano;
    }

    fn poner(&mut self, ficha : (u8, u8)) {
        let index : usize = self.fichas.iter().position(|f| {
            ficha == f.ficha.0 || ficha == f.ficha.1
        }).unwrap();

        self.fichas[index].estado = Estado::Mesa;

    }

    fn mano(&self) -> Vec<Ficha> {
        return self.fichas.iter().filter(|ficha| {
            ficha.estado == Estado::Mano
        }).cloned().collect();
    }

    fn mesa(&self) -> Vec<Ficha> {
        return self.fichas.iter().filter(|ficha| {
            ficha.estado == Estado::Mesa
        }).cloned().collect();
    }

    fn pozo(&self) -> Vec<Ficha> {
        return self.fichas.iter().filter(|ficha| {
            ficha.estado == Estado::Pozo
        }).cloned().collect();
    }

    fn probabilidades(&self) -> Vec<(Ficha, f32)> {
        let mut result : Vec<(Ficha, f32)> = Vec::new();

        for enMano in self.mano().iter() {
            result.push((*enMano,
                ((self.pozo().iter().filter(|enPozo| {
                    enMano.ficha.0.0 == enPozo.ficha.0.0 || enMano.ficha.0.0 == enPozo.ficha.0.1
                }).cloned().collect::<Vec<Ficha>>().iter().len() as f32 / self.pozo().iter().len() as f32
                +
                self.pozo().iter().filter(|enPozo| {
                    enMano.ficha.0.1 == enPozo.ficha.0.0 || enMano.ficha.0.1 == enPozo.ficha.0.1
                }).cloned().collect::<Vec<Ficha>>().iter().len() as f32 / self.pozo().iter().len() as f32)
                / 2 as f32) * 100 as f32
            ));
        }

        return result;
    }
} impl CLI for Juego {
    fn prints(&self, title : &str) {
        match title {
            "inicio" => println!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                " _____   ______  __    __  __  __   __  ______\n/\\  __-./\\  __ \\/\\ \"-./  \\/",
                "\\ \\/\\ \"-.\\ \\/\\  __ \\\n\\ \\ \\/\\ \\ \\ \\/\\ \\ \\ \\-./\\ \\ \\ \\ \\ \\-",
                ".  \\ \\ \\/\\ \\\n \\ \\____-\\ \\_____\\ \\_\\ \\ \\_\\ \\_\\ \\_\\\\\"\\_\\ \\__",
                "___\\\n  \\/____/ \\/_____/\\/_/  \\/_/\\/_/\\/_/ \\/_/\\/_____/",
                " v0.1.5\n                                                   by Ed Wolfbear\n\n",
                "╔═══════════════════════╗\n",
                "║ Seleccione una opcion ║\n",
                "╠═══╦═══════════════════╣\n",
                "║ 0 ║ Nuevo Juego       ║\n",
                "╠═══╬═══════════════════╣\n",
                "║ 1 ║ Poner             ║\n",
                "╠═══╬═══════════════════╣\n",
                "║ 2 ║ Tomar             ║\n",
                "╠═══╬═══════════════════╣\n",
                "║ 3 ║ Fichas en mesa    ║\n",
                    "╠═══╬═══════════════════╣\n",
                    "║ 4 ║ Fichas en pozo    ║\n",
                    "╚═══╩═══════════════════╝\n"
            ),
            "ver mano" => {
                println!("\n{}{}",
                    "╔════════╦══════════════╗\n",
                    "║ Ficha  ║ probabilidad ║"
                );

                for enMano in self.probabilidades().iter() {
                    println!("{}\n{}",
                        "╠════════╬══════════════╣",
                        format!(
                            "║ {:?} ║    {:0width$.2}%    ║",
                            enMano.0.ficha.0, enMano.1, width = 5)
                    );
                }

                println!("╚════════╩══════════════╝");
            },
            _ => {  }
        }
    }

    fn exec(&mut self) { loop {
        self.prints("inicio");

        self.options(&self.input("Escriba el numero y presione enter...")[0..1]);
    }}

    fn options(&mut self, choosed : &str) {
        match choosed {
            "0" => {
                for i in 0..7 {
                    let ficha = &self
                        .input("\nEscribe ambos numeros de la ficha y presione enter...");

                    self.tomar((
                        *(&ficha[..1].parse().unwrap()),
                        *(&ficha[1..2].parse().unwrap())
                    ));
                }
            },
            "1" => {
                self.prints("ver mano");
                let ficha = &self
                    .input("\nEscribe ambos numeros de la ficha y presione enter...");

                self.poner((
                    *(&ficha[..1].parse().unwrap()),
                    *(&ficha[1..2].parse().unwrap())
                ));
            },
            "2" => {
                let ficha = &self
                    .input("\nEscribe ambos numeros de la ficha y presione enter...");

                self.tomar((
                    *(&ficha[..1].parse().unwrap()),
                    *(&ficha[1..2].parse().unwrap())
                ));
            },
            "3" => { for ficha in self.mesa().iter() { println!("{:?}", ficha); } },
            "4" => { for ficha in self.pozo().iter() { println!("{:?}", ficha); } },
            _ => {  }
        }
    }
}

fn main() {
    let mut juego = Juego::new();
    juego.exec();
}
