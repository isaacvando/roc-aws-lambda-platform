hosted Effect
    exposes [
        Effect,
        after,
        map,
        always,
        forever,
        loop,
        stdoutLine,
        sendRequest,
    ]
    imports [
        InternalHttp,
    ]
    generates Effect with [after, map, always, forever, loop]

# Stdout
stdoutLine : Str -> Effect {}

# Http
sendRequest : Box InternalHttp.InternalRequest -> Effect InternalHttp.InternalResponse
