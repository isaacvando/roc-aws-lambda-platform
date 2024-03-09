app "effects"
    packages {
        pf: "platform/main.roc",
    }
    imports [
        pf.Task.{ Task },
        pf.Http,
        pf.Stdout,
        pf.Env,
        pf.Utc,
    ]
    provides [main] to pf

main : List U8 -> Task Str Str
main = \_ ->
    now <- Utc.now |> Task.await

    var <- Env.var "SOME_ENV_VAR"
        |> Task.mapErr \_ -> "Env var not found"
        |> Task.await

    {} <- Stdout.line "SOME_ENV_VAR is set to $(var)\nThe time is $(now |> Utc.toIso8601Str)" |> Task.await

    Http.getUtf8 "https://example.com"
    |> Task.mapErr \_ -> "Error fetching page"
