# BOM Server API
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

### List Parts - `GET /v1/parts?filter=<all|top_level|assembly|component|subassembly|orphan>`
A request to this uri will return the list of parts held in the server.

There are several filter options which map to the following:
```
all - list all parts
assemblies - list all assemblies (any part that contains other parts)
top_level - list all top level assemblies (assemblies that are not children of another assembly)
subassemblies - list all subassemblies (assemblies that are children of another assembly)
component - list all component parts (parts that are not subassemblies, but are included in a parent assembly)
orphan - list all orphan parts (parts with neither parents nor children)
```
The default value when `filter` is not specified is `all`.

### Create Part - `POST /v1/parts`
A request to this uri along with a New Part Request Body, will create a new part in the server with the specified unique name.

### Get Part - `GET /v1/parts/<id>`
A request to this uri will return `<id>`.

### Delete Part - `DELETE /v1/parts/<id>`
A request to this uri will delete `<id>` and remove it from the children of other parts that contained it.

### Get Children - `GET /v1/parts/<id>/children?filter=<all|component|top_level>`
A request to this uri will return the children of `<id>`, optionally filtered to specific types.

The options are similar to those for List Parts but limited to the following:
```
all - list all parts
top_level - list all top level assemblies (assemblies that are not children of another assembly)
component - list all component parts (parts that are not subassemblies, but are included in a parent assembly)
```
The default value when `filter` is not specified is `all`.

### Update Children - `POST /v1/parts/<id>/children?action=<add|remove|replace>`
A request to this uri along with a Update Children Request Body, will update the children of a part.

An action can be specified to determine how to apply the update with the supplied part ids:
```
add - add parts listed to `<id>`
remove - remove parts listed from `<id>`
replace - replace children of `<id>` with parts listed
```
The default value when `action` is not specified is `all`.

### Get Contained - `GET /v1/parts/<id>/contained`
A request to this uri will return all assemblies that contain `<id>`, either directly or indirectly.

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

