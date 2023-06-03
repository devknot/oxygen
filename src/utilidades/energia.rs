use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::marker::PhantomData;

use super::Comum;


pub struct Joule;

impl super::Energia for Joule {}

pub struct Energia<En = Joule>
where
    En: super::Energia,
{
    valor: Comum,
    phantom: PhantomData<En>,
}

impl <En> Energia<En>
where
    En: super::Energia,
{
    pub fn gerar(valor: Comum) -> Self {
        Self {
            valor,
            phantom: PhantomData,
        }
    }
}

impl fmt::Display for Energia<Joule> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}J", self.valor)
    }
}

impl <En> AddAssign<Self> for Energia<En>
where
    En: super::Energia,
{
    fn add_assign(&mut self, outro: Self) {
        self.valor += outro.valor;
    }
}

impl <En> Add<Self> for Energia<En>
where
    En: super::Energia,
{
    type Output = Self;
    
    fn add(self, outro: Self) -> Self::Output {
        Self::Output::gerar(self.valor + outro.valor)
    }
}


impl <En> SubAssign<Self> for Energia<En>
where
    En: super::Energia,
{
    fn sub_assign(&mut self, outro: Self) {
        self.valor -= outro.valor;
    }
}

impl <En> Sub<Self> for Energia<En>
where
    En: super::Energia,
{
    type Output = Self;
    
    fn sub(self, outro: Self) -> Self::Output {
        Self::Output::gerar(self.valor - outro.valor)
    }
}

/*
impl <Tm> Calor<Tm>
where
    Tm: Temperatura,
{   
    pub fn from<Tp>(calor: Calor<Tp>) -> Calor<Tm>
    where
        Tp: Temperatura,
    {
        Self::gerar(((calor.valor - Tp::FUSAO)/(Tp::EBULICAO - Tp::FUSAO))*(Tm::EBULICAO - Tm::FUSAO)+Tm::FUSAO) 
    }
}
*/


