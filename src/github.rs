use std::fmt::{Debug, Display, Formatter};
use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_string_from_number;
use crate::conf;

/******************************************************************************
This is a subset of the resulting structure returned by the GitHub notification
API. This must be defined in order for the reqwest library to parse the JSON
response into an object type that is useful in rust.
******************************************************************************/
#[derive(Debug, Clone, Deserialize)]
pub struct Repository {
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub id: String,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Subject {
    pub title: String,

    #[serde(rename = "type")]
    pub category: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct Notification {
    pub id: String,
    pub repository: Repository,
    pub subject: Subject
}

/******************************************************************************
This defines a type that can be used as an Error return type. Typically you'd
use a crate like `thiserror` or `snafu` to derive these definitions from a few
attributes. The code is expanded out here to show what is really happening in
those derive macros.
******************************************************************************/
#[derive(Debug, Clone)]
pub struct RequestError {
    message: String
}

impl RequestError {
    fn new(msg: &str) -> Self {
        Self {
            message: msg.to_owned()
        }
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RequestError: {}", self.message)
    }
}

impl From<InvalidHeaderValue> for RequestError {
    fn from(value: InvalidHeaderValue) -> Self {
        RequestError { message: format!("{value}") }
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(value: reqwest::Error) -> Self {
        RequestError { message: format!("{value}") }
    }
}

/******************************************************************************
These are the API functions that we want to call from the main loop.
******************************************************************************/
fn create_headers() -> Result<HeaderMap, RequestError> {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_str("application/vnd.github+json")?);
    headers.insert("X-GitHub-Api-Version", HeaderValue::from_str("2022-11-28")?);
    headers.insert("User-Agent", HeaderValue::from_str("Danbot")?);

    let auth_value = format!("Bearer {}", conf::get_conf(false).github_token());
    headers.insert("Authorization", HeaderValue::from_str(&auth_value)?);

    Ok(headers)
}

pub async fn list_notifications() -> Result<Vec<Notification>, RequestError> {
    let client = reqwest::Client::new();
    let res = client.get("https://api.github.com/notifications")
        .headers(create_headers()?)
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}

pub async fn mark_read(_thread_id: String) -> Result<(), RequestError> {
    Ok(())
    // let client = reqwest::Client::new();
    // let res = client.patch(format!("https://api.github.com/notifications/threads/{thread_id}"))
    //     .headers(create_headers()?)
    //     .send()
    //     .await?;
    //
    // if res.status().is_success() {
    //     Ok(())
    // } else {
    //     Err(RequestError::new("failed"))
    // }
}