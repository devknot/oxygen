
//use crate::utilidades::energia;
use crate::erros;
use crate::itens;
//use super::consumivel;

pub trait Consumidor {
    type Resultante;

    fn consumir(&mut self, quantidade: itens::Comum) -> Result<Self::Resultante, erros::material::Erro>;
}


