
## --- Day 10: Elves Look, Elves Say ---

Today, the Elves are playing a game called [look-and-say][1]. They take turns making sequences by reading aloud the previous sequence and using that reading as the next sequence. For example, `211` is read as
"one two, two ones", which becomes `1221` (`1` `2`, `2` `1`s).

Look-and-say sequences are generated iteratively, using the previous value as input for the next step. For each step, take the previous value, and replace each run of digits (like `111`) with the number of
digits (`3`) followed by the digit itself (`1`).

For example:

* `1` becomes `11` (`1` copy of digit `1`).
* `11` becomes `21` (`2` copies of digit `1`).
* `21` becomes `1211` (one `2` followed by one `1`).
* `1211` becomes `111221` (one `1`, one `2`, and two `1`s).
* `111221` becomes `312211` (three `1`s, two `2`s, and one `1`).

Starting with the digits in your puzzle input, apply this process 40 times. What is *the length of the result*?

Your puzzle answer was `329356`.

## --- Part Two ---

Neat, right? You might also enjoy hearing [John Conway talking about this sequence][2] (that's Conway of *Conway's Game of Life* fame).

Now, starting again with the digits in your puzzle input, apply this process *50* times. What is *the length of the new result*?

Your puzzle answer was `4666278`.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should [return to your Advent calendar][3] and try another puzzle.

Your puzzle input was `3113322113`.

You can also [Shareon [Twitter][4] [Mastodon][5]] this puzzle.

[1] https://en.wikipedia.org/wiki/Look-and-say_sequence
[2] https://www.youtube.com/watch?v=ea7lJkEhytA
[3] /2015
[4] https://twitter.com/intent/tweet?text=I%27ve+completed+%22Elves+Look%2C+Elves+Say%22+%2D+Day+10+%2D+Advent+of+Code+2015&url=https%3A%2F%2Fadventofcode%2Ecom%2F2015%2Fday%2F10&related=ericwastl&hashtags=Ad
ventOfCode
[5] javascript:void(0);

