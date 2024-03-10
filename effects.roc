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
        pf.File,
        pf.Path,
    ]
    provides [main] to pf

main : List U8 -> Task Str Str
main = \_ ->
    path = Path.fromStr "/tmp/file.txt"
    {} <- File.writeUtf8 path "I'm in a file!"
        |> Task.mapErr \e -> "Error writing to file: $(Inspect.toStr e)"
        |> Task.await
    {} <- Stdout.line "Successfully wrote to file" |> Task.await

    content <- File.readUtf8 path
        |> Task.mapErr \_ -> "Error reading from file"
        |> Task.await
    {} <- Stdout.line "The file content was \"$(content)\"" |> Task.await

    var <- Env.var "SOME_ENV_VAR"
        |> Task.mapErr \_ -> "Env var not found"
        |> Task.await
    {} <- Stdout.line "SOME_ENV_VAR is set to $(var)" |> Task.await

    now <- Utc.now |> Task.await
    {} <- Stdout.line "The time is $(Utc.toIso8601Str now)" |> Task.await

    Http.getUtf8 "https://example.com"
    |> Task.mapErr \_ -> "Error fetching page"

