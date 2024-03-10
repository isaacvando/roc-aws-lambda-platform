app "webpage"
    packages {
        pf: "platform/main.roc",
    }
    imports [
        pf.Task.{ Task },
    ]
    provides [main] to pf

main : List U8 -> Task Str Str
main = \_ -> Task.ok "Hello, World!"
