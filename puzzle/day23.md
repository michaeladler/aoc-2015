
## --- Day 23: Opening the Turing Lock ---

Little Jane Marie just got her very first computer for Christmas from some unknown benefactor. It comes with instructions and an example program, but the computer itself seems to be malfunctioning. She's curious what the program does, and
would like you to help her run it.

The manual explains that the computer supports two [registers][1] and six [instructions][2] (truly, it goes on to remind the reader, a state-of-the-art technology). The registers are named `a` and `b`, can hold any [non-negative
integer][3], and begin with a value of `0`. The instructions are as follows:

* `hlf r` sets register `r` to *half* its current value, then continues with the next instruction.
* `tpl r` sets register `r` to *triple* its current value, then continues with the next instruction.
* `inc r` *increments* register `r`, adding `1` to it, then continues with the next instruction.
* `jmp offset` is a *jump*; it continues with the instruction `offset` away *relative to itself*.
* `jie r, offset` is like `jmp`, but only jumps if register `r` is *even* ("jump if even").
* `jio r, offset` is like `jmp`, but only jumps if register `r` is `1` ("jump if *one*", not odd).

All three jump instructions work with an *offset* relative to that instruction. The offset is always written with a prefix `+` or `-` to indicate the direction of the jump (forward or backward, respectively). For example, `jmp +1` would
simply continue with the next instruction, while `jmp +0` would continuously jump back to itself forever.

The program exits when it tries to run an instruction beyond the ones defined.

For example, this program sets `a` to `2`, because the `jio` instruction causes it to skip the `tpl` instruction:

`inc a
jio a, +2
tpl a
inc a
`

What is *the value in register `b`* when the program in your puzzle input is finished executing?

Your puzzle answer was `255`.

## --- Part Two ---

The unknown benefactor is *very* thankful for releasi-- er, helping little Jane Marie with her computer. Definitely not to distract you, what is the value in register `b` after the program is finished executing if register `a` starts as
`1` instead?

Your puzzle answer was `334`.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should [return to your Advent calendar][4] and try another puzzle.

If you still want to see it, you can [get your puzzle input][5].

You can also [Shareon [Twitter][6] [Mastodon][7]] this puzzle.

[1] https://en.wikipedia.org/wiki/Processor_register
[2] https://en.wikipedia.org/wiki/Instruction_set
[3] https://en.wikipedia.org/wiki/Natural_number
[4] /2015
[5] 23/input
[6] https://twitter.com/intent/tweet?text=I%27ve+completed+%22Opening+the+Turing+Lock%22+%2D+Day+23+%2D+Advent+of+Code+2015&url=https%3A%2F%2Fadventofcode%2Ecom%2F2015%2Fday%2F23&related=ericwastl&hashtags=AdventOfCode
[7] javascript:void(0);

