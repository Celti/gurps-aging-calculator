GURPS Aging Calculator
======================

An iterative calculator for the [SJGames](http://www.sjgames.com/) [GURPS™ 4th Edition](http://www.sjgames.com/gurps/) rules for character aging (*Basic Set: Campaigns*, p. 444, with supplementary rules from *Basic Set: Characters*, pp. 53, 66, 153, and 154).

Now written in [Rust](https://www.rust-lang.org/) instead of [Perl](https://github.com/Celti/gurps-scripts/)!

Usage
=====
```
$ ./gurps-aging-calculator --help
gurps-aging-calculator 2.0.0
Patrick Burroughs (Celti) <celti@celti.name>
Iterative calculator for the GURPS aging rules.

USAGE:
	gurps-aging-calculator [OPTIONS]

OPTIONS:
    -a, --add <ADD>                       Total additional optional modifiers to the aging roll.
    -d, --death <DEATH>                   The HT the character is considered dead at (default is 4).
    -x, --extended-lifespan <EXTENDED>    The character's level of Extended Lifespan (default is 0).
    -h, --ht <HT>                         The character's starting HT (default is 10).
    -i, --iterations <ITERATIONS>         The number of iterations to calculate (default is 100,000).
    -l, --longevity                       The character has Longevity.
    -p, --max-procs <MAX_PROCS>           The maximum number of threads to spawn (default is 4).
    -D, --self-destruct                   The character has Self-Destruct.
    -s, --short-lifespan <SHORT>          The character's level of Short Lifespan (default is 0).
    -t, --tl <TL>                         The character's lifetime medical TL (default is 8).
    -v, --verbose <VERBOSE>               Log verbose output to specified file ('-' indicates stdout).
    -?, --help                            Prints help information
    -V, --version                         Prints version information

$ ./gurps-aging-calculator
Median age of death is 77.5 (highest is 151, lowest is 53).
Mean is 78.8345475; StdDev is 10.548953687426954.
```

Downloading
===========
Either clone this repository and build it with `cargo`...

```
$ git clone https://github.com/Celti/gurps-aging-calculator.git
$ cargo build --release
$ target/release/gurps-aging-calculator
```

...or see the [releases page](https://github.com/Celti/gurps-aging-calculator/releases) for Linux and Windows 64-bit static binaries.

License
=======

GURPS is a trademark of Steve Jackson Games, and its rules and art are
copyrighted by Steve Jackson Games. All rights are reserved by Steve Jackson
Games. This game aid is the original creation of Patrick Burroughs and is
released for free distribution, and not for resale, under the permissions
granted in the [Steve Jackson Games Online Policy](http://www.sjgames.com/general/online_policy.html).

Copyright (c) 2016 Patrick L. H. Burroughs.

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software for non-commercial use, including without limitation the rights to
use, copy, modify, merge, publish, distribute, or sublicense under a compatible
license, and to permit persons to whom the Software is furnished to do so,
subject to the following condition:

The above copyright and trademark notices and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
