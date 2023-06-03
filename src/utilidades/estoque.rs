
use crate::erros;

pub struct Estoque<Data> {
    data: Vec<Data>,
    maximo: usize,
    //fornecedor: Arc<Fornecedor>,
}

impl <Data> Estoque<Data> {
    pub fn gerar(maximo: usize) -> Self {
        Self {
            data: Vec::with_capacity(maximo),
            maximo,
        }
    }

    pub fn estocar(&mut self, item: Data) -> Result<(), erros::utilidades::Erro> {
        if self.data.len() == self.maximo {
            return Err(erros::utilidades::Erro::Cheio);
        }

        self.data.push(item);

        Ok(())
    }

    pub fn retirar(&mut self) -> Option<Data> {
        self.data.pop()
    }
}

