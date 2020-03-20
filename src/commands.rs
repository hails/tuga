use telegram_bot::*;

use crate::process;

pub async fn start(api: &Api, message: &Message) -> Result<(), Error> {
    api.send(message.text_reply(format!(
        "Olá, {}! Esse bot irá lhe ajudar a acompanhar o estado do seu processo.\n\
            Para começarmos, preciso que me envie o código do processo.\n\
            Simplesmente digite /code CODIGO que comecarei a acompanhar.",
        message.from.first_name
    )))
    .await?;
    Ok(())
}

pub async fn invalid(api: &Api, message: &Message) -> Result<(), Error> {
    api.send(message.text_reply(
        "Command not found!\n\
                Use /help to list the available commands",
    ))
    .await?;

    Ok(())
}

pub async fn code(api: &Api, message: &Message, data: &str) -> Result<(), Error> {
    let code: String = data.split_whitespace().skip(1).take(1).collect();

    let reply = if code.is_empty() {
        String::from("Você me precisa me enviar algum código")
    } else {
        let process = process::start(message.from.id.to_string(), code)
            .await
            .unwrap();
        format!(
            "Status: {}\n\
            Mensagem: {}",
            process.status, process.info
        )
    };

    api.send(message.text_reply(reply)).await?;

    Ok(())
}
