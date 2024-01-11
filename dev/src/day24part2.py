#!/usr/bin/env python3

# taken from AoC reddit comment by mayoff

# pip install z3-solver
from z3 import *

rx, ry, rz = Ints('rx ry rz')
rvx, rvy, rvz = Ints('rvx rvy rvz')
t0, t1, t2 = Ints('t0 t1 t2')
answer = Int('answer')

solve(
    rx + t0 * rvx == 296136747977213 + t0 * 88,
    ry + t0 * rvy == 400026919462961 + t0 * 359,
    rz + t0 * rvz == 245942583851044 + t0 * -31,

    rx + t1 * rvx == 135797317464983 + t1 * 138,
    ry + t1 * rvy == 392120809901003 + t1 * -222,
    rz + t1 * rvz == 313062084315250 + t1 * 6,

    rx + t2 * rvx == 176557441160429 + t2 * 88,
    ry + t2 * rvy == 143347387408157 + t2 * 58,
    rz + t2 * rvz == 61882073031568 + t2 * 292,

    answer == rx + ry + rz,
)
