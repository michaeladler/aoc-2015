
## --- Day 1: Not Quite Lisp ---

Santa was hoping for a white Christmas, but his weather machine's "snow" function is powered by stars, and he's fresh
out! To save Christmas, he needs you to collect *fifty stars* by December 25th.

Collect stars by helping Santa solve puzzles. Two puzzles will be made available on each day in the Advent calendar;
the second puzzle is unlocked when you complete the first. Each puzzle grants *one star*. Good luck!

Here's an easy puzzle to warm you up.

Santa is trying to deliver presents in a large apartment building, but he can't find the right floor - the directions
he got are a little confusing. He starts on the ground floor (floor `0`) and then follows the instructions one
character at a time.

An opening parenthesis, `(`, means he should go up one floor, and a closing parenthesis, `)`, means he should go down
one floor.

The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.

For example:

* `(())` and `()()` both result in floor `0`.
* `(((` and `(()(()(` both result in floor `3`.
* `))(((((` also results in floor `3`.
* `())` and `))(` both result in floor `-1` (the first basement level).
* `)))` and `)())())` both result in floor `-3`.

To *what floor* do the instructions take Santa?

Your puzzle answer was `74`.

## --- Part Two ---

Now, given the same instructions, find the *position* of the first character that causes him to enter the basement
(floor `-1`). The first character in the instructions has position `1`, the second character has position `2`, and so
on.

For example:

* `)` causes him to enter the basement at character position `1`.
* `()())` causes him to enter the basement at character position `5`.

What is the *position* of the character that causes Santa to first enter the basement?

Your puzzle answer was `1795`.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should [return to your Advent calendar][1] and try another puzzle.

If you still want to see it, you can [get your puzzle input][2].

You can also [Shareon [Twitter][3] [Mastodon][4]] this puzzle.

[1] /2015
[2] 1/input
[3] https://twitter.com/intent/tweet?text=I%27ve+completed+%22Not+Quite+Lisp%22+%2D+Day+1+%2D+Advent+of+Code+2015&url
=https%3A%2F%2Fadventofcode%2Ecom%2F2015%2Fday%2F1&related=ericwastl&hashtags=AdventOfCode
[4] javascript:void(0);
