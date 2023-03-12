# i3-rng

## Overview

`i3-rng` is a random number generator (RNG) based off of the [i3 window manager](https://i3wm.org/). It has two binary targets: a daemon which is meant to run in the background, and a client program which can be used on the command line to query the daemon for randomness. 

RNGs have two main components: a source of entropy, or "randomness", and a mechanism to transform that entropy into a target distribution (usually a uniform distribution). In this project, key-bindings for the i3 window manager are used as a source of entropy--both the key-bindings themselves and the time in between each key-binding. The latter should be a very reliable source of entropy: we measure the time between two user inputs, in nanoseconds, and this should give us at least a couple of bits of entropy (conservatively). 

The second component, transforming the entropy into a desired target distribution (in this case the uniform distribution of raw byte values, which can then be used to generate random characters, hex strings, etc.), is done in a manner very similar to the Linux kernel's RNG, `/dev/random` ([source code](https://elixir.bootlin.com/linux/latest/source/drivers/char/random.c)). Namely, the `blake2` hash function is used to collect and store entropy (by performing updates on the raw entropy coming from the i3-wm events), and this entropy is then expanded into a stream of arbitrary length by the `chacha20` stream cipher. The default Rust implementations of these cryptographic functions are used.

## Installation
TBD
