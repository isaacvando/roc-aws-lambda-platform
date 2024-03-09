app "http"
    packages {
        pf: "platform/main.roc",
    }
    imports [
        pf.Task.{ Task },
        pf.Http,
        pf.Stdout,
        pf.Env,
    ]
    provides [main] to pf

main : List U8 -> Task Str Str
main = \_ ->
    var <- Env.var "SOME_VAR"
        |> Task.mapErr \_ -> "Couldn't find env var"
        |> Task.await
    {} <- Stdout.line "SOME_VAR = $(var)" |> Task.await
    Http.getUtf8 "https://example.com"
    |> Task.mapErr \_ -> "Error fetching page"
