interface Stdout
    exposes [line]
    imports [Effect, Task.{ Task }, InternalTask]

## Write the given string to [standard output](https://en.wikipedia.org/wiki/Standard_streams#Standard_output_(stdout)),
## followed by a newline.
##
line : Str -> Task {} *
line = \str ->
    Effect.stdoutLine str
    |> Effect.map (\_ -> Ok {})
    |> InternalTask.fromEffect

