
/*
pub enum Comando {
    Pare,
    Continue,
}

pub struct Contexto;
*/

pub struct Conteiner;

pub trait Trabalho {
    type Resultante;
    fn preparando(&mut self, conteiner: Conteiner);  // -> Result<(), >;
    fn retirando(&mut self) -> Resultante;
}

pub struct Ficticio;

impl Trabalho for Ficticio {
    type Resultante = ();
    
    fn preparando(&mut self, _: Conteiner) {
        
    }
    
    fn retirando(&mut self) -> Self::Resultante {
        Self::Resultante
    }
}
