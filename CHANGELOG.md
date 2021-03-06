# Changelog

## 0.8.1 - October 04, 2020

* 1dc211c0: Fix make install for macOS/BSD
* 5440b4e4: Write new manifest to the root of the S3 bucket

## 0.8.0 - October 03, 2020

* 3c260f85: Escape Make subcommand for tagging
* c34a9116: Update wepoll-binding to 3.0
* 9b6738c3: Patch libffi to use unreleased bindgen changes
* f5acdb2e: Remove outdated PHONY entries
* 5f9bf6e0: Fix Makefile for building/installing
* b64323fe: Overhaul the build process and the CLI
* b7011125: Allow using of the system libffi installation
* 890b3896: Move comment in core::bootstrap to the right place
* 1d1677d8: Remove outdated TODO from the bytecode parser
* 695ddbf3: Rename various Duration as_X methods to to_X
* edc6a48e: Remove std::net::bits.to_stream
* 3aa3a3e1: Remove some outdated VM comments
* 2b3f9b61: Rename SetArray to AllocateArray
* ed44bf67: Split Allocate into Allocate and AllocatePermanent
* 8504e68c: Rename SetObject to Allocate
* 1e9aef76: Enable AES-NI using RUSTFLAGS
* 9e36965f: Give tracer threads a name
* b23d4cbb: Relocate the allocate_stacktrace() function
* bf1c5d39: Fix a NoMethodError when there are diagnostics
* 5268c5e0: Replace bytecode files with a bytecode image
* 6930068a: Add support for literal indexes beyond than 65 535
* 133600aa: Store the current module in a ExecutionContext
* 34ca7972: Reuse thread pools for GC tracer threads
* 0b55d1e2: Use aHash instead of SipHasher and FNV
* 76276999: Don't store keys in a Hasher

## 0.7.0 - August 04, 2020

* 35fb0ee7: Move instruction handlers to a separate directory
* 0619de82: Refactor various aspects of VM instructions
* a215a691: Rename Register to Registers
* b241834e: Limit methods to 255 arguments
* a1d6c210: Rename SetRegister to CopyRegister
* 0fdaf451: Remove various methods from the VM Process type
* d78ebee2: Don't store line numbers in ExecutionContext
* e1472acd: Rename "block return" to "method return"
* 7f3364b3: Remove some excessive RcBinding cloning
* e62a4f8c: Update various VM dependencies
* 5276839b: Don't clone when finding binding parents
* baf6d686: Fix generating of release tarballs
* 4868f9a9: Fix stdout.print/stderr.print panicking
* cc20ee2a: Fix clippy warning about using as_deref()
* 08385683: Replace the Dynamic type with the "Any" trait
* 3ca999ab: Fix clippy and rustfmt warnings
* 715aff37: Add Boolean.true? and .false?, and remove .not
* f71a1e72: Use single-line if_true/if_false where possible
* 5352ec7f: Upgrade the "time" crate to 0.2.x
* d5c10f8b: Remove type parameters when re-opening objects
* 278a1652: First-class modules and remove top-level object
* a97f10be: Clean up Nil by splitting it into NilType and Nil
* ddedead0: Restore Boolean "if" instruction cleanup
* f46694ef: Remove support for constant receivers
* 1102c455: Use String for DefineObject and DefineTrait names
* 42bc9b0b: Don't define a panic handler in std::vm
* 2e49e503: Clean up std::prelude a little bit
* f6bf5de0: Add Array.reverse_iter
* 4af9354f: Remove support for extending traits
* 9f152024: Remove forward trait declaration from std::net::ip
* 679ccf7b: Rename a few AST nodes to use the pattern DefineX
* 81a8fd1e: Support circular types in the Ruby compiler
* def89e1f: Allow hashing of any object
* 22328acc: Revert Boolean and Nil cleanup
* 8f9109a4: Split Nil into Nil and NilType
* ecd11a07: Clean up Boolean using an "if" instruction
* bbffd1f3: Updated various VM dependencies
* f668bbe2: Fix Clippy warnings introduced with Rust 1.40
* 87fc0364: Fix kqueue network poller deprecation
* dcc7798b: Added tests for std::fs::path::bits
* 977f0740: Rename std::fs::raw to std::fs::bits
* 46dc7ddf: Add Path.directory
* a65c52d0: Add String.slice_bytes() and String.split()
* 510e6d0c: Change Process.identifier to return an Integer
* d3841771: Refactor std::mirror
* b59f2f72: Remove CallFrame.to_string
* e10bee15: Add ByteArray to the prelude
* 7a990558: Rename Void to Never
* 98544de7: Use keywords for all use of Pair and Triple
* 12346150: Tag return types where Dynamic is not needed
* 6c4ec92a: Lower collection thresholds from 8/16 to 2/4
* 9cf7f6e6: Return self in implementations of "each"
* ba9df4f1: Added std::set::Set
* 1a65f83f: Add Array.join for joining Array values
* a64f2e70: Remove left-over prefetch code
* 725dd097: Use bit flags for tracking a process' status
* 9be61a04: Stop using underscores for "private" methods
* e0a458d4: Simplify tracing code to improve performance
* f2f7283e: Use a Vec for BlockList
* a4ab6e07: Fix parsing of block arguments without parentheses

