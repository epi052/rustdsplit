# rustdsplit

![CI Pipeline](https://github.com/epi052/rustdsplit/workflows/CI%20Pipeline/badge.svg)

# What's it For?

> Several years ago, the tool "DSplit" was released by class101 which was used to demonstrate how some AV signatures could be bypassed by finding and modifying one byte within the binary. Unfortunately, the original file (and source code?) is no longer available for download by the author.
>
> http://obscuresecurity.blogspot.com/2012/12/finding-simple-av-signatures-with.html

During OSCE's AV bypass module, I recalled learning about the method described in the linked post and using DSplit to bypass signature based AV detection.  I wanted to play around with it using the OSCE labs.  I proceeded to search for DSplit and came to the same conclusion as the above author, what **can** be found looks rather janky.

At the same time I was playing with the Rust programming language.  I decided it would be a fun learning exercise to re-implement DSplit using Rust.  This project is the result.       

# What's it Do?

This tool takes a binary (or really any file) and splits it in half.  It also adds a 12-byte signature to each half that stores the original file's offsets (main takeaway is that files can be named w/e and still be split again).  

# How do I Use it?

The typical work flow is as follows:

1. Take original binary that AV flags as malicious
2. Split it in half
3. Scan both halves
4. Find the half that still gets flagged by the AV
5. Repeat steps 2-4 until you know the byte-offset of the signature
6. Change it

Due to the fact that only half of the file is (re)split, the number of total splits is quite small (we effectively perform a binary search on the file, which is O(log n) in the worst case).

# Small Demo

[![asciicast](https://asciinema.org/a/351311.svg)](https://asciinema.org/a/351311)

# Downloads

There are pre-built binaries for the following systems:

- [Linux x86](https://github.com/epi052/rustdsplit/releases/latest/download/x86-linux-rustdsplit.zip)
- [Linux x86_64](https://github.com/epi052/rustdsplit/releases/latest/download/x86_64-linux-rustdsplit.zip)
- [MacOS x86_64](https://github.com/epi052/rustdsplit/releases/latest/download/x86_64-macos-rustdsplit.zip)
- [Windows x86](https://github.com/epi052/rustdsplit/releases/latest/download/x86-windows-rustdsplit.exe.zip)
- [Windows x86_64](https://github.com/epi052/rustdsplit/releases/latest/download/x86_64-windows-rustdsplit.exe.zip)

