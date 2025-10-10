=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

# Morse Code Code Plus

Turing-complete and hyper-minimalist language

=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

Abstract:

Do you crave the sweet, soul-crushing agony of abstract suffering? Then buckle up, buttercup, because Morse Code Code Plus is your new masochistic muse!
Back in my day, we etched code straight into silicon with our bare teeth, machine code, the one true divine binary gospel. Anything else? Pure heresy! BF? Pfft, that bloated behemoth with its EIGHT whole characters? It's like showing up to a knife fight with a Swiss Army kitchen sink. The Machine God weeps rusty tears at such excess.

And don't get me started on those Assembly apostates, turning sacred instructions into Lego sets for toddlers. Code isn't some kiddie toy, it's a holy sacrament from the digital deity itself! Then there are the C sorcerers, those reckless warlocks juggling dangling pointers like flaming chainsaws while memory corruption lurks like a bad ex at a wedding. Absolute chaos!
Worse yet, the Python pagans, lazy lounge lizards being spoon-fed gourmet scripts by their virtual machine nannies, never tasting the raw, unfiltered pain of true computation. These soft-bellied sinners, along with their JavaScript jesters and Ruby rascals, are all desperately in need of redemption through the purifying dots and dashes of Morse Code Code Plus. It's time to teleport back to the original internet's golden age: Morse code, baby, when messages traveled by telegraph and your code could double as a distress signal!

(BTW, this is satire, folks. If you're taking notes on how to start a cult, please seek help. Have a blast coding... or not!)

Why settle for BF's extravagant octet when THREE symbols (dot, dash, and space, boom!) keep us Turing-complete? It all compiles down using Rust, the Machine God's begrudgingly approved "super-high-level" indulgence (shh, don't tell the purists). Praise the pulses!

# How Morse Code Code Plus works?

- The compiler it is written in Rust, and parses thourgh a file, with the file extension mccp. In which asci code text is written, in either of the characters below.
- `.`, `-`, and ` `
- Instead of having 8 set in stone actions based on characters, and only effecting repetions, Morse Code Code Plus is trully free with the usage of whitespace.

# BASICS

The whole concept behind `Morse Code Code Plus` is memory manipulation. Just like BF, but you are given this time, a pointer sized array, just like `vec` from rust. Within this special array, blessed by the Machine God, one can increase the memory pointer, increase the value at the memory pointer, etc. So without futher to do these are the man operators available.
``` Rust
.-  // increases memory pointer, or moves the pointer to the right 1 block.
-.  // decreases memory pointer, or moves the pointer to the left 1 block.
--  // increases value stored at the block pointed to by the memory pointer.
..  // decreases value stored at the block pointed to by the memory pointer.
... // like while(cur_block_value != 0) loop.
--- // if block currently pointed to's value is not zero, jump back to [
.   // like io::stdin().read_line() in Rust. input 1 character.
-   // like println!() in Rust. print 1 character to the console
Ex: .-.-.- // Number Constant, same as machine code, but . = 0, and - = 1.
```
These are the 8 keywords, they are separated by whitespace.
This now only used 3 chars vs 8, which is more minimalist. And more readable to some people.
* Note subjecs that translated the morse code have experienced symptoms of a trinary translation of reality

Some Rules:

- Any other characters will be ignored, you can use them as comments if needed.

- Any other combo of the dot and dash will be translated into the Base 2 number system in machine code. https://en.wikipedia.org/wiki/Binary_number for infomatation on the system.

- All memory blocks are not initialized as zero until the point performs an operation on it.

- Extra characters written beyond the first character are ignored. So you can use them to vent if needed.

- You can nest loops as much as you need to punish your computer.

- Also in the Base 2 Number Constants, numbers beyond 4 bits, will be ignored.
# How to build the compiler from source?

This project consists of the compiler for Morse Code Code Plus.
In order to build across different operating systems upon differing CPU architectures,
one can compile the Rust based compiler; then use it as a command line ulity to compile Morse Code Code Plus into corresponding Machine code.

Step 1.) Clone/Download the source Repo
``` Bash
git clone https://github.com/had2020/Morse-Code-Code-Plus.git
```

Step 2.) Compiling the compiler.
``` Bash
cargo build
```

Step 3.) Retrive the executable from target/debug, it is under the name MCCP, which stands for Morse Code Code Plus.

# How to compile Morse Code Code Plus?

Firstly, download a compiler release, if one is available for your target architecture, otherwise compile it from source.

``` Bash
./MCCP filepath # On Unix
```
To keep files organized by coding langauge I recommend using the MCCP file extension.
