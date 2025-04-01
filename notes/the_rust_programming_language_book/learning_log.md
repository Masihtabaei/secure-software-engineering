# Masih’s Learning Log: Exploring The Rust Programming Language Book

## Introduction

Here you will find my notes and thoughts on the well-known book [*The Rust Programming Language*](https://doc.rust-lang.org/book/). After a while I switched to the more interactive version of the book (available [here](https://rust-book.cs.brown.edu/experiment-intro.html)).

## 4 Understanding Ownership

### Lecture Correspondence

This chapter corresponds partially to lecture slides form the first two weeks with following file names:

- 01_SecSE25_Slides_20250321.pdf
- 02_SecSE25_Slides_20250328.pdf

### Notes and Thoughts

#### Whaf is Ownership?

Goals of Rust:
- Preventing undefined behaviors
- Compile-time preventioln for the sake of performance and productivity

Safety:
- Absence of undefined bahvior (e. g. using an undefined variable)

Undefined bahvior:
- [Comprehensive list](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
- Successfull execution without crashing
- Immediate crash due to some faults (e. g. segmentation fault) or other operating system errors
- Later crash after an actor misused the gap
- Dangerous esp. for low-level programs (memory corruptions cause of around 70% of reported security vulnerabilities)
- 

Checking for undefined bahviors during compilation time and runtime:
- Compile error during compilation time (e. g. in Rust)
- Exception during runtime (e. g. in Python or Javascript)
- Compile-time checks more efficient (less runtime checks and also avoiding bugs in production)

Ownership:
- Most unique feature of Rust
- Discipline of pointer management (not restricted to only heap pointers)
- Enabler for memory safety gurantees without garbage collector

Stack and Heap in Rust:
- *Arrays* on the stack and *Boxes* on the heap
- Actual copy for stack-based values
- Pointer or address copy for heap-based values (esp. more efficient for *Boxes*) 

Memory and its management:
- Pointers -> pointees
- Manual memory management in Rust forbidden (no direct access to *malloc* or *free*)!
- Owner reponsible for automatic memory management

Boxes:
- One way of allocating memory on the heap
- Self or similar structures used by other data structures like *Vec*, *String*, and *HashMap*

Move:
- Shallow copy
- Source not available after

Clone:
- Deep copy
- Both source and target available after move

Summary:
- Only one owner for heap data permitted
- Automatic heap deallocation after going out of scope (*Box* Deallocation Principle)
- Ownership transfer possible via move (used by assignments and function calls)

<hr>

#### References and Borrowing

References:
- Non-owning pointers
- Borrow
- 


Goal of references:
- Offering more convenience for move-only API and heap values (e. g. for handing over *Strings* as function arguments)
  

Dereferencing:
- Dereference operator `*`
- Can be concatenated
- Not that much visible because of automation done in many cases by Rust (e. g. using dot-syntax or method-call syntax like [`variable.method()`] or some macros)

Types of (de-) reference:
- Implicit
- Explicit

Alisiaing:
- Accesssing the same data through different variables
- Potentially dangerous (root of some problems like race conditions and more)

<u>Pointer Safety Principle:</u> Aliasing and mutating data at the same time should be forbidden.

Borrow Checker:
- Responsible for ensuring the safety of references and finding permission violations
- Three possible permissions on data (read, write, and own) at the compile-time (no runtime existence)

Permissions:
- Read: Copying data to another location allowed
- Write: Mutating data allowed
- Own: Moving or dropping data allowed
- By default read/own for all variables (using `mut` also write permitted)
- Temporary changes by references
- Defined on places (everything on the left-side of an assignment) and not just variables (the reason for possible nuances regarding permissions of a  reference and data behind it)
