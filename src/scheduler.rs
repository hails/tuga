use crate::process;
use std::env;
use std::time::Duration;
use telegram_bot::types::refs::UserId;
use telegram_bot::*;
use tokio;

pub fn start(token: String) {
    tokio::spawn(async move {
        let update_in_secs = env::var("UPDATE_DATA_IN_SECS").expect("UPDATE_DATA_IN_SECS not set");
        let update_in_secs = update_in_secs.parse::<u64>().unwrap();

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
            tokio::time::delay_for(Duration::from_secs(update_in_secs)).await;
        }
    });
}
