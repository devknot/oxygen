use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::marker::PhantomData;

use super::Superficie;
use super::Comum;

pub struct Metro;

impl Superficie for Metro {}


pub struct Area<Ar = Metro>
where
    Ar: Superficie,
{
    valor: Comum,
    phantom: PhantomData<Ar>,
}


impl <Ar> Area<Ar>
where
    Ar: Superficie,
{
    pub fn gerar(valor: Comum) -> Self {
        Self {
            valor,
            phantom: PhantomData,
        }
    }
}

impl fmt::Display for Area<Metro> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}m²", self.valor)
    }
}

impl <Ar> AddAssign<Area<Ar>> for Area<Ar>
where
    Ar: Superficie,
{
    fn add_assign(&mut self, area: Self) {
        self.valor += area.valor;
    }
}

impl <Ar> Add<Area<Ar>> for Area<Ar>
where
    Ar: Superficie,
{
    type Output = Self;
    
    fn add(self, area: Self) -> Self::Output {
        Self::gerar(self.valor + area.valor)
    }
}


impl <Ar> SubAssign<Area<Ar>> for Area<Ar>
where
    Ar: Superficie,
{
    fn sub_assign(&mut self, area: Self) {
        self.valor -= area.valor;
    }
}

impl <Ar> Sub<Area<Ar>> for Area<Ar>
where
    Ar: Superficie,
{
    type Output = Self;
    
    fn sub(self, area: Self) -> Self::Output {
        Self::gerar(self.valor - area.valor)
    }
}

/*
impl <Fr> Forca<Fr>
where
    Fr: Potencia,
{   
    pub fn from<Fc>(forca: Forca<Fc>) -> Forca<Fr>
    where
        Fc: Potencia,
    {
        Self::gerar(((calor.valor - Tp::FUSAO)/(Tp::EBULICAO - Tp::FUSAO))*(Tm::EBULICAO - Tm::FUSAO)+Tm::FUSAO) 
    }
}
*/

