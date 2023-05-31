
use super::Item;
use super::Tipo;

pub struct Carvao;

impl Item for Carvao {
    const TIPO: Tipo = Tipo::Consumivel;
}

