let f(i: 1..3, j: 1..2): Bool

constraint c = (
    f(1, 1) and not f(1, 2)
)
