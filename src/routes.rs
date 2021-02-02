use std::convert::TryInto;

use rocket::{http::RawStr, State};
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid as RocketUuid;
use uuid::Uuid;

use crate::errors::PartsErrorCode;
use crate::parts_list::{Part, PartsList, PartsListFilter};
use crate::query::{NewPart, UpdateChildren};
use crate::response::Response;
use crate::SharedPartsList;

#[get("/")]
pub fn index() -> &'static str {
    r####"# BOM Server API
The `bom-server` API exposes a simple REST API to allow for management of BOM parts.

## API Overview
The following APIs can be used to interact with the BOM Server:

```
GET     /v1/parts?filter=<all|top_level|assembly|component|subassembly|orphan> -> list all parts
POST    /v1/parts                                                -> create a new part
GET     /v1/parts/<id>                                           -> get part <id> information
DELETE  /v1/parts/<id>                                           -> delete part <id> from server
GET     /v1/parts/<id>/children?filter=<all|component|top_level> -> get children of part <id>
POST    /v1/parts/<id>/children?action=<add|remove|replace>      -> update children of part <id>
GET     /v1/parts/<id>/contained -> get assemblies that include part <id> directly or indirectly
```

## Responses
Each query to a valid API on the server returns a response object in JSON format the body of the reply.

### Response Body
Each field is optional and should be checked for `null` before referencing values. The `error` field
will be set only if an error occurred as a result of the requested operation.  Otherwise, `result`
and `data` should be populated as shown below.

```
{
    "result": {
        "code": <int>,
        "description": "<Result information String>"
    },
    "data": [
        {
            "id": "<UUID String>",
            "name": "<part name>",
            "parents" : [ "<UUID String>", ... ],
            "children" : [ "<UUID String>", ... ]
        },
        ...
    ]
    "error": {
        "code": <int>,
        "description": "<Error description String>"
    }
}
```

## Requests
Each POST command requires a properly formatted JSON object in the request body.

### New Part Request Body
To request creation of a part, supply a unique name for the part as follows:

```
{
    "name": "<name of the part>"
}
```

### Update Children Request Body
To request updates to the children of a part, supply the child identifiers for the operation as follows:

```
{
    "children": ["<child part id1>", "<child part id2>", ... ]
}
```

"####
}

#[get("/v1/parts?<filter>")]
pub fn list_parts(filter: Option<&RawStr>, parts: State<SharedPartsList>) -> Json<Response> {
    let response = Response::new();
    match filter
        .unwrap_or(RawStr::from_str("all"))
        .as_str()
        .try_into()
    {
        Ok(filter) => {
            if let Ok(parts) = parts.0.try_read() {
                let parts: &PartsList = &parts;
                let list = parts.list(filter).into_iter().cloned().collect();
                Json(
                    response
                        .result(200, "Fetched all parts successfully")
                        .data(list),
                )
            } else {
                Json(response.error(PartsErrorCode::LockError, "Couldn't read lock parts list!"))
            }
        }
        Err(e) => Json(response.error(
            PartsErrorCode::RequestError,
            &format!("Invalid filter type passed: {}", e),
        )),
    }
}

#[post("/v1/parts", format = "json", data = "<data>")]
pub fn create_part(data: Json<NewPart>, parts: State<SharedPartsList>) -> Json<Response> {
    let response = Response::new();
    if let Ok(mut parts) = parts.0.try_write() {
        let part = Part::new(&data.name);
        match parts.add(part) {
            Ok(part) => Json(
                response
                    .result(201, "New part created successfully")
                    .data(vec![part.clone()]),
            ),
            Err(e) => Json(response.error(PartsErrorCode::CreatePartError, &format!("{}", e))),
        }
    } else {
        Json(response.error(PartsErrorCode::LockError, "Couldn't write lock parts list!"))
    }
}

#[get("/v1/parts/<part_id>")]
pub fn get_part(part_id: RocketUuid, parts: State<SharedPartsList>) -> Json<Response> {
    let response = Response::new();
    let part_id = Uuid::from_bytes(part_id.as_bytes().clone());
    if let Ok(parts) = parts.0.try_read() {
        match parts.get(&part_id) {
            Ok(part) => Json(
                response
                    .result(200, "Found part in parts list")
                    .data(vec![part.clone()]),
            ),
            Err(e) => Json(response.error(PartsErrorCode::MissingPartError, &format!("{}", e))),
        }
    } else {
        Json(response.error(PartsErrorCode::LockError, "Couldn't read lock parts list!"))
    }
}

