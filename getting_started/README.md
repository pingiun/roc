# Getting Started

Roc is a language for making delightful software. It does not have an 0.1 release yet, and we
certainly don't recommend using it in production in its current state! However, it can be fun to
play around with as long as you have a high tolerance for missing features and compiler bugs. :)

The [tutorial](../TUTORIAL.md) is the best place to learn about how to use the language - it assumes no prior knowledge of Roc or similar languages. (If you already know [Elm](https://elm-lang.org/), then [Roc for Elm Programmers](https://github.com/roc-lang/roc/blob/main/roc-for-elm-programmers.md) may be of interest.)

There's also a folder of [examples](https://github.com/roc-lang/roc/tree/main/examples) - the [CLI form example](https://github.com/roc-lang/roc/tree/main/examples/interactive/form.roc) in particular is a reasonable starting point to build on.

If you have a specific question, the [FAQ](../FAQ.md) might have an answer, although [Roc Zulip chat](https://roc.zulipchat.com) is overall the best place to ask questions and get help! It's also where we discuss [ideas](https://roc.zulipchat.com/#narrow/stream/304641-ideas) for the language. If you want to get involved in contributing to the language, Zulip is also a great place to ask about good first projects.

## Installation

- [Linux x86](linux_x86.md)
- [MacOS Apple Silicon](macos_apple_silicon.md)
- [MacOS x86](macos_x86.md)
- [Windows](windows.md)
- [Other](other.md)

## Running Examples

You can run examples as follows:

```
$ cd examples/hello-world
$ roc run
```

Some examples like `examples/benchmarks/NQueens.roc` require input after running.
For NQueens, input 10 in the terminal and press enter.

[examples/benchmarks](examples/benchmarks) contains larger examples.

**Tip:** when programming in roc, we recommend to execute `./roc check myproject/Foo.roc` before `./roc myproject/Foo.roc` or `./roc build myproject/Foo.roc`. `./roc check` can produce clear error messages in cases where building/running may panic.

## Getting Involved

The number of people involved in Roc's development has been steadily increasing
over time - which has been great, because it's meant we've been able to onboard
people at a nice pace. (Most people who have contributed to Roc had previously
never done anything with Rust and also never worked on a compiler, but we've
been able to find beginner-friendly projects to get people up to speed gradually.)

If you're interested in getting involved, check out
[CONTRIBUTING.md](https://github.com/roc-lang/roc/blob/main/CONTRIBUTING.md)!
