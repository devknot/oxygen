use std::marker::PhantomData;


type Comum = u16;

pub struct Itens<Im>
where
    Im: Item,
{
    quantidade: Comum,
    item: Box<Im>,
}

impl <Im> Itens<Im>
where
    Im: Item,
{
    pub fn gerar() -> Self {
        Self {
            quantidade: 0,
            item: Box::new(Im::gerar()),
        }
    }

    pub fn gerar_quantidade(quantidade: Comum) -> Self {
        Self {
            quantidade,
            item: Box::new(Im::gerar()),
        }
    }
    
    pub fn retirar(&mut self, quantidade: Comum) -> Option<Self> {
        if self.quantidade < quantidade {
            return None;
        }

        self.quantidade -= quantidade;

        Some(Self::gerar_quantidade(quantidade))
    }

    
}


pub trait Item {
    const ID: Comum;
    
    fn gerar() -> Self where Self: Sized;
}

