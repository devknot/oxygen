
pub struct Nada;

impl super::Item for Nada {
    const ID: super::Comum = 0;
    
    fn gerar() -> Self
    where
        Self: Sized,
    {
        Self
    }
}

pub struct Carvao;

impl super::Item for Carvao {
    const ID: super::Comum = 1;
    
    fn gerar() -> Self
    where
        Self: Sized,
    {
        Self
    }
}

