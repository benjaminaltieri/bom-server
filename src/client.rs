use reqwest::Client;
use url::Url;
use uuid::Uuid;

use crate::parts_list::{PartsListFilter, PartsListUpdate};
use crate::query;
use crate::response::Response;

pub struct ClientContext {
    pub client: Client,
    pub base_url: Url,
}

impl ClientContext {
    pub fn new(base_url: Url) -> ClientContext {
        ClientContext { client: Client::new(),  base_url: base_url }
    }
}

pub async fn get_index(context: &ClientContext) -> anyhow::Result<String> {
    Ok(reqwest::get(context.base_url.join("/")?)
        .await?
        .text()
        .await?)
}

pub async fn list_parts(context: &ClientContext, filter: PartsListFilter) -> anyhow::Result<Response> {
    let request_uri: String = format!("/v1/parts?filter={}", Into::<&str>::into(filter));
    Ok(reqwest::get(context.base_url.join(&request_uri)?)
        .await?
        .json::<Response>()
        .await?)
}

pub async fn create_part(context: &ClientContext, name: &str) -> anyhow::Result<Response> {
    let uri_path = "/v1/parts";
    let request_url = context.base_url.join(&uri_path)?;
    Ok(context.client.post(request_url)
        .json(&query::NewPart{name: name.into()})
        .send()
        .await?
        .json::<Response>()
        .await?)
}

pub async fn get_part(context: &ClientContext, id: &Uuid) -> anyhow::Result<Response> {
    let uri_path: String = format!("/v1/parts/{}", id);
    Ok(reqwest::get(context.base_url.join(&uri_path)?)
        .await?
        .json::<Response>()
        .await?)
}

pub async fn delete_part(context: &ClientContext, id: &Uuid) -> anyhow::Result<Response> {
    let uri_path: String = format!("/v1/parts/{}", id);
    let request_url = context.base_url.join(&uri_path)?;
    Ok(context.client.delete(request_url)
                     .send()
                     .await?
                     .json::<Response>()
                     .await?)
}

pub async fn get_children(context: &ClientContext, id: &Uuid, filter: PartsListFilter) -> anyhow::Result<Response> {
    let uri_path: String = format!("/v1/parts/{}/children?filter={}", id, Into::<&str>::into(filter));
    Ok(reqwest::get(context.base_url.join(&uri_path)?)
        .await?
        .json::<Response>()
        .await?)
}

pub async fn update_part(context: &ClientContext, id: &Uuid, children: &[Uuid], action: PartsListUpdate) -> anyhow::Result<Response> {
    let uri_path = format!("/v1/parts/{}/children?action={}", id, Into::<&str>::into(action));
    let request_url = context.base_url.join(&uri_path)?;
    Ok(context.client.post(request_url)
        .json(&query::UpdateChildren{children: children.iter().copied().collect()})
        .send()
        .await?
        .json::<Response>()
        .await?)
}

pub async fn get_contained(context: &ClientContext, id: &Uuid) -> anyhow::Result<Response> {
    let uri_path: String = format!("/v1/parts/{}/contained", id);
    Ok(reqwest::get(context.base_url.join(&uri_path)?)
        .await?
        .json::<Response>()
        .await?)
}

