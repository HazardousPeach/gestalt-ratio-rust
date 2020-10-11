Gestalt Pattern Matching
========================

This crate provides a single function, gestalt_ratio, which computes
the gestalt pattern matching ratio between two strings, based on
recursively looking at longest common substrings. The algorithm is
described here: https://en.wikipedia.org/wiki/Gestalt_Pattern_Matching
, and was originally described by John W. Ratcliff and John
A. Obershelp in Dr. Dobbs Journal in 1988.

This metric is intended to show strings which "look similar" as more
similar.

This crate was written by Alex Sanchez-Stern
