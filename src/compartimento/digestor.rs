
use crate::item;

use crate::utilidades;

use crate::erro;

use crate::item;

use crate::material;

// pseudocódigo

pub trait Maquinario<'maquinario, 'consumivel, Tr, Cs, Cm>
where
    Tr: material::trabalho::Trabalho + 'maquinario,
    Cs: material::consumidor::Consumidor + 'maquinario,
    Cm: material::consumivel::Consumivel + 'consumivel,
{
    type resto;
    
    fn processar(&mut self) -> Result<Self::resto, erro::compartimento::Erro>;
    fn travar(&mut self) -> Result<(), erro::compartimento::Erro>;
    fn destravar(&mut self) -> Result<(), erro::compartimento::Erro>;
    fn taxa(&self) -> utilidades::Comum;
}

/*
impl Quardador for Fornalha {
    fn quardar<Im>(&mut self, im: Im) -> Result<(), (Im, erro::item)>
    where
        Im: item::Item,
    {
        if Im::Tipo != Tipo::Combustivel {
            Err((im, erro::item::Nec));
        }

        
    }
}
*/


