# strcmp

This is a tiny command-line program that prints whether its first two
command-line arguments are the same.  This is intended for checking whether two
checksums match each other.

## Quick start

    $ git clone https://github.com/davepacheco/strcmp
    $ cd strcmp
    $ cargo build

    $ ./target/debug/strcmp 0b7800f24bb872e61428d200aef4a66e 0b7800f24bb872e61428d200aef4a66e
    strcmp: arguments match ("0b7800f24bb872e61428d200aef4a66e")

    $ ./target/debug/strcmp 0b7800f24bb872e61428d200aef4a66e 0b7800f24bb812e61428d200aef4a66e
    strcmp: argument mismatch
    arg1: 0b7800f24bb872e61428d200aef4a66e
    arg2: 0b7800f24bb812e61428d200aef4a66e
