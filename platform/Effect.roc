hosted Effect
    exposes [
        Effect,
        after,
        map,
        always,
        forever,
        loop,
        sendRequest,
    ]
    imports [
        InternalHttp,
    ]
    generates Effect with [after, map, always, forever, loop]

# Http
sendRequest : Box InternalHttp.InternalRequest -> Effect InternalHttp.InternalResponse
