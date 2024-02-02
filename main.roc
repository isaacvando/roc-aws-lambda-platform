app "rocLovesRust"
    packages { pf: "platform/main.roc" }
    imports []
    provides [main] to pf

main = "It works!\n"
