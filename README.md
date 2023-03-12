# i3-rng

## Overview

`i3-rng` is a random number generator (RNG) based off of the [i3 window manager](https://i3wm.org/), written in Rust. It has two binary targets: a daemon which is meant to run in the background, and a client program which can be used on the command line to query the daemon for randomness. This project is intended to be more theoretical and educational than practical: if you are using Linux (and you probably are if you're using i3wm), you already have a robust RNG available through your kernel, which is saturated with entropy almost instantly after boot. In contrast, `i3-rng` may take minutes to gather enough entropy to deliver randomness, as the daemon blocks until enough entropy has been collected. Finally, I am not a cryptographer and cannot make any security guarantees about the randomness produced by this program. Nonetheless, a home-made RNG that runs in the background is an interesting project to tackle, and it has taught me a lot about multi-threading, sockets, and Rust in general.

RNGs have two main components: a source of entropy, or "randomness", and a mechanism to transform that entropy into a target distribution (usually a uniform distribution). In this project, key-bindings for the i3 window manager are used as a source of entropy--both the key-bindings themselves and the time in between each key-binding. The latter should be a very reliable source of entropy: we measure the time between two user inputs, in nanoseconds, and this should give us at least a couple of bits of entropy, as the least significant bits of this measurements will contain a high level of entropy. (This method of using time differences in user inputs is commonly used in other cryptographic settings in which entropy needs to be collected, although often times better sources of entropy are available to the kernel, as user input is slow). 

The second component, transforming the entropy into a desired target distribution (in this case the uniform distribution of raw byte values, which can then be used to generate random characters, hex strings, etc.), is done in a manner very similar to the Linux kernel's RNG, `/dev/random` ([source code](https://elixir.bootlin.com/linux/latest/source/drivers/char/random.c)). Namely, the `blake2` hash function is used to collect and store entropy (by performing updates on the raw entropy coming from the i3-wm events), and this entropy is then expanded into a stream of arbitrary length by the `chacha20` stream cipher. The default Rust implementations of these cryptographic functions are used. 

## Installation
First, clone the repository:
```
git clone https://github.com/akblakney/i3-rng.git
```

Then, to install the binaries into your `/usr/local/bin`, run the following commands in root directory of the project:

```
cargo install --path ./
sudo install target/release/daemon /usr/local/bin/i3-rngd
sudo install target/release/client /usr/local/bin/i3-rng
```

Now, the daemon which runs in the background is accessible with `i3-rngd` and the client program (which the user will use to get randomness from) is accessible with `i3-rng`. If you like you can have the daemon automatically execute on startup by adding `exec --no-startup-id i3-rngd` to your i3 config file.

## Usage

Run with `i3-rng help` to see help menu. Here it is reproduced below:

```
DESCRIPTION: a random number generator based on entropy gathered                                                                                                                                                  
from the i3 window manager in the background. Follow the installation process                                                                                                                                     
at https://github.com/akblakney/i3-rng and make sure the daemon process, 
i3-rngd, is running in the background to use i3-rng. 

USAGE: i3-rng command [options]

i3-rng help

  bring up this help menu

i3-rng query

  query the daemon for how much entropy, in bits, it has collected. By default
  the daemon blocks output until 512 bits of entropy are collected.

i3-rng rand [-n <number of bytes>] [-f bytes | ascii | alpha-numeric 
  | Alpha-numeric | hex | digits] [--harden]

  NOTE: this command will return an error exit status when the daemon has
  insufficient entropy.

  -n=<int> gives the number of bytes requested from the RNG. Defaults to 64.
  Note: this parameter does NOT give the length of the output in ascii
  characters, alpha-numeric characters, etc., as often times more than one
  byte is required for one character of output. To generate strings of a certain
  length, use a larger value for -n and then truncate. See examples.

  -f=format gives the output format. Defaults to bytes. alpha-numeric is
  lower-case only while Alpha-numeric is mixed case. 

  --harden combines entropy from i3wm with random output from /dev/urandom.

EXAMPLES:

i3-rng rand > random_bytes
   
  outputs 64 random bytes to the file random_bytes

i3-rng rand -f ascii | head -c 16
   
  output 64 bytes of random output in the form of printable ASCII characters,
  then truncate the output to 16 characters with head.

i3-rng rand -n 1000
  print 1KB of random output to stdout
```
