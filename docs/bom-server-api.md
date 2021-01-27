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

