use telegram_bot::*;

use crate::process;

pub async fn start(api: &Api, message: &Message) -> Result<(), Error> {
    api.send(message.text_reply(format!(
        "Olá, {}! Esse bot irá lhe ajudar a acompanhar o estado do seu processo.\n\
            Para começarmos, preciso que me envie o código do processo.",
        message.from.first_name
    )))
    .await?;
    Ok(())
}

pub async fn invalid(api: &Api, message: &Message) -> Result<(), Error> {
    api.send(message.text_reply("Comando ou codigo inválido"))
        .await?;

    start(&api, &message).await?;

    Ok(())
}

pub async fn process_unformatted_code(
    api: &Api,
    message: &Message,
    code: &str,
) -> Result<(), Error> {
    let code = format!("{}-{}-{}", &code[0..4], &code[4..8], &code[8..12]);
    println!("{}", code);
    process_code(&api, &message, &code).await
}

pub async fn process_code(api: &Api, message: &Message, code: &str) -> Result<(), Error> {
    let process = process::start(message.from.id.to_string(), code)
        .await
        .unwrap();
    let reply = if let Some(process) = process {
        format!(
            "Status: {}\n\
            Mensagem: {}",
            process.status, process.info
        )
    } else {
        "Processo não encontrado ou com status desconhecido.".to_owned()
    };

    api.send(message.text_reply(reply)).await?;

    Ok(())
}
