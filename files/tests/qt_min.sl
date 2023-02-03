struct S {
    x: Int
}
inst a, b, c: S

constraint c0 = forall e: S | (e.x >= 2) and (e.x <= 10) end
constraint c1 = (
    (min e: S | e.x end) >= 5
)

minimize (max e: S | e.x end) until 10