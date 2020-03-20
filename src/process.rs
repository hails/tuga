use diesel::prelude::*;
use regex::Regex;
use reqwest;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::error;

use crate::database;
use crate::models::*;
use crate::schema::processes::dsl::*;

#[derive(Debug)]
pub struct ProcessResponse {
    pub status: String,
    pub info: String,
}

pub async fn start(
    telegram_id: String,
    process_code: String,
) -> Result<ProcessResponse, Box<dyn error::Error>> {
    let p = fetch_for_one(&process_code).await?;
    save(&telegram_id, &process_code, &p.status).await;

    Ok(p)
}

async fn fetch_for_one(process_code: &str) -> Result<ProcessResponse, Box<dyn error::Error>> {
    Ok(fetch_citizenship_status(&process_code).await?)
}

async fn save(telegram_id: &str, process_code: &str, process_status: &str) {
    let connection = database::establish_connection();
    diesel::insert_into(processes)
        .values((
            telegram_user_id.eq(telegram_id),
            code.eq(process_code),
            status.eq(process_status.to_lowercase()),
        ))
        .on_conflict((telegram_user_id, code))
        .do_update()
        .set(status.eq(process_status.to_lowercase()))
        .execute(&connection)
        .expect("error while inserting process");
}

pub async fn fetch_for_all() -> Result<Vec<(String, ProcessResponse)>, Box<dyn error::Error>> {
    let connection = database::establish_connection();
    let user_processes = processes
        .filter(status.ne("finished"))
        .load::<Process>(&connection)
        .expect("Error loading processes");

    let mut updated_processes = Vec::new();
    for process in user_processes {
        let process_response = fetch_for_one(&process.code).await?;
        if process_response.status.to_lowercase() != process.status {
            save(
                &process.telegram_user_id,
                &process.code,
                &process_response.status,
            )
            .await;
            updated_processes.push((process.telegram_user_id, process_response));
        }
    }

    Ok(updated_processes)
}

async fn fetch_citizenship_status(process_code: &str) -> Result<ProcessResponse, reqwest::Error> {
    let res = reqwest::Client::new()
        .post("https://nacionalidade.justica.gov.pt/Home/GetEstadoProcessoAjax")
        .form(&[("SenhaAcesso", process_code)])
        .send()
        .await?;
    let body = res.text().await?;

    let document = Document::from(body.as_str());

    let mut p = ProcessResponse {
        status: String::from("unknown"),
        info: String::from(""),
    };

    if let Some(st) = document.find(Class("active1").descendant(Name("p"))).last() {
        p.status = st.text();
    }

    if let Some(st) = document.find(Class("active2").descendant(Name("p"))).last() {
        p.status = st.text();
    }

    if let Some(st) = document.find(Class("active3").descendant(Name("p"))).last() {
        p.status = st.text();
    }

    if let Some(st) = document.find(Class("container")).last() {
        let re = Regex::new(r"\s+").unwrap();

        p.info = re.replace_all(&st.text(), " ").trim().to_string();
    }

    Ok(p)
}
