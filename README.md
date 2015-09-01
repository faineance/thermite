# thermite [![Build Status](https://travis-ci.org/faineance/thermite.svg)](https://travis-ci.org/faineance/thermite) [![Build status](https://ci.appveyor.com/api/projects/status/fq1iw0x3xx9jqqld?svg=true)](https://ci.appveyor.com/project/faineance/thermite) [![Circle CI](https://circleci.com/gh/faineance/thermite.svg?style=svg)](https://circleci.com/gh/faineance/thermite)  [![Build Status](https://snap-ci.com/faineance/thermite/branch/master/build_image)](https://snap-ci.com/faineance/thermite/branch/master) [![Coverage Status](https://coveralls.io/repos/faineance/thermite/badge.svg?branch=master)] (https://coveralls.io/r/faineance/thermite?branch=master)
A virtual machine 

## Building
You'll need a pretty up to date version of rust nightly.

```cargo build``` to build.

```cargo test``` to run tests.
## Usage
```cargo run repl``` to run repl.

```cargo run -- run example/factorial.vma``` to run factorial.vma.
## Instruction Set
#### Arithmetic Operations
Arithmetic operations are in the format

| ``instruction`` | ``source`` | ``target`` | ``destination`` |
|-----------------|------------|------------|---------------|

| Instruction | Usage                     | Function                                                                  |
|-------------|---------------------------|---------------------------------------------------------------------------|
| add         | add ``ra`` ``rb`` ``rc``  | set ``rc`` to the sum of ``ra`` and ``rb``                                |
| sub         | sub ``ra`` ``rb`` ``rc``  | set ``rc`` to the difference of ``ra`` and ``rb``        |
| mul         | mul ``ra`` ``rb`` ``rc``  | set ``rc`` to the product of ``ra`` and ``rb``          |
| div         | div ``ra`` ``rb`` ``rc``  | set ``rc`` to the quotient of ``ra`` and ``rb``      |
#### Register Operations
| Instruction | Usage     | Function                                                                  |
|-------------|-----------|---------------------------------------------------------------------------|
| str         | str ``6`` ``ra`` | store 6 to ``ra``                    |
#### Jump Operations
| Instruction | Usage     | Function                                                                  |
|-------------|-----------|---------------------------------------------------------------------------|
| jmp         | jmp ``label``     | jump to  ``label``                     |
| jz          | jz  ``ra`` ``label``     | jump if ``ra`` is zero to ``label``    |
| jnz         | jnz ``ra`` ``label``     | jump if ``ra`` is nonzero to ``label``  |
#### Other Operations
| Instruction | Usage     | Function                                                                  |
|-------------|-----------|---------------------------------------------------------------------------|
| out         | out ``ra``       | print the contents of ``ra``                                                       |
| hlt         | hlt       | halt the program                                                          |
| nop         | nop       | do nothing                                                                |

## License
thermite is licensed under the [MIT License](/LICENSE).