## 0.6.0 - November 17, 2019

* bdd32f5a: Upgrade some code and dependencies for Rus 1.39
* 347ade20: Fix various GC bugs and improve GC performance
* cb17da33: Use AtomicU8 for line and object bytemaps
* 8652b763: Add some additional tests for tracing objects
* b4cc7534: Clean up racy scheduler test to not use a barrier
* 9d2c6ba7: Clean up pushing binding and register pointers
* c6407a0a: Remove async finalisation of objects
* bde4250c: Make ArcWithoutWeak NonNull
* 60d86364: Clean up extending of types in the runtime
* 3634ceee: Add Pathname.join, .absolute?, and .relative?
* 7b7165ef: Add String.byte
* d42a6819: Rename Expressions to Body
* 486cc3c8: Add Iterator.partition and std::pair
* 075c7af5: Move std::trait.implement to Trait.implement
* 457706e1: Move std::array_iter to std::array::extensions
* 146b462a: Added Integer.times
* 839916ae: Add Iterator.select
* d22e281e: Add Iterator.any?
* b0ccdba1: Add Object.not_nil?
* 60c47568: Do not expose Trait and Module by default
* d87be76d: Clean up Object, Conditional, and Boolean

## 0.5.0 - September 16, 2019

* bffdb0c1: Implement Inko's parser in Inko
* d99f8407: "Short circuit" printing.
* 6f06114b: Improve output of failed tests
* a106a003: Use a function for integer shift errors
* 460e66b5: Reduce use of format!() for VM panics
* 10a3f29f: Move integer formatting out of a macro
* 13ff6916: Fix Cargo warnings using latest Rust
* c5d3b303: Remove support for binary newline sends
* c94fb713: Remove support for Array literals
* 94ac7d14: Remove hash map literals
* 7fe9e24f: Rename HashMap to Map
* 78bf415a: Merge Table and HashMap together
* 12fe6648: Move IP parsing methods to the IP objects
* a638341e: Move std::float.parse to Float.parse
* 42b9fd21: Move std::integer.parse to Integer.parse
* bf31fe71: Move hash_map.from_array to HashMap.from_array
* 563a0ab5: Use static methods for SystemTime and Duration
* f86eb0b0: Add syntax support for static methods
* 48dfebed: Require all attributes to be assigned in "init"
* 5ca42d1a: Define attributes inside object bodies
* 8554cc3b: Remove non-primitive prototypes from the VM
* f3270b13: Use a single File type in the VM
* 48e4213e: Change the syntax for the not-Nil operator
* 61961924: Use different token types for the lexer
* 55a58ace: Improve support for passing trait arguments
* a518c120: Implement Inko's lexer in Inko itself
* feb8e42e: Fix rehashing of HashMap
* 6385da73: Don't overwrite attribute types upon reassignments
* 770bc946: Rework HashMap internals and add RNG support
* 23f2e7fb: Fix lexing hexadecimal numbers containing "e"
* 2c96d0dd: Optimise looking up unused HashMap keys
* e357fd08: Added ToByteArray trait
* c88ba04f: Remove support for nested objects and traits
* e6aa5900: Add support for building the VM with jemalloc
* b513911a: Use AtomicU16 for Immix histograms
* fc417e39: Add ByteArray.slice
* ded03aad: Remove support for setting compiler options
* 53ff6b02: Remove support for `object ... impl` syntax
* 0b9ccdab: Remove use of `object ... impl` syntax
* eddffb9f: Add Range.cover?
* 468a7044: Fix failing Clippy builds
* da7e3128: Fix two use-after-free bugs regarding sockets
* 55473d46: Add Enumerator for writing iterators

## 0.4.1 - May 14, 2019

* 53675b7: Refactor connecting of sockets

## 0.4.0 - May 11, 2019

