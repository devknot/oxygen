
use std::marker::PhantomData;

use crate::material;
use crate::itens;
use crate::erros;

pub struct Fornalha<Cs>
where
    Cs: material::consumivel::Consumivel,
{
    quantidade: itens::Comum,
    phantom: PhantomData<Cs>,
}

impl <Cs> material::consumidor::Consumidor for Fornalha<Cs>
where
    Cs: material::consumival::Consumivel,
{
    type Resultante = (Cs::Energia, Cs::Resto);
    
    fn consumir(&mut self, quantidade: itens::Comum) -> Result<Self::Resultante, erros::material::Erro> {
        if self.quantidade < quantidade {
            return Err(erros::material::Erro::Sc);
        }

        self.quantidade -= quantidade;

        (Cs::Energia * quantidade, Cs::Resto * quantidade)
    }
}

impl Fornalha<itens::item::Carvao> {
    pub fn gerar() -> Self {
        Self {
            quantidade: 0,
            phantom: PhantomData,
        }
    }
}
