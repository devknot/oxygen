
type Id = u16;

#[derive(Debug, PartialEq)]
pub enum Tipo {
    Combustivel,
}

pub trait Item {
    const ID: Id;
    const TIPO: Tipo;
}

