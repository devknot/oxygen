
use crate::utilidades;

pub trait Receita {
    const NECESSARIO: utilidades::Comum; // energia necessaria para fazer esta receita
    type Produto; // produto final
}

