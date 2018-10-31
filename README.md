# Trie

[![Build Status](https://travis-ci.com/leshow/trie-rs.svg?branch=master)](https://travis-ci.com/leshow/trie-rs)

**PRs are welcome**

My first trie implementation was in javascript, I needed it for fast string searching in a project at work. I later refactored it, extracted it to it's own module (not available publicly) and added Flow type annotations to gain some greater confidence in the code.

Since wasm has been taking off, I've wanted to use a trie implemented in rust to increase performance, and more importantly lower the memory cost of the data structure. This is my attempt at that.

## crates.io

Published on [crates.io](https://crates.io/crates/trie_map)

Evan Cameron
