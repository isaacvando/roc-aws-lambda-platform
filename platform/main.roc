platform "roc-aws-lambda"
    requires {} { main : List U8 -> Task Str Str }
    exposes []
    packages {}
    imports [Task.{ Task }]
    provides [mainForHost]

mainForHost : List U8 -> Task Str Str
mainForHost = \x -> main x
