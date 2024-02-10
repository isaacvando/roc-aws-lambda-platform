app "main"
    packages { pf: "platform/main.roc" }
    imports []
    provides [main] to pf

main = "A roc string!"
