#(Builtin.local strings
#	(Builtin.Map.from_flat_list (list
#			"hello" "world"
#			"hi" "jeff"
#			"good" "bye"
#	) )
#)

(Builtin.scope Builtin)
(Builtin.alias Builtin.Math $M )

(local $.var 1 )
(if '( ($M.= $.var 1 ) )' '(
	(print "true\n" )
)' else '(
	(print "false\n" )
)' )

(local $.hello "uwu" )
(print "Hello, " $.hello "\n" )
