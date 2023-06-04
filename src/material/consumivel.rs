
use crate::itens;

pub trait Consumivel {
    const ENERGIA: itens::Comum;
    const RESTO: itens::Comum;
}

// 1 kilo de carvão produz 25 milhões de joules e 3 quilogramas de dióxido de carbono

impl Consumivel for itens::item::Carvao {
    const ENERGIA: itens::Comum = 25000000;
    const RESTO: itens::Comum = 3000;
}


