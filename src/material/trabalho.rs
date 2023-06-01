
pub enum Comando {
    Pare,
    Continue,
}

pub trait Trabalho {
    fn comandar(&mut self, comando: &Comando);
}

pub struct Ficticio;

impl Trabalho for Ficticio {
    fn comandar(&mut self, _: &Comando) {
        
    }
}
