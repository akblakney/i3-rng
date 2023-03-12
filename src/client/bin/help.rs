pub fn handle_help() {
  println!(
"****************************** i3-rng ******************************
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
");
}
