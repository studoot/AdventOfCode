This problem was solved by hand, symbolically executing the code in the input program (see [the spreadsheet containing the working](./Book1.xlsx)), yielding the following equalities that define constraints for the valid values of version number digits:

* `d3` == `d2` - 7
* `d7` == `d6` - 5
* `d9` == `d8` + 7
* `d10` == `d5` - 6
* `d11` == `d4`
* `d12` == `d1` + 3
* `d13` == `d0` - 3

In order to maximize the version number (part 1), `d0`, `d2`, `d4`, `d5`, `d6`, `d9`, `d11` and `d12` must be set to '9' (with the equalities yielding the other digit values), while to minimize the version number, `d1`, `d3`, `d4`, `d7`, `d8`, `d10`, `d11` and `d13` must be set to 1 (again with the other digit values being derived from the equalities).