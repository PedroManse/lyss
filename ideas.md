# Values
Atom :: Tuple with function, then arguments
Value :: DefinitionPath | Literal | Instance
DefinitionPath :: Immutable, Maybe Indexed, Shared Record={ Key -> Value }?[Idx]
Literal :: Int, String, ...

# Syntax
`(return ())` -> Returns result of first atom
`(if () () else ())` -> If first atom returns true, execute and become first atom, otherwise, execute and become second atom
`! (macro)` -> Reads next atom as a macro invocation

