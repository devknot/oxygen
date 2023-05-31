pub mod temperatura;
pub mod forca;
pub mod area;
pub mod massa;

pub type Comum = f32;

trait Zero {
    const ZERO: Self;
}

impl Zero for Comum {
    const ZERO: Self = 0.0;
}

pub trait Temperatura {
    const FUSAO: Comum; // fusão da água como referência
    const EBULICAO: Comum; // ebulição da água como referência
    const ZERO: Comum; // zero absoluto
}

pub trait Potencia {}

pub trait Superficie {}

pub trait Massa {}
