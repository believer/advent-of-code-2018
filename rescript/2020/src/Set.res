type t = {size: int}

@new external make: unit => t = "Set"

@send external add: (t, string) => unit = "add"
@send external forEach: (t, 'a => unit) => unit = "forEach"
