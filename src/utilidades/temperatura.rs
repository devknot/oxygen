use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::marker::PhantomData;

use super::Temperatura;
use super::Comum;


pub struct Kelvin;

impl Temperatura for Kelvin {
    const FUSAO: Comum = 273.15;
    const EBULICAO: Comum = 373.15;
    const ZERO: Comum = 0.0;
}

pub struct Celsius;

impl Temperatura for Celsius {
    const FUSAO: Comum = 0.0;
    const EBULICAO: Comum = 100.0;
    const ZERO: Comum = -273.15;
}

pub struct Calor<Tm = Kelvin>
where
    Tm: Temperatura,
{
    valor: Comum,
    phantom: PhantomData<Tm>,
}

impl <Tm> Calor<Tm>
where
    Tm: Temperatura,
{
    pub fn gerar(valor: Comum) -> Self {
        Self {
            valor,
            phantom: PhantomData,
        }
    }
}

impl fmt::Display for Calor<Kelvin> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}ºk", self.valor)
    }
}

impl fmt::Display for Calor<Celsius> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}ºc", self.valor)
    }
}

impl <Tm> AddAssign<Calor<Tm>> for Calor<Tm>
where
    Tm: Temperatura,
{
    fn add_assign(&mut self, calor: Calor<Tm>) {
        self.valor += calor.valor;
    }
}

impl <Tm> Add<Calor<Tm>> for Calor<Tm>
where
    Tm: Temperatura,
{
    type Output = Self;
    
    fn add(self, calor: Calor<Tm>) -> Self::Output {
        Calor::gerar(self.valor + calor.valor)
    }
}


impl <Tm> SubAssign<Calor<Tm>> for Calor<Tm>
where
    Tm: Temperatura,
{
    fn sub_assign(&mut self, calor: Calor<Tm>) {
        self.valor -= calor.valor;
    }
}

impl <Tm> Sub<Calor<Tm>> for Calor<Tm>
where
    Tm: Temperatura,
{
    type Output = Self;
    
    fn sub(self, calor: Calor<Tm>) -> Self::Output {
        Calor::gerar(self.valor - calor.valor)
    }
}


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



