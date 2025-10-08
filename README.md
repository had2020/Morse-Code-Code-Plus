=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

# Morse Code Code Plus

Turing-complete and hyper-minimalist language

=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

Abstract:

Do you enjoy abstract suffering? Then this langauge is just ripe for you! 
Back in my days people had to write code in machine code. The only real pure form of code. Langauges like BF are too bloated.
The great Machine God would be displeased. Other more hertical even suggest of thing like Assembly, which is basically turning code into toy building blocks.
Code is not a tool, it is a gift of the machine code. Some hertical wizards even deal in C, playing with dealy hanging pointors and memory courption. 
While other stright up hertics in python, are spoon feed by the python virtual machines. They along with the others of their kindling are all in needed of the 
saving grace of Morse Code Code Plus. And a return to the roots of the real orignal internet, morse code.

- Btw this is a satire langauge for those of you who don't get it. Have fun coding! 

Morse Code Code Plus was written on arch without a desktop in vim, the way it was ment to be. Now the problem with BF is that 8 chars in the synatx is too much
we need only 3. With three we can still remain turing-competete. It all runs on the only super-high-level langauge accepted by the Machine God, Rust. 
Morse Code

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

- Any other hertical characters will be ignored, you can use them as comments if needed.
  
- Any other combo of the dot and dash will be translated into the Base 2 number system in machine code. https://en.wikipedia.org/wiki/Binary_number for infomatation on the system.

- All memory blocks are not initialized as zero until the point performs an operation on it.

- Extra characters written beyond the first character are ignored. So you can use them to vent if needed.

- You can nest loops as much as you need to punish your computer.

- Also in the Base 2 Number Constants, numbers beyond 4 bits, will be added as a separate number.

# Huge inspiration:
https://gist.github.com/roachhd/dce54bec8ba55fb17d3a
