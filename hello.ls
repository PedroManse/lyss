#(Builtin.local strings
#	(Builtin.Map.from_flat_list (list
#			"hello" "world"
#			"hi" "jeff"
#			"good" "bye"
#	) )
#)

(Builtin.local $.var 1 )
(if '( (Builtin.Math.= $.var 1 ) )' '(
	(Builtin.print "true\n" )
)' else '(
	(Builtin.print "false" )
)' )

(Builtin.local $.hello "uwu" )
(Builtin.print "Hello, " $.hello "\n" )
