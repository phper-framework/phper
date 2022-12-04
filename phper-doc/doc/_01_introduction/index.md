# Introduction

`PHPER` is the framework that allows us to write PHP extensions using pure and safe Rust whenever possible.

`PHPER` means `PHP Enjoy Rust`.

## Rust ❤️ PHP

The crates are not only the PHP binding for Rust, but also the framework for writing PHP extension.

## Purpose

I used to use C language to write PHP extensions. At that time, C/C++ are the only way to write PHP extensions.

But I found the problem is that using C language can easily cause memory problems, which is very troublesome when debugging PHP extensions.

Moreover, third-party libraries in C language are not easy to use, and version compatibility problems are often encountered in dynamic linking, which is inconvenient to use.

Later, Rust appeared, and I started to use C to call Rust's FFI to develop PHP extensions. The experience is better than only use C language.

However, it is not convenient for Rust to generate C ABI and then call C, so I got the idea of using pure Rust to write PHP extensions.

So I started to build the framework of phper.

The other goal is to enable PHP to benefit from the Rust ecosystem.
