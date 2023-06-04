use std::pin;

pub struct Contexto;

impl Contexto {
    pub fn gerar() -> Self {
        Self
    }
    pub fn contextualizar<'contexto>(&'contexto self) -> pin::Pin<&'contexto Self> {
        pin::Pin::new(self)
    }
}

