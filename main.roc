app "rocLovesRust"
    packages { pf: "platform/main.roc" }
    imports []
    provides [main] to pf

main = "Roc <3 Rust!\n"
