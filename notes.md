## Verilog Number Parsing

Might need to create rust AST type as for verilog number parsing. I am starting to
feel that defining the PEST syntax and writing manual logic to convert the resulting
parse Pairs into internal bit representation is more difficult than first converting
parse Pairs to internal AST and then converting that to internal bit representation.
Many fields in the verilog number syntax are optional and it is not clear to me how
to write the parsign logic using Pairs iterators when some fields are missing.