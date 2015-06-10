# vmachine [![Build Status](https://travis-ci.org/faineance/vmachine.svg)](https://travis-ci.org/faineance/vmachine)
A virtual machine 

## Instruction Set
#### Stack Operations
Where ``s1`` is the TOS and ``s2`` is the element on the stack below the TOS.

| Instruction | Usage     | Function                                                                  |
|-------------|-----------|---------------------------------------------------------------------------|
| psh         | psh ``n`` | push ``n`` onto the top of the stack                                    |
| pop         | pop       | pop the top of the stack                                                 |
| add         | add       | replace top two elements of the stack with their sum (``s2 + s1``)       |
| sub         | sub       | replace top two elements of the stack with their difference (``s2 - s1``)|
| mul         | mul       | replace top two elements of the stack with their product (``s2 * s1``)   |
| div         | div       | replace top two elements of the stack with their product (``s2 / s1``)   |
#### Register Operations
| Instruction | Usage     | Function                                                                  |
|-------------|-----------|---------------------------------------------------------------------------|
| set         | set ``r``  ``n``  | set register ``r`` to value ``n``                                            |
| mov         | mov ``r1`` ``r2`` | move the value in register ``r1`` to register ``r2``                      |
#### Jump Operations
| Instruction | Usage     | Function                                                                  |
|-------------|-----------|---------------------------------------------------------------------------|
| jmp         | jmp i     | jump to the location ``<i>`` if not pop stack and continue                    |
| jz          | jz  i     | jump if TOS is zero to the location  ``<i>`` if not pop stack and continue   |
| jnz         | jnz i     | jump if TOS is nonzero to the location ``<i>`` if not pop stack and continue |
#### Other Operations
| Instruction | Usage     | Function                                                                  |
|-------------|-----------|---------------------------------------------------------------------------|
| out         | out       | print top of stack                                                       |
| hlt         | hlt       | halt the program                                                          |
| nop         | nop       | do nothing                                                                |

## License
Vmachine is licensed under the [MIT License](/LICENSE).
