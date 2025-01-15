Branchless encoding of codepoints into UTF-8.

Thanks to Nathan for the question and Lorenz for the key insight.
Any remaining mistakes are mine.

This doesn't handle invalid codepoints, e.g. the space for surrogate pairs;
it accepts some things it shouldn't, and rejects some things it should.
Send a PR if it bothers you!

See https://github.com/skeeto/branchless-utf8 for decoding (the reverse).

