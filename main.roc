app "main"
    packages {
        pf: "platform/main.roc",
        json: "https://github.com/lukewilliamboswell/roc-json/releases/download/0.6.3/_2Dh4Eju2v_tFtZeMq8aZ9qw2outG04NbkmKpFhXS_4.tar.br",
    }
    imports [
        json.Core,
    ]
    provides [main] to pf

Request : {
    rawPath : Str,
}

main = \bytes ->
    when getRequest bytes is
        Err _ -> "Error deserializing request"
        Ok req ->
            response req

getRequest : List U8 -> Result Request _
getRequest = \bytes ->
    Decode.fromBytes bytes Core.json

response = \req ->
    bodyStr =
        body req
        |> Encode.toBytes Core.json
        |> Str.fromUtf8
        |> Result.withDefault "Utf8 Error"
    """
    {"statusCode": 200,"headers":{"Content-Type": "text/html"},"body":$(bodyStr)}
    """

body = \req ->
    name =
        when Str.split req.rawPath "/" is
            [_, n, ..] if !(Str.isEmpty n) -> n
            _ -> "world"
    """
    <!DOCTYPE html>
    <html>
    <head>
    <title>Roc on AWS Lambda</title>
    </head>

    <body>
    <div>
        <h1>roc-aws-lambda</h1>
        <p>Hello, $(name)!</p>
        <p>Try updating the part of the url after .aws</p>
    </div>

    </body>
    </html>
    """
