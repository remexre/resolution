resolution
==========

A solver for the resolution problems in CSCI4511W.

(Like all my stuff related to classes, please don't use this to cheat on your homework, thanks.)

Input Format
------------

Input files are a single line specifying the proposition to prove true (by contradiction), then propositions known to be true (which form the knowledge base).

Each is specified as a list of lowercase ASCII letters separated with a pipe (`|`). Blank lines are ignored, as is non-newline whitespace.

Example
-------

> Proposition: `a`
>
> KB:
>
> -	`a V c`
> -	`b`
> -	`!b V !c`

This would be specified as:

```
a|

ac|
 b|
  |bc
```

Output Format
-------------

Four output formats are implemented:

-	`ascii`: ASCII-art sequents will be output to show the derivation.
-	`latex`: [bussproofs](https://ctan.org/pkg/bussproofs)-compatible proofs will be output.
-	`silent`: No output will be printed to stdout. The program will exit with status 0 if the proposition was provable, and 1 if it was not.
-	`unicode`: Similar to `ascii`, but uses U+2228 and U+00AC instead of `V` and `!`.
