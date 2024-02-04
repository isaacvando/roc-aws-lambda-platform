app "rocLovesRust"
    packages { pf: "platform/main.roc" }
    imports []
    provides [main] to pf

main = "This string came from Roc!\n"
