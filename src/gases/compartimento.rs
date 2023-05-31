use std::marker::PhantomData;

use crate::utilidades;

use super::Gas;



pub struct Compartimento<Gs> 
where
    Gs: Gas,
{
    valor: utilidades::Comum,
    phantom: PhantomData<Gs>,
}

impl <Gs> Compartimento<Gs>
where
    Gs: Gas,
{
    pub fn gerar(valor: utilidades::Comum) -> Self {
        Self {
            valor,
            phantom: PhantomData,
        }
    }
}

pub struct Vacuo;

impl Vacuo {
    pub fn gerar(_: utilidades::Comum) -> Self {
        Self
    }
}

pub struct Void;

impl Void {
    pub fn gerar(_: utilidades::Comum) -> Self {
        Self
    }
}