interface Env
    exposes [list, var, decode]
    imports [Task.{ Task }, Effect, InternalTask, EnvDecoding]

## Reads the given environment variable.
##
## If the value is invalid Unicode, the invalid parts will be replaced with the
## [Unicode replacement character](https://unicode.org/glossary/#replacement_character) ('�').
var : Str -> Task Str [VarNotFound]
var = \name ->
    Effect.envVar name
    |> Effect.map (\result -> Result.mapErr result \{} -> VarNotFound)
    |> InternalTask.fromEffect

## Reads the given environment variable and attempts to decode it.
##
## The type being decoded into will be determined by type inference. For example,
## if this ends up being used like a `Task U16 _` then the environment variable
## will be decoded as a string representation of a `U16`. Trying to decode into
## any other type will fail with a `DecodeErr`.
##
## Supported types include;
## - Strings,
## - Numbers, as long as they contain only numeric digits, up to one `.`, and an optional `-` at the front for negative numbers, and
## - Comma-separated lists (of either strings or numbers), as long as there are no spaces after the commas.
##
## For example, consider we want to decode the environment variable `NUM_THINGS`;
##
## ```
## # Reads "NUM_THINGS" and decodes into a U16
## getU16Var : Str -> Task U16 [VarNotFound, DecodeErr DecodeError] [Read [Env]]
## getU16Var = \var -> Env.decode var
## ```
##
## If `NUM_THINGS=123` then `getU16Var` succeeds with the value of `123u16`.
## However if `NUM_THINGS=123456789`, then `getU16Var` will
## fail with [DecodeErr](https://www.roc-lang.org/builtins/Decode#DecodeError)
## because `123456789` is too large to fit in a [U16](https://www.roc-lang.org/builtins/Num#U16).
##
decode : Str -> Task val [VarNotFound, DecodeErr DecodeError] where val implements Decoding
decode = \name ->
    Effect.envVar name
    |> Effect.map
        (
            \result ->
                result
                |> Result.mapErr (\{} -> VarNotFound)
                |> Result.try
                    (\varStr ->
                        Decode.fromBytes (Str.toUtf8 varStr) (EnvDecoding.format {})
                        |> Result.mapErr (\_ -> DecodeErr TooShort)))
    |> InternalTask.fromEffect

## Reads all the process's environment variables into a List (Str, Str).
##
## If any key or value contains invalid Unicode, the [Unicode replacement character](https://unicode.org/glossary/#replacement_character)
## will be used in place of any parts of keys or values that are invalid Unicode.
list : Task (List (Str, Str)) *
list =
    Effect.envList
    |> Effect.map Ok
    |> InternalTask.fromEffect
