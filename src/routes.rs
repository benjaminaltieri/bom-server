use std::convert::TryInto;

use rocket::{State, http::RawStr};
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid as RocketUuid;
use uuid::Uuid;

use crate::errors::{PartsErrorCode};
use crate::parts_list::{Part, PartsList, PartsListFilter};
use crate::query::{NewPart,  UpdateChildren};
use crate::response::Response;
use crate::SharedPartsList;


#[get("/")]
pub fn index() -> &'static str {
r#"BOM-Server

API v1
------
Use the following APIs to interact with the BOM Server:

GET     /v1/parts      -> list all parts
POST    /v1/parts      -> create a new part
GET     /v1/parts/<id> -> get part <id> information
DELETE  /v1/parts/<id> -> delete part <id> from server
GET     /v1/parts/<id>/children -> get children of part <id>
POST    /v1/parts/<id>/children -> update children of part <id>
GET     /v1/parts/<id>/contained -> get assemblies that include part <id> directly or indirectly

New Part Request Body:
{
    name: "Name of the part"
}

Update Children Request Body:
{
    parts: ["child part id1", "child part id2", ... ]
}

Response Body:
{
    result: {
      code: int,
      description: "Result information"
    },
    data: [{name: "part id1", id: ###, {name: "part id2", id: ###}, ... ],
    error: {
      code: int,
      description: "Error description"
    }
}
Fields above are optional and should be checked before referencing values.

"#
}


#[get("/v1/parts?<filter>")]
pub fn list_parts(filter: &RawStr,
                  parts: State<SharedPartsList>) -> Json<Response> {
    let response = Response::new();
    match filter.as_str().try_into() {
        Ok(filter) => {
            if let Ok(parts) = parts.0.try_read() {
                let parts: &PartsList = &parts;
                let list = parts.list(filter).into_iter().cloned().collect();
                Json(response.result(200,
                                    "Fetched all parts successfully")
                             .data(list))
            } else {
                Json(response.error(PartsErrorCode::LockError,
                                    "Couldn't read lock parts list!"))
            }
        }
        Err(e) => {
            Json(response.error(PartsErrorCode::RequestError,
                                &format!("Invalid filter type passed: {}", e)))
        }
    }
}

#[post("/v1/parts", format = "json", data = "<data>")]
pub fn create_part(data: Json<NewPart>,
                   parts: State<SharedPartsList>) -> Json<Response> {
    let response = Response::new();
    if let Ok(mut parts) = parts.0.try_write() {
        let part = Part::new(&data.name);
        match parts.add(part) {
            Ok(part) =>
                Json(response.result(201,
                                    "New part created successfully")
                             .data(vec![part.clone()])),
            Err(e) =>
                Json(response.error(PartsErrorCode::CreatePartError,
                                    &format!("{}", e)))
        }
    } else {
        Json(response.error(PartsErrorCode::LockError,
                            "Couldn't write lock parts list!"))
    }
}

#[get("/v1/parts/<part_id>")]
pub fn get_part(part_id: RocketUuid,
                parts: State<SharedPartsList>) -> Json<Response> {
    let response = Response::new();
    let part_id = Uuid::from_bytes(part_id.as_bytes().clone());
    if let Ok(parts) = parts.0.try_read() {
        match parts.get(&part_id) {
            Ok(part) => Json(response.result(200,
                                              "Found part in parts list")
                                       .data(vec![part.clone()])),
            Err(e) => Json(response.error(PartsErrorCode::MissingPartError,
                                          &format!("{}", e)))
        }
    } else {
        Json(response.error(PartsErrorCode::LockError,
                            "Couldn't read lock parts list!"))
    }
}

#[delete("/v1/parts/<part_id>")]
pub fn delete_part(part_id: RocketUuid,
                   parts: State<SharedPartsList>) -> Json<Response> {
    let response = Response::new();
    let part_id = Uuid::from_bytes(part_id.as_bytes().clone());
    if let Ok(mut parts) = parts.0.try_write() {
        match parts.delete(&part_id) {
            Ok(_) => Json(response.result(200,
                                          "Deleted part from list")),
            Err(e) => Json(response.error(PartsErrorCode::MissingPartError,
                                          &format!("{}", e)))
        }
    } else {
        Json(response.error(PartsErrorCode::LockError,
                            "Couldn't write lock parts list!"))
    }
}

#[get("/v1/parts/<part_id>/children?<filter>")]
pub fn get_children(part_id: RocketUuid,
                    filter: &RawStr,
                    parts: State<SharedPartsList>) -> Json<Response> {
    let response = Response::new();
    let part_id = Uuid::from_bytes(part_id.as_bytes().clone());
    match filter.as_str().try_into() {
        Ok(filter) => {
            match filter {
                PartsListFilter::All | PartsListFilter::Component | PartsListFilter::TopLevel => {
                    if let Ok(parts) = parts.0.try_read() {
                        let parts: &PartsList = &parts;
                        match parts.get_children(&part_id, filter) {
                            Ok(children) => {
                                let children = children.into_iter().cloned().collect();
                                Json(response.result(200,
                                                     "Fetched all parts successfully")
                                             .data(children))
                            }
                            Err(e) => Json(response.error(PartsErrorCode::MissingPartError,
                                                          &format!("{}", e)))
                        }
                    } else {
                        Json(response.error(PartsErrorCode::LockError,
                                            "Couldn't read lock parts list!"))
                    }
                }
                _ => Json(response.error(PartsErrorCode::RequestError,
                                         "Unsupported filter on children, only all, top_level and component are supported"))
            }
        }
        Err(e) => {
            Json(response.error(PartsErrorCode::RequestError,
                                &format!("Invalid filter type passed: {}", e)))
        }
    }
}

#[post("/v1/parts/<part_id>/children?<action>", format = "json", data = "<data>")]
pub fn update_children(part_id: RocketUuid,
                       action:  &RawStr,
                       data: Json<UpdateChildren>,
                       parts: State<SharedPartsList>) -> Json<Response> {
    let response = Response::new();
    let part_id = Uuid::from_bytes(part_id.as_bytes().clone());
    match action.as_str().try_into() {
        Ok(action) => {
            if let Ok(mut parts) = parts.0.try_write() {
                match parts.update(&part_id, &data.children.iter().collect::<Vec<&Uuid>>(), action) {
                    Ok(_) =>
                        Json(response.result(200,
                                            "Part children updated successfully")),
                    Err(e) =>
                        Json(response.error(PartsErrorCode::CreatePartError,
                                            &format!("{}", e)))
                }
            } else {
                Json(response.error(PartsErrorCode::LockError,
                                    "Couldn't write lock parts list!"))
            }
        }
        Err(e) => {
            Json(response.error(PartsErrorCode::RequestError,
                                &format!("Invalid action type passed: {}", e)))
        }
    }
}

#[get("/v1/parts/<part_id>/contained")]
pub fn get_contained(part_id: RocketUuid,
                     parts: State<SharedPartsList>) -> Json<Response> {
    let response = Response::new();
    let part_id = Uuid::from_bytes(part_id.as_bytes().clone());
    if let Ok(parts) = parts.0.try_read() {
        match parts.get_children(&part_id, PartsListFilter::Assembly) {
            Ok(children) => {
                let children = children.into_iter().cloned().collect();
                Json(response.result(200,
                                     "Fetched all parts successfully")
                              .data(children))
            },
            Err(e) => Json(response.error(PartsErrorCode::MissingPartError,
                                          &format!("{}", e)))
        }
    } else {
        Json(response.error(PartsErrorCode::LockError,
                            "Couldn't read lock parts list!"))
    }
}

