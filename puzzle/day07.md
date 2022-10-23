
## --- Day 7: Some Assembly Required ---

This year, Santa brought little Bobby Tables a set of wires and [bitwise logic gates][1]! Unfortunately, little Bobby is a little under the recommended age range, and he needs help
assembling the circuit.

Each wire has an identifier (some lowercase letters) and can carry a [16-bit][2] signal (a number from `0` to `65535`). A signal is provided to each wire by a gate, another wire, or some
specific value. Each wire can only get a signal from one source, but can provide its signal to multiple destinations. A gate provides no signal until all of its inputs have a signal.

The included instructions booklet describes how to connect the parts together: `x AND y -> z` means to connect wires `x` and `y` to an AND gate, and then connect its output to wire `z`.

For example:

* `123 -> x` means that the signal `123` is provided to wire `x`.
* `x AND y -> z` means that the [bitwise AND][1] of wire `x` and wire `y` is provided to wire `z`.
  
  [1] https://en.wikipedia.org/wiki/Bitwise_operation#AND
* `p LSHIFT 2 -> q` means that the value from wire `p` is [left-shifted][1] by `2` and then provided to wire `q`.
  
  [1] https://en.wikipedia.org/wiki/Logical_shift
* `NOT e -> f` means that the [bitwise complement][1] of the value from wire `e` is provided to wire `f`.
  
  [1] https://en.wikipedia.org/wiki/Bitwise_operation#NOT

Other possible gates include `OR` ([bitwise OR][3]) and `RSHIFT` ([right-shift][4]). If, for some reason, you'd like to *emulate* the circuit instead, almost all programming languages
(for example, [C][5], [JavaScript][6], or [Python][7]) provide operators for these gates.

For example, here is a simple circuit:

`123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
`

After it is run, these are the signals on the wires:

`d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456
`

In little Bobby's kit's instructions booklet (provided as your puzzle input), what signal is ultimately provided to *wire `a`*?

Your puzzle answer was `16076`.

## --- Part Two ---

Now, take the signal you got on wire `a`, override wire `b` to that signal, and reset the other wires (including wire `a`). What new signal is ultimately provided to wire `a`?

Your puzzle answer was `2797`.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should [return to your Advent calendar][8] and try another puzzle.

If you still want to see it, you can [get your puzzle input][9].

You can also [Shareon [Twitter][10] [Mastodon][11]] this puzzle.

[1] https://en.wikipedia.org/wiki/Bitwise_operation
[2] https://en.wikipedia.org/wiki/16-bit
[3] https://en.wikipedia.org/wiki/Bitwise_operation#OR
[4] https://en.wikipedia.org/wiki/Logical_shift
[5] https://en.wikipedia.org/wiki/Bitwise_operations_in_C
[6] https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_Operators
[7] https://wiki.python.org/moin/BitwiseOperators
[8] /2015
[9] 7/input
[10] https://twitter.com/intent/tweet?text=I%27ve+completed+%22Some+Assembly+Required%22+%2D+Day+7+%2D+Advent+of+Code+2015&url=https%3A%2F%2Fadventofcode%2Ecom%2F2015%2Fday%2F7&related=er
icwastl&hashtags=AdventOfCode
[11] javascript:void(0);

