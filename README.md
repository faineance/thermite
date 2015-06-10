# vmachine [![Build Status](https://travis-ci.org/faineance/vmachine.svg)](https://travis-ci.org/faineance/vmachine)
A virtual machine 

## Instruction Set
Where s1 is the TOS and s2 is the element on the stack below the TOS.

| Instruction | Usage     | Function                                                                  |
|-------------|-----------|---------------------------------------------------------------------------|
| out         | out       | prints top of stack                                                       |
| psh         | psh n     | pushes <n> onto the top of the stack                                      |
| pop         | pop       | pops the top of the stack                                                 |
| add         | add       | replaces top two elements of the stack with their sum (s2 + s1)           |
| sub         | sub       | replaces top two elements of the stack with their difference (s2 + s1)    |
| mul         | mul       | replaces top two elements of the stack with their product (s2 * s1)       |
| div         | div       | replaces top two elements of the stack with their product (s2 / s1)       |
| set         | set r  n  | sets register <r> to value <n>                                            |
| mov         | mov r1 r2 | moves the value in register <r1> to register <r2>                         |
| jmp         | jmp i     | jump to the location <i> if not pop stack and continue                    |
| jz          | jz  i     | jump if TOS is zero to the location  <i\> if not pop stack and continue   |
| jnz         | jnz i     | jump if TOS is nonzero to the location <i\> if not pop stack and continue |
| hlt         | hlt       | halt the program                                                          |
| nop         | nop       | do nothing                                                                |

## License
Vmachine is licensed under the [MIT License](/LICENSE).
