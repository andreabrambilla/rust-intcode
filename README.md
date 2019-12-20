A simple bytecode interpreter, inspired by the the Advent of Code

Integer codes implemented so far:
- `1`(`arg0 arg1 arg2`): sums the values referred to by `arg0` and `arg1` and writes it to the location referred to by `arg2`
- `2`(`arg0 arg1 arg2`): multiplies the values referred to by `arg0` and `arg1` and writes it to the location referred to by `arg2`
- `3`(`arg0`): reads a value from the input tape and stores it in the location referred to by `args0`
- `4`(`arg0`): writes to the output tape the value referred to by `args0`
- `5`(`arg0 arg1`): if `arg0` is not zero, jumps to `arg1`
- `6`(`arg0 arg1`): if `arg0` is zero, jumps to `arg1`
- `7`(`arg0 arg1 arg2`): writes `1` to the location referred to by `arg2` if `arg0` is less than `arg1`, `0` otherwise
- `8`(`arg0 arg1 arg2`): writes `1` to the location referred to by `arg2` if `arg0` is equal to `arg1`, `0` otherwise
- `99`: terminate execution

Integer codes are assumed to have 5 digits. The last two digits identifies the operation as described as above.
The first, second and third digits determine the _mode_ of the first second and third arguments:
- `0`: read the value from the location given by the argument
- `1`: immediate mode, the argument is the value to be used in the operation
If an argument refers to a store location (eg, the third argument for code `1`, or the first one for code `5`), the argument cannot be in imediate mode.
