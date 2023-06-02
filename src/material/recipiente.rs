
use super::receita;
use crate::utilidades::norma;


pub struct Recipiente<Rc>
where
    Rc: receita::Receita,
{
    quantidade: Option<u8>, //quantidade a ser produzido. None é igual ao infinito
    estoque: Estoque<Rc::Receituario, norma::ESTOQUE>,
}


impl Recipiente<Rc>
where
    Rc: receita::Receita,
{
    pub fn gerar(quantidade: Option<u8>) -> Self {
        Self {
            quantidade,
            estoque: Estoque::gerar(),
        }
    }

    pub fn retirar(&mut self) -> Option<Rc::Receituario> {
        self.estoque.retirar()
    }
}