* 2fb8e93: Fix non-blocking socket reads
* 0393df5: Turn accepted sockets into non-blocking sockets
* f913117: Added support for shutting down sockets
* 3974f47: Added support for non-blocking network IO
* 99c9f04: Removed use of "extern crate"
* 5ca5a29: Use Rust 2018 for the VM
* e9331a7: Removed unused code from the compiler
* ff6297f: Reduce Immix block size to 8 KB
* 11ee161: Remove PIDs from processes
* c7c0bef: Refactor the global allocator and block sizes
* bd3465a: Fix storing module methods in globals
* e74843a: Fix various memory leaks in the VM tests
* 1b92357: Replace std::sync::Arc with ArcWithoutWeak
* 291d76b: Added some finalization tests when copying/moving
* 8290e5a: Don't send empty finalization requests
* 3e5882b: Rewrite the process scheduler from the ground up
* d80c4c3: Use BufReader when reading bytecode from files
* 4758073: Implement Length for StringBuffer
* a3d9ab9: Move empty? to the Length trait
* 4b87da2: Make late binding noops for blocks as return types
* 6bea945: Reorganise and test std::debug
* fc2f797: Removed the Compare trait
* bb35dea: Reorganise and test std::test
* 7f0cb46: Reorganise and test std::time
* f9c07ef: Fixed outdated keyword argument optimisation
* a4c2d68: Added method Float.to_bits
* 57247e7: Rename Immix bitmap structures to bytemaps
* 9af6498: Fix lookups in ObjectMirror.implements_trait?
* e33ed34: Added tests for std::trait
* 1a57743: Move all "class" methods to module methods
* 26f0023: Enforce a limit on bytecode input sizes
* 6558d6b: Parsing of Strings into Floats and Integers
* a38d0f5: Remove SystemTime.format and SystemTime.parse
* b3c7e36: Move std::reflection into std::mirror
* bf0d6d6: Tests for std::process and remove receive_if
* 8747098: Add tests for std::range and expose via prelude
* a8105b0: Added tests for std::mirror
* a51d1b6: Reset maiting state when there is a process
* 86cadf1: Removed storing of entire process statuses
* 3caf963: Use two survivor spaces for local allocators
* 65a8b19: Added type size test for LocalAllocator
* 3a7d49f: Stop storing garbage collection counts
* f84ec08: Update young and mature statistics separately
* 82284cc: Remove the tail from BlockList
* 05d8972: Clean up code for Clippy on Rust 1.31
* f0b6fde: Move histograms from buckets to allocators
* 5aaa746: Use i8 for the bucket ages
* ff158aa: Use u32 for block allocation thresholds
* e5b1919: Rework storage and use of GC statistics
* 0a762c8: Removed double zeroing of Immix histograms
* d149633: Use 1 env variable for concurrency settings
* ddb6cc2: Replace RwLock usage with parking_lot mutexes
* 8450931: Improve remapping of initialised type parameters

## 0.3.0 - November 25, 2018

* 1411241: Use Rust's Debug to format Float
* 7ac65aa: Added Array.contains?
* 6d1d704: Removed left-over traces of the Compatible marker
* 7dc4d1c: Added tests for std::hash_map
* b750f17: Fixed error generation when parsing brackets
* d95a357: Remove uninitialised params from return types
* 8cd6b79: Handle remapping of parameters for optional types
* 739a274: Added tests for std::fs::path::Path
* 93bbd5e: Fix race condition in ObjectPointer::status()
* a8df710: Fix tests for Object::mark_for_forward()
* 21b0ab0: Use seconds instead of milliseconds for timeouts
* e2b5bff: Move interpreter code out of the main loop
* 2f57fa4: Fixed passing of excessive arguments
* 26f535f: Add a Foreign Function Interface for C code
* 9e33e40: Reformat some VM code using rustfmt
* 850824b: Improve support for pinning processes
* f39ce80: Rework handling of prototypes of built-in types
* c1c8f5d: The Platform instruction now returns a string
* 77fbe37: Move std::vm.panic to std::process
* 96c362b: Reduce moving of threads in std::stdio
* 1a6e4b4: Simplify setting the default thread counts
* 3d15c20: Supported nested calls to process.blocking
* b79e9e8: Expose thread pinning to the runtime
* faf02ea: Pinning of processes to threads
* 6d37d40: Fixed Clippy warning for Histogram::new
* 86e740f: Add tests for Histogram::reset()
* 8be5d5b: Use Chunk instead of Vec in Histogram
* 8f7c240: Implement Drop for Chunk
* 0e0b775: Use an f32 for heap growth factors
* 3e1c4c0: Use a u8 for configuring the number of threads
* bc674fa: Fixed formatting of Clippy attributes
* c5519cb: Run Clippy on Rust stable
* 709d928: Add support for custom sha256sum programs
* 72e16d4: Basic support for late binding of Self
* b0bc5e4: Fix process status in ProcessSuspendCurrent
* c983568: Fixed arity of implements_trait?
* bac0fe8: Use u16 for ExecutionContext.return_register
* 64e8a9c: Refactor std::range so it compiles again
* b1cb8a3: Support for type checking nested required traits
* ca7a359: Manually define Nil.new

