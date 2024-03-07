app "http"
    packages {
        pf: "platform/main.roc",
    }
    imports [
        pf.Task.{ Task },
        pf.Http,
    ]
    provides [main] to pf

main : List U8 -> Task Str Str
main = \_ ->
    Http.getUtf8 "https://example.com"
    |> Task.mapErr \_ -> "Error fetching page"
