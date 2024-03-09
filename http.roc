app "http"
    packages {
        pf: "platform/main.roc",
    }
    imports [
        pf.Task.{ Task },
        pf.Http,
        pf.Stdout,
    ]
    provides [main] to pf

main : List U8 -> Task Str Str
main = \_ ->
    {} <- Stdout.line "About to make an HTTP request..." |> Task.await
    Http.getUtf8 "https://example.com"
    |> Task.mapErr \_ -> "Error fetching page"
