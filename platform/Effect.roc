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
        posixTime,
        fileReadBytes,
        fileDelete,
        fileWriteUtf8,
        fileWriteBytes,
    ]
    imports [
        InternalHttp,
        InternalFile,
    ]
    generates Effect with [after, map, always, forever, loop]

# Stdout
stdoutLine : Str -> Effect {}

# Http
sendRequest : Box InternalHttp.InternalRequest -> Effect InternalHttp.InternalResponse

# Env
envList : Effect (List (Str, Str))
envVar : Str -> Effect (Result Str {})

# Utc
posixTime : Effect U128

# File
fileWriteBytes : List U8, List U8 -> Effect (Result {} InternalFile.WriteErr)
fileWriteUtf8 : List U8, Str -> Effect (Result {} InternalFile.WriteErr)
fileDelete : List U8 -> Effect (Result {} InternalFile.WriteErr)
fileReadBytes : List U8 -> Effect (Result (List U8) InternalFile.ReadErr)