## 0.2.5 - September 11, 2018

* 38379fc: Fixed type checking of unknown_message
* b29fdc0: Added std::test::assert.true and false
* ad529e5: Add base implementation of implements_trait?
* d7df0cc: Use parentheses for process.receive_if example
* ab8d7b3: Use trailing commas for arguments and literals

## 0.2.4 - September 08, 2018

### Compiler

* f827c13: Parse trailing blocks as arguments
* 3f038c6: Fix sending to types that implement UnknownMessage
* 9c4be37: Fix parsing of arguments without parenthesis
* 703ff73: Add support for deferred execution of blocks
* 6fffc53: Fix setting the receiver type of Send nodes
* 15713ea: Fix parsing backslashes in strings
* 5e6920e: Add support for registering panic handlers
* 79103c8: Explicitly bind receivers to blocks and bindings
* 69a7592: Added std::env for managing environment data

### Runtime

* 40cde2a: Remove throw requirement from Close.close
* 97ec45b: Use trailing blocks for unit tests
* 703ff73: Add support for deferred execution of blocks
* 8c06796: Return Array!(Path) in std::dir.list
* 0bbe33f: Added std::test::assert.no_panic
* 5f8d558: Rework std::test to use panics instead of throws
* 5d6d783: Add file separator constant to std::fs::path
* 9b61ffa: Added std::os.windows? to check Windows usage
* 2627f18: Return/take Path in std::env in more places
* 8fcd93e: Add support for testing panics
* 5e6920e: Add support for registering panic handlers
* 79103c8: Explicitly bind receivers to blocks and bindings
* 69a7592: Added std::env for managing environment data

### Virtual machine

* d9b6f07: Remove source level document of instructions
* 6990ca9: Remove nightly workaround for VecDeque::append()
* e2ea21d: Fixed file open modes in the VM
* 04bf29d: Update Clippy settings for latest nightly
* 703ff73: Add support for deferred execution of blocks
* db00ebb: Fix Clippy offences
* 4119eab: Remove the "locals" queue from process mailboxes
* 5a502f2: Reduce memory necessary to create processes
* bb14527: Use linked lists for process mailboxes
* 9b0105e: Fix using integers when checking prototype chains
* 5e6920e: Add support for registering panic handlers
* 79103c8: Explicitly bind receivers to blocks and bindings
* 69a7592: Added std::env for managing environment data
* 57cc1f4: Work around nightly failures for the time being
* bf55645: Added --features option to IVM
* 843aa10: Add prefetching support for stable Rust
* dd5e959: make Mailbox::mailbox_pointers unsafe
* 1caa5d0: Clean up unsafe references in the interpreter loop

### Other

* 0a517f7: Emit warnings as errors in Clippy
* 843aa10: Add prefetching support for stable Rust

## 0.2.3 - August 19, 2018

### Compiler

No changes.

### Runtime

No changes.

### Virtual machine

* 02a2cbd: Rework allocations to support parallel moving

### Other

* e672582: Moved tagging/releasing code to separate scripts
* 0888635: Use a single Make task for building a release
* 5504bb7: Use VERSION not version in "make versions"
* 019b186: Added Make task for updating version files
* 9542354: Added tooling for managing a changelog

## 0.2.2 - August 13, 2018

### Compiler

No changes.

### Runtime

No changes.

### Virtual machine

No changes.

### Other

* 11ea041: Fixed error generation in the root Makefile

## 0.2.1 - August 12, 2018

### Compiler

* fb587d8: Bump version to 0.2.1
* be2eca2: Corrected the compiler version
* 7e462a2: Expose module names and paths to the runtime
* 64afd81: Always set "self" in a lambda

### Runtime

* 6c88fa0: Added tests for std::fs
* 7e462a2: Expose module names and paths to the runtime
* 19252b8: Move debugging code from std::vm to std::debug
* 2f61662: Rename Format.format to format_for_inspect
* 6ac3b65: Fixed incorrect process.status comment

### Virtual machine

* c761817: Reformat VM code using rustfmt
* 42d8aa6: Handle integer divisions by zero explicitly
* fab3805: Fixed two Clippy warnings in interpreter

### Other

* 107dbc5: Handle Make errors in realpath.sh
* 52bd9a0: Use make -C instead of cd'ing into a directory
* 4106928: Enable rustfmt in CI
* c396f15: Run Clippy in CI
