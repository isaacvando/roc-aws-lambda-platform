# roc-aws-lambda-platform

See instead [roc-aws-lambda](https://github.com/isaacvando/roc-aws-lambda) for a solution using Basic CLI.

---

A platform that provides support for writing AWS Lambda Functions using Roc. It can only be used on Linux currently.

## Installing

You will need a recent version of Roc and the [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime) installed to compile a Lambda and an AWS account to deploy it in :rocket:.

## Examples

Each function accepts a `List U8` as an argument and returns a `Task Str Str`. The input will be JSON and the output must be also. If the platform cannot convert the output string into a valid JSON value, it treat it as a JSON string containing the value returned by the Roc app.

Hello world looks like this:

```roc
# hello.roc

main : List U8 -> Task Str Str
main = \_ -> Task.ok "Hello, World!"
```

These are some of the effects the platform supports:

```roc
# effects.roc

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
```

This platform can be used to write a function that serves a web app from a [Lambda URL](https://docs.aws.amazon.com/lambda/latest/dg/urls-configuration.html):

```roc
# webpage.roc

# This type mirrors the JSON that is passed into a Lambda when it is used with an API Gateway or Function URL.
# For our purposes, we only need the rawPath field, but we could add more as needed.
Request : {
    rawPath : Str,
}

main : List U8 -> Task Str Str
main = \bytes ->
    when Decode.fromBytes bytes Core.json is
        Err _ -> Task.err "I was unable to decode the request into the expected type"
        Ok req ->
            respond req |> Task.ok

# Generate the JSON response Lambda expects for rendering a web page
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

# Generate the HTML body to be rendered when a user visits the Function URL for this Lambda
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
```

## Compiling

To compile a function, use the included build script:

```bash
$ ./build.sh yourfunction.roc
```

Then you can deploy the Lambda to your AWS account with the deployment script:

```bash
$ ./deploy.sh yourfunction arn:aws:iam::{your_account_id}:role/{your_role_name}
```

or you can manually upload the `bootstrap.zip` output by `build.sh` to the AWS Console.

## Contributing

PRs are very welcome!

## TODO

It would be great to have all of these features eventually:

- [ ] The ability to compile Lambdas from MacOS.
- [ ] A more robust build process.
- [ ] A more robust deployment process.
- [ ] The ability to use Lambda functions with the local development server provided by the Rust runtime.
