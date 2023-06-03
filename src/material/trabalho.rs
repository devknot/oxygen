
use super::recipiente;
use super::receita;

pub trait Trabalho<Rc>
where
    Rc: receita::Receita,
{
    type Resultante;
    
    fn preparar(&mut self, recipiente: recipiente::Recipiente<Rc>);  // -> Result<(), >;
    fn retirar(&mut self) -> Self::Resultante;
}

/*
pub struct Ficticio;

impl Trabalho for Ficticio {
    type Resultante = ();
    
    fn preparando(&mut self, _: recipiente::Recipiente<>) {
        
    }
    
    fn retirando(&mut self) -> Self::Resultante {
        ()
    }
}
*/
