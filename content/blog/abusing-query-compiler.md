+++
title = "Abusing the powers of query based compilers (draft)"
date = 2025-05-25
draft = true

[taxonomies]
tags = ["Programming", "Compilers", "Syntax", "Language Design"]
+++

Long before spacetime had been born, wizards of olde fought in the endless legendary debates of naming conventions. Some wielded CamelCase as heavy handily as they wielded C++ and Java. Others wielding the incomprehensible powers of snake_case could materialize deep learning codebases using only a wave of their hand and 16 gigabytes of Python extension modules. The wizards using Upper_Snake_Case died out long ago.

<!-- more -->

Notably, despite their powers, kebab-case was apparently non-existent. While the greatest wizards had slain many dragons with foreboding names like LR parsing, LALR parsing, abstract interpretation, or even sparse conditional constant propagation, this simple incantation is still classified as utterly incomprehensible:

```c
int impossible-things(int x) {
  int apple-count = x * 2;
  int world = apple-count - 5;
  char hello-world[] = "I serve no master! And I could say that %d times!";
  printf(hello-world, world);
  // Huzzah! Negative apples!
  return world-apple-count;
}
```

The spell castinator would of course read this and get back to you, exclaiming, "apples, count, hello? Insanity."

I'm not aware of any programming language  that allows kebab-case, because usually they allow you to express things like subtracting variables. Thus kebab-case is impossible if you want to allow unreadable subtraction expresssions, which spells QED and so we are done.

But... compilers are smart, right? It should be possible to notice apple-count and observe that it is an already declared identifier. However, let me explore one side tangent (likely first of many) on parsers and typedefs.

* TODO: mention lisp, COBOL, forth
* TODO: write section about agda language

## typedef and the C lexer hack

## Fearless kebab-case


* TODO: discuss how FORTRAN and ALGOL supported spaces once allowed whitespace in identifiers
* Inform7 supports whitespace in identifiers
* SQL supports whitespace in identifiers
* Fortran bug urban legend: https://www-users.york.ac.uk/~ss44/cyc/p/fbug.htm
