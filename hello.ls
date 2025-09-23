(Builtin.local strings
	(Builtin.Map.from_flat_list ( list
			"hello" "world"
			"hi" "jeff"
			"good" "bye"
	))
)

! (print "Hello, %s" $.strings.hello)
