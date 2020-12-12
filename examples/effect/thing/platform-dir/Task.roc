interface Task
    exposes [ Task, after, always, fail, map, putLine, getLine ]
    imports [ Effect ]

Task a err : Effect.Effect (Result a err)

always : a -> Task a *
always = \x -> Effect.always (Ok x)

fail : err -> Task * err
fail = \x -> Effect.always (Err x)

getLine : Task Str *
getLine = Effect.after Effect.getLine always

putLine : Str -> Task {} *
putLine = \line -> Effect.map (Effect.putLine line) (\_ -> Ok {})

map : Task a err, (a -> b) -> Task b err
map = \task, transform ->
    Effect.map task \res ->
        when res is
            Ok x -> Ok (transform x)
            Err e -> Err e

after : Task a err, (a -> Task b err) -> Task b err
after = \task, transform ->
    Effect.after task \res ->
        when res is
            Ok x -> transform x
            Err e -> Task.fail e