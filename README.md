unicode-branchless: fast char to UTF-8

Branchless encoding of codepoints into UTF-8.

This Rust library contains several functions that translate
a Rust `char` to an array `[u8; 4]` plus a length. (It would
be better to return `&[u8]`, but there is no reasonable way
to do this with Rust's ownership rules unless a mutable
slice is passed in — that approach was not chosen here.)

## Configuration

Choose one of three versions based on Rust compile-time
feature flags:

* Default version is branchless self-contained code.

* Feature `tabled` is branchless and uses table lookup for
  some constants.
  
* Feature `looped` has a controlling `while` loop that has
  one conditional and one unconditional branch per byte.

All versions produce identical results up to performance.

## Benchmarking

A very simple Criterion benchmark is included. On the
benchmark machine (AMD Ryzen 9 3900X):

* The default version runs in about 3.00ns on all inputs.

* The `tabled` version runs in about 2.75ns on all inputs.

* The `looped` version runs in linear time as expected.
  * single byte: 2.11ns
  * two bytes: 2.52ns
  * three bytes: 2.93ns
  * four bytes: 3.31ns.


This is a very micro-benchy microbenchmark. In a real-world
scenario:

* The tableless versions will cause additional register
  pressure.

* The looped version will likely perform worse unless
  its branch predictors stay around.
  
* The tabled version will likely perform quite poorly unless
  its tables stay cached in L1.

## Notes

Thanks to Nathan for the question and Lorenz for the CLZ
insight of an earlier draft.

See <https://github.com/skeeto/branchless-utf8> for decoding (the reverse).

See <https://cceckman.com/writing/branchless-utf8-encoding>
for the blog post describing this.
