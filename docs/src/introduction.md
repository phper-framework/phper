# Introduction

`PHPER` is the framework that allows us to write PHP extensions using pure and safe Rust whenever possible.

`PHPER` means `PHP Enjoy Rust`.

## Rust ❤️ PHP

The crates are not only the PHP binding for Rust, but also the framework for writing PHP extension.

## Purpose

I used to use the C language to write PHP extensions. At that time, C and C++ were the only ways to write PHP extensions.

But I found the problem was that using the C language could easily cause memory problems, which were very troublesome when debugging PHP extensions.

Moreover, third-party libraries in the C language are not easy to use, and version compatibility problems are often encountered in dynamic linking, which is inconvenient to use.

Later, Rust appeared, and I started to use C to call Rust's FFI to develop PHP extensions. The experience was better than using only the C language.

However, it was not convenient for Rust to generate C ABI and then call C, so I got the idea of using pure Rust to write PHP extensions.

So I started building the framework for phper.

Another goal is to enable PHP to benefit from the Rust ecosystem.
