use std::marker::PhantomData;

pub struct Combustivel<Cm>
where
    Cm: Material,
{
    peso: Peso,
    phantom: PhantomData<Cm>,
}

impl <Cm> Combustivel<Cm>
where
    Cm: Material,
{
    pub fn gerar(peso: Peso) -> Self {
        Self {
            peso,
            phantom: PhantomData,
        }
    }
}

impl <Cm> Consumivel for Combustivel<Cm> {
    fn consumir(self) -> (Calor, Gas) {
        (Cm::ENERGIA * self.peso, Cm::GAS * self.peso)
    }
}



