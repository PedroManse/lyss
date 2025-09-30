#(Builtin.local strings
#	(Builtin.Map.from_flat_list (list
#			"hello" "world"
#			"hi" "jeff"
#			"good" "bye"
#	) )
#)
#
#(Builtin.local var 1 )
#(if '( (= $.var 1) )' '(
#	(print "true" )
#)' else '(
#	(print "false" )
#)' )

(Builtin.local hello "uwu" )
(Builtin.print "Hello, " $.hello "\n" )
