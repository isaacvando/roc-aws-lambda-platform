app "main"
    packages { pf: "platform/main.roc" }
    imports []
    provides [main] to pf

main = \num -> "Number passed in: $(Num.toStr num)"
