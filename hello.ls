(Builtin.local strings
	(Builtin.Map.from_flat_list (list
			"hello" "world"
			"hi" "jeff"
			"good" "bye"
	) )
)

(local var 1 )
(if '( (= $.var 1) )' '(
	(print "true" )
)' else '(
	(print "false" )
)' )

! (print "Hello, %s" $.strings.hello )
