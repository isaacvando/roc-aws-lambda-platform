app "main"
    packages { pf: "platform/main.roc" }
    imports []
    provides [main] to pf

main = \req -> 
    reqStr = when Str.fromUtf8 req is
        Ok r -> r
        Err _ -> crash "utf problem"
    "request: $(reqStr)"
