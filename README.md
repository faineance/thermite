# thermite [![Build Status](https://travis-ci.org/faineance/thermite.svg)](https://travis-ci.org/faineance/thermite) [![Build status](https://ci.appveyor.com/api/projects/status/fq1iw0x3xx9jqqld?svg=true)](https://ci.appveyor.com/project/faineance/thermite) [![Circle CI](https://circleci.com/gh/faineance/thermite.svg?style=svg)](https://circleci.com/gh/faineance/thermite)  [![Build Status](https://snap-ci.com/faineance/thermite/branch/master/build_image)](https://snap-ci.com/faineance/thermite/branch/master) [![Coverage Status](https://coveralls.io/repos/faineance/thermite/badge.svg?branch=master)] (https://coveralls.io/r/faineance/thermite?branch=master)
A virtual machine 

## Building
You'll need an up to date version of rust nightly to compile.

```cargo build``` to build.

```cargo test``` to run tests.
## Usage
```cargo run repl``` to run repl.

```cargo run -- run example/factorial.vma``` to run factorial.vma.
## Instruction Set
#### Arithmetic Operations
Arithmetic operations are in the format:

| ``opcode`` | ``source`` | ``target`` | ``destination`` |
|-----------------|------------|------------|---------------|

| Opcode | Usage                     | Function                                                                  |
|-------------|---------------------------|---------------------------------------------------------------------------|
| add         | add ``ra`` ``rb`` ``rc``  | **``rc = ra + rb``**      |
| sub         | sub ``ra`` ``rb`` ``rc``  | **``rc = ra - rb``**      |
| mul         | mul ``ra`` ``rb`` ``rc``  | **``rc = ra * rb``**        |
| div         | div ``ra`` ``rb`` ``rc``  | **``rc = ra / rb``**   |
| max         | max ``ra`` ``rb`` ``rc``  | **``rc = max(ra, rb)``**      |
| min         | min ``ra`` ``rb`` ``rc``  | **``rc = min(ra, rb)``**     |

#### Bitwise Operations
Bitwise operations are in the same format as arithmatic.

| Opcode | Usage                     | Function                                                                  |
|-------------|---------------------------|---------------------------------------------------------------------------|
| and         | and ``ra`` ``rb`` ``rc`` |  **``rc = ra & rb``**               |
| or          | or ``ra`` ``rb`` ``rc`` |  **``rc = ra | rb``**      |
| xor         | xor ``ra`` ``rb`` ``rc`` |  **``rc = ra ^ rb``**       |
| shl         | shl ``ra`` ``rb`` ``rc`` |  **``rc = ra << rb``**                       |
| shr         | shr ``ra`` ``rb`` ``rc`` |  **``rc = ra >> rb`` (arithmetic)**         |

#### Branching Operations
| Instruction | Usage     | Function                                                                  |
|-------------|-----------|---------------------------------------------------------------------------|
| jmp         | jmp ``label``     | **``ip = label``**                    |
| jz          | jz  ``ra`` ``label``     | **`` if (ra == 0) ip = label``**    |
| jnz         | jnz ``ra`` ``label``     | **`` if (ra != 0) ip = label``**   |

#### Assignment Operations
| Instruction | Usage     | Function                                                                  |
|-------------|-----------|---------------------------------------------------------------------------|
| str         | str ``6`` ``ra`` | **``ra = 6``**                     |
| cpy         | cpy ``rb`` ``ra`` | **``ra = rb``**                     |

#### IO Operations
| Opcode | Usage                     | Function                                                                  |
|-------------|---------------------------|---------------------------------------------------------------------------|
| out         | out ``ra``   | print to sdout **``ra``**     |
| in         | in ``ra``   | store stdin to **``ra``**      |

#### Other Operations
| Instruction | Usage     | Function                                                                  |
|-------------|-----------|---------------------------------------------------------------------------|
| hlt         | hlt       | halt the program                                                          |
| nop         | nop       | do nothing                                                                |

## License
thermite is licensed under the [MIT License](/LICENSE).
