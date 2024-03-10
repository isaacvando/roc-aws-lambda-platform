platform "roc-aws-lambda"
    requires {} { main : List U8 -> Task Str Str }
    exposes [
        Path,
        Env,
        File,
        FileMetadata,
        Http,
        Stdout,
        Task,
        Url,
        Utc,
    ]
    packages {}
    imports [Task.{ Task }]
    provides [mainForHost]

mainForHost : List U8 -> Task Str Str
mainForHost = \x -> main x
