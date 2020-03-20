use regex::Regex;
use reqwest;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use telegram_bot::*;

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

pub async fn code(api: &Api, message: &Message, data: &String) -> Result<(), Error> {
    let code: String = data.split_whitespace().skip(1).take(1).collect();

    let reply = if code.is_empty() {
        String::from("Você me precisa me enviar algum código")
    } else {
        let process = fetch_citizenship_status(&code).await.unwrap();
        format!(
            "Status: {}\n\
            Mensagem: {}",
            process.status, process.info
        )
    };

    api.send(message.text_reply(reply)).await?;

    Ok(())
}

struct Process {
    status: String,
    info: String,
}

async fn fetch_citizenship_status(code: &String) -> Result<Process, reqwest::Error> {
    let res = reqwest::Client::new()
        .post("https://nacionalidade.justica.gov.pt/Home/GetEstadoProcessoAjax")
        .form(&[("SenhaAcesso", code)])
        .send()
        .await?;
    let body = res.text().await?;

    let document = Document::from(body.as_str());

    let mut process = Process {
        status: String::from("Unknown"),
        info: String::from(""),
    };

    if let Some(st) = document.find(Class("active1").descendant(Name("p"))).last() {
        process.status = st.text();
    }

    if let Some(st) = document.find(Class("active2").descendant(Name("p"))).last() {
        process.status = st.text();
    }

    if let Some(st) = document.find(Class("active3").descendant(Name("p"))).last() {
        process.status = st.text();
    }

    if let Some(st) = document.find(Class("container")).last() {
        let re = Regex::new(r"\s+").unwrap();

        process.info = re.replace_all(&st.text(), " ").trim().to_string();
    }

    Ok(process)
}
