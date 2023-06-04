
use crate::material;
use crate::utilidades;

pub struct Gerador<'gerador> {
    energia: utilidades::ponte::Ponte<'gerador>,
}

impl <'gerador, Extensao> material::trabalho::Trabalho<'gerador, Extensao> for Gerador {
    fn gerar(&mut self, energia: utilidades::ponte::Ponte<'gerador>, _: Extensao) -> Self {
        Self {
            energia,
        }
    }
    //fn receber(&mut self)
}

