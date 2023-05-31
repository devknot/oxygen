use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::marker::PhantomData;

use super::Potencia;
use super::Comum;

pub struct Newton;

impl Potencia for Newton {}


pub struct Forca<Fr = Newton>
where
    Fr: Potencia,
{
    valor: Comum,
    phantom: PhantomData<Fr>,
}


impl <Fr> Forca<Fr>
where
    Fr: Potencia,
{
    pub fn gerar(valor: Comum) -> Self {
        Self {
            valor,
            phantom: PhantomData,
        }
    }
}

impl fmt::Display for Forca<Newton> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}N", self.valor)
    }
}

impl <Fr> AddAssign<Forca<Fr>> for Forca<Fr>
where
    Fr: Potencia,
{
    fn add_assign(&mut self, forca: Forca<Fr>) {
        self.valor += forca.valor;
    }
}

impl <Fr> Add<Forca<Fr>> for Forca<Fr>
where
    Fr: Potencia,
{
    type Output = Self;
    
    fn add(self, forca: Forca<Fr>) -> Self::Output {
        Forca::gerar(self.valor + forca.valor)
    }
}


impl <Fr> SubAssign<Forca<Fr>> for Forca<Fr>
where
    Fr: Potencia,
{
    fn sub_assign(&mut self, forca: Forca<Fr>) {
        self.valor -= forca.valor;
    }
}

impl <Fr> Sub<Forca<Fr>> for Forca<Fr>
where
    Fr: Potencia,
{
    type Output = Self;
    
    fn sub(self, forca: Forca<Fr>) -> Self::Output {
        Forca::gerar(self.valor - forca.valor)
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

