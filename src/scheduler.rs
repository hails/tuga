use crate::process;
use std::time::Duration;
use telegram_bot::types::refs::UserId;
use telegram_bot::*;
use tokio;

pub fn start(token: String) {
    // api.send(request: Req)
    tokio::spawn(async move {
        let api = Api::new(&token);

        loop {
            let citizenships = process::fetch_for_all().await.unwrap();

            for (telegram_user_id, process_response) in citizenships {
                let user_id = UserId::new(telegram_user_id.parse::<i64>().unwrap());
                let message = SendMessage::new(
                    user_id,
                    format!(
                        "Status: {}\n\
                        Mensagem: {}",
                        process_response.status, process_response.info
                    ),
                );
                api.send(message).await.unwrap();
            }
            tokio::time::delay_for(Duration::from_secs(1)).await;
        }
    });
}
