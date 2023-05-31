
use crate::erro::material;

pub trait Objeto {
    const CONSUMIVEL: bool,
}

pub trait Quardar {
    fn quardar<Material: Objeto>(&mut self, objeto: Material) -> Result<(), (Material, material::error)>;
}

