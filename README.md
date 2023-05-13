# WhyHash
WhyHash is a very simple and easy to understand hash function questionably derived from wyHash and MUM.
It should be fast, relying only on a few simple operations.
It should also be somewhat safe, producing well distributed hashes and being guarded against a sticky 0 state and zeroing of the internal state except for sheer bad luck.
The 0xda942042e4dd58b5 constant is shamelessly copied from Daniel Lemire's blog. What's good enough for him is good enough for me, right?
