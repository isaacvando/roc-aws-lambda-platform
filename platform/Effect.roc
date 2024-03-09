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
        envList,
        envVar,
    ]
    imports [
        InternalHttp,
    ]
    generates Effect with [after, map, always, forever, loop]

# Stdout
stdoutLine : Str -> Effect {}

# Http
sendRequest : Box InternalHttp.InternalRequest -> Effect InternalHttp.InternalResponse

# Env
envList : Effect (List (Str, Str))
envVar : Str -> Effect (Result Str {})
