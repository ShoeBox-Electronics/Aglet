### Current Task List

- Function signatures like `funk fib(n: int) -> int { ... }`
- No significant whitespace
- requiring statements to end in a semicolon is OKAY (see if we can avoid it though?)
- Entry point will be `main` function: `funk main() -> void { ... }`
- Valid types: `int`(signed 16 bit), `char` (unsigned 8 bit), `void` (function doesn't return a value)
- Possibly support for fixed-length arrays? we'll see. Not high priority.
  - If so, keep in mind that arrays will need some way to have the length checked...


### Basic Road Map (will change)

- Don't worry about objects yet.
- Don't worry about arrays yet.
- Don't worry about strings yet.
- Don't worry about floats yet.
- Don't worry about booleans yet.
- Don't worry about integers bigger than 8bit.
- Basically, for first iteration, just integers and characters.

- Functions are first class citizens.
- NO ASYNC / PARALLEL (don't think the 6502 can even do that?)
- IF based conditionals. NO CASE (at least at first)
- Range based for loops... I don't think C's for loops are good at all. Don't have to worry about this yet. will be V2

- Don't worry about self-hosting yet! that is a ways away.
- However, syntax should be simple enough that self-hosting may be possible in the future!
- Ideally the source code can be compiled in a single pass...
- Need to keep tiny memory footprint in mind!!!!!!
- Alternately, we could have the compiler

Talking points:
- Iterators as a language construct? might be cool.
- Constant VS mutable? Something like
- How many keywords do we want? I like to keep them to a minimum.
- Strict typing is a must, it simplifies stuff so much.
Later...:
- How will we tell the OS to run some program? maybe syntax like `!['test', 'param1', 'param2', var]`? it needs to be simple, easy to add... maybe `![test param1 param2 $var]`? The latter can just be syntax sugar for `exec(['test', 'param1', 'param2', var])`:: DON'T WORRY ABOUT THIS YET

- In the future, maybe allow dot notation for function calls? e.g. `foo.bar(baz)` is just syntax sugar for `bar(foo, baz)`... something to think about.

-----

Some form of timekeeping, just simple as "how long has the computer been running"
start with while loops, do for loops LATER!


NOTES FOR FIRST RUN:
>	We want functions
>
>	WHILE and IF are only 2 control structures
>
>	"set" for constants, "let" for mutable vars
>	syntax like `set x:i16 = 123` or `let y:i8 = 10`
>	Might be too similar? need to crowd test this, see if it's too easy to confuse.
>
>	Don't worry about integers bigger than 1 byte, or non-integer types.
>
>	allow inline assembly at first, and then when we generate raw bytecode, allow inline bytecode.
