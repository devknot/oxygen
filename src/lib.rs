pub mod estruturas;
pub mod material;
pub mod compartimento;
pub mod erros;
pub mod gases;
pub mod itens;
pub mod utilidades;
pub mod contexto;

mod teste;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = teste::add(2, 2);
        assert_eq!(result, 4);
    }
}

