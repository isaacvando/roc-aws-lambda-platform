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

main : List U8 -> Str
main = \bytes ->
    when Decode.fromBytes bytes Core.json is
        Err _ -> "I was unable to decode the request into the expected type"
        Ok req ->
            respond req

respond : Request -> Str
respond = \req ->
    serializedHtml =
        body req
        |> Encode.toBytes Core.json
        |> Str.fromUtf8
        |> Result.withDefault "Utf8 Error"
    """
    {"statusCode": 200,"headers":{"Content-Type": "text/html"},"body":$(serializedHtml)}
    """

body : Request -> Str
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
