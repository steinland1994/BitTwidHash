# BitTwidHash
BitTwidHash is a fast hashfunction built on ZwoHash's write function, gathers the data by adding and rotating and then finishes with a round of xoshiro128++.

ZwoHash's write function may collide on certain inputs (eg. 1111 and 111111), which is why another, slower write variant is also present in the code.

THIS HASH FUNCTION IS WRITTEN FOR LEARNING PURPOSES. PLEASE DO NOT RELY UPON IT (YET).
