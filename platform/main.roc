platform "roc-aws-lambda"
    requires {} { main : U8 -> Str }
    exposes []
    packages {}
    imports []
    provides [mainForHost]

mainForHost : U8 -> Str
mainForHost = \x -> main x
