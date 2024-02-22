app "main"
    packages {
        pf: "platform/main.roc",
        json: "https://github.com/lukewilliamboswell/roc-json/releases/download/0.6.3/_2Dh4Eju2v_tFtZeMq8aZ9qw2outG04NbkmKpFhXS_4.tar.br",
    }
    imports [
        json.Core
    ]
    provides [main] to pf

Request : {
    rawPath: Str
}

main = \bytes ->
    when getRequest bytes is
        Err _ -> "Error deserializing request"
        Ok req -> 
            "Hello $(req.rawPath)!"

getRequest : List U8 -> Result Request _
getRequest = \bytes -> 
    Decode.fromBytes bytes Core.json
    
