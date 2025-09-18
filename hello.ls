(BT.defn index_array
	(list (T.int "idx") (T.array T.any "arr") )
	(list (T.maybe any) )
	('
		(return (if (>= (Array.len $arr ) $idx  ) ('
			(BT.None )
		') else ('
			(BT.Some (BT.index_array $idx $arr ) )
		')) )
	')
)


(defn! index_araray )

(defn println! [ "str" ] ('
	(print str)
	(print "\n")
))

(defn if ('
	(BT.index_array (array _2 _3)( BT.bool_to_int _1 ))
))

(defn unwrap! ('
	(if (is_ok _1) (
		BT.unwrap_result _1
	) (

	))
))

((' print _1) "hello" )
(unwrap! )

(print "hello")
