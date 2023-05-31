use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::marker::PhantomData;

use super::Massa;
use super::Comum;


pub struct KiloGrama;

impl Massa for KiloGrama {}

pub struct Peso<Qt = KiloGrama>
where
    Qt: Massa,
{
    valor: Comum,
    phantom: PhantomData<Qt>,
}

impl <Qt> Peso<Qt>
where
    Qt: Massa,
{
    pub fn gerar(valor: Comum) -> Self {
        Self {
            valor,
            phantom: PhantomData,
        }
    }
}

impl fmt::Display for Peso<KiloGrama> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}Kg", self.valor)
    }
}

impl <Qt> AddAssign<Self> for Peso<Qt>
where
    Qt: Massa,
{
    fn add_assign(&mut self, outro: Self) {
        self.valor += outro.valor;
    }
}

impl <Qt> Add<Self> for Peso<Qt>
where
    Qt: Massa,
{
    type Output = Self;
    
    fn add(self, outro: Self) -> Self::Output {
        Self::Output::gerar(self.valor + outro.valor)
    }
}


impl <Qt> SubAssign<Self> for Peso<Qt>
where
    Qt: Massa,
{
    fn sub_assign(&mut self, outro: Self) {
        self.valor -= outro.valor;
    }
}

impl <Qt> Sub<Self> for Peso<Qt>
where
    Qt: Massa,
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


