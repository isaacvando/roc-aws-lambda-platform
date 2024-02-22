platform "roc-aws-lambda"
    requires {} { main : List U8 -> Str }
    exposes []
    packages {}
    imports []
    provides [mainForHost]

mainForHost : List U8 -> Str
mainForHost = \x -> main x
