use crate::model::ResendCreds;
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};
use std::env;

impl ResendCreds {
    pub fn get_resend_creds() -> Self {
        Self {
            resend_key: env::var("RESEND_KEY").expect("KEY missing"),
            resend_from: env::var("RESEND_FROM").expect("FROM missing"),
            resend_to: env::var("RESEND_TO").expect("TO missing"),
        }
    }
}

pub async fn send_confirm(res_creds: &ResendCreds) -> Result<()> {
    let resend = Resend::new(&res_creds.resend_key);

    let from = &res_creds.resend_from;

    let to = &res_creds.resend_to;

    let subj = "VM CRIADA !";

    let email = CreateEmailBaseOptions::new(from, [to], subj)
        .with_html("<i><b>Vm foi criada com sucesso, check o <a href=\"https://cloud.oracle.com/compute/instances?region=sa-saopaulo-1\">gerenciador de instancias</a></b></i>");

    let _email = resend.emails.send(email).await?;

    Ok(())
}
