use crate::mail;
use crate::{model::AppConfig, script::get_vm};
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub async fn cron_fn(config: Arc<AppConfig>) -> Result<(), JobSchedulerError> {
    let sched = JobScheduler::new().await?;

    sched
        .add(Job::new_async("1/30 * * * * * *", move |_uuid, mut _l| {
            let c = Arc::clone(&config);

            Box::pin(async move {
                println!("[CRON] FISGANDO VM ...");

                match get_vm(&c.oci) {
                    Ok(status) => {
                        if status.success() {
                            println!("[SUCESSO] VM FOI CRIADA!!!");
                            if let Err(e) = mail::send_confirm(&c.resend).await {
                                eprintln!("[ERRO] Falha ao enviar o email: {:?}", e);
                            };
                        } else {
                            println!("[RETRY] Sem estoque ou Erro na API");
                        }
                    }
                    Err(e) => eprintln!("[ERRO] Falha ao executar o oci cli: {}", e),
                }
            })
        })?)
        .await?;

    sched.start().await?;

    Ok(())
}
