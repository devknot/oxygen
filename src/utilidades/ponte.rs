
enum Mensagem<Msg> {
    Enviar(Msg),
    Esperando,
}

pub struct Ponte<'ponte, En, Msg> {
    enviar: &'ponte En,
    mensagem: Box<Mensagem<Msg>>,
}

trait Contratar<'cm, Cm> {
    fn receber_sinal(&mut self, &'cm Cm);
    fn enviar_sinal(&self);
}

impl <'ponte, En, Msg> Ponte<'ponte, En, Msg> {
    pub fn gerar(enviar: &'ponte mut En) -> Self {
        let retorno: Self = Self {
            enviar,
            mensagem: Mensagem::Esperando,
        };

        enviar.receber_sinal(&retorno);
        enviar.enviar_sinal();
        
        retorno
    }

    fn enviar_contrato(enviar: &'ponte En) {
        enviar
    }

    fn receber_contrato() {
        
    }
}

