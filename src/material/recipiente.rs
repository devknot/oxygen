
use super::receita;
use crate::utilidades::{estoque, norma};


pub struct Recipiente<Rc>
where
    Rc: receita::Receita,
{
    quantidade: Option<u8>, //quantidade a ser produzido. None é igual ao infinito
    estoque: estoque::Estoque<Rc::Receituario>,
}


impl <Rc> Recipiente<Rc>
where
    Rc: receita::Receita,
{
    pub fn gerar(quantidade: Option<u8>) -> Self {
        Self {
            quantidade,
            estoque: estoque::Estoque::gerar(norma::ESTOQUE),
        }
    }

    pub fn receita(&mut self) -> Option<Rc::Receituario> {
        self.estoque.retirar()
    }

    pub fn receita_recebida(&mut self) {
        match self.quantidade {
            Some(ref mut itens) => {
                *itens -= 1
            }
            None => {},
        }
    }
    
    pub fn acabou(&self) -> bool {
        match self.quantidade {
            Some(itens) => {
                itens == 0
            },
            None => false,
        }
    }
}