#[delete("/v1/parts/<part_id>")]
pub fn delete_part(part_id: RocketUuid, parts: State<SharedPartsList>) -> Json<Response> {
    let response = Response::new();
    let part_id = Uuid::from_bytes(part_id.as_bytes().clone());
    if let Ok(mut parts) = parts.0.try_write() {
        match parts.delete(&part_id) {
            Ok(_) => Json(response.result(200, "Deleted part from list")),
            Err(e) => Json(response.error(PartsErrorCode::MissingPartError, &format!("{}", e))),
        }
    } else {
        Json(response.error(PartsErrorCode::LockError, "Couldn't write lock parts list!"))
    }
}

#[get("/v1/parts/<part_id>/children?<filter>")]
pub fn get_children(
    part_id: RocketUuid,
    filter: Option<&RawStr>,
    parts: State<SharedPartsList>,
) -> Json<Response> {
    let response = Response::new();
    let part_id = Uuid::from_bytes(part_id.as_bytes().clone());
    match filter
        .unwrap_or(RawStr::from_str("all"))
        .as_str()
        .try_into()
    {
        Ok(filter) => match filter {
            PartsListFilter::All | PartsListFilter::Component | PartsListFilter::TopLevel => {
                if let Ok(parts) = parts.0.try_read() {
                    let parts: &PartsList = &parts;
                    match parts.get_children(&part_id, filter) {
                        Ok(children) => {
                            let children = children.into_iter().cloned().collect();
                            Json(
                                response
                                    .result(200, "Fetched all parts successfully")
                                    .data(children),
                            )
                        }
                        Err(e) => Json(
                            response.error(PartsErrorCode::MissingPartError, &format!("{}", e)),
                        ),
                    }
                } else {
                    Json(
                        response.error(PartsErrorCode::LockError, "Couldn't read lock parts list!"),
                    )
                }
            }
            _ => Json(response.error(
                PartsErrorCode::RequestError,
                "Unsupported filter on children, only all, top_level and component are supported",
            )),
        },
        Err(e) => Json(response.error(
            PartsErrorCode::RequestError,
            &format!("Invalid filter type passed: {}", e),
        )),
    }
}

#[post(
    "/v1/parts/<part_id>/children?<action>",
    format = "json",
    data = "<data>"
)]
pub fn update_children(
    part_id: RocketUuid,
    action: Option<&RawStr>,
    data: Json<UpdateChildren>,
    parts: State<SharedPartsList>,
) -> Json<Response> {
    let response = Response::new();
    let part_id = Uuid::from_bytes(part_id.as_bytes().clone());
    match action
        .unwrap_or(RawStr::from_str("add"))
        .as_str()
        .try_into()
    {
        Ok(action) => {
            if let Ok(mut parts) = parts.0.try_write() {
                match parts.update(
                    &part_id,
                    &data.children.iter().collect::<Vec<&Uuid>>(),
                    action,
                ) {
                    Ok(_) => Json(response.result(200, "Part children updated successfully")),
                    Err(e) => {
                        Json(response.error(PartsErrorCode::CreatePartError, &format!("{}", e)))
                    }
                }
            } else {
                Json(response.error(PartsErrorCode::LockError, "Couldn't write lock parts list!"))
            }
        }
        Err(e) => Json(response.error(
            PartsErrorCode::RequestError,
            &format!("Invalid action type passed: {}", e),
        )),
    }
}

#[get("/v1/parts/<part_id>/contained")]
pub fn get_contained(part_id: RocketUuid, parts: State<SharedPartsList>) -> Json<Response> {
    let response = Response::new();
    let part_id = Uuid::from_bytes(part_id.as_bytes().clone());
    if let Ok(parts) = parts.0.try_read() {
        match parts.get_children(&part_id, PartsListFilter::Assembly) {
            Ok(children) => {
                let children = children.into_iter().cloned().collect();
                Json(
                    response
                        .result(200, "Fetched all parts successfully")
                        .data(children),
                )
            }
            Err(e) => Json(response.error(PartsErrorCode::MissingPartError, &format!("{}", e))),
        }
    } else {
        Json(response.error(PartsErrorCode::LockError, "Couldn't read lock parts list!"))
    }
}
