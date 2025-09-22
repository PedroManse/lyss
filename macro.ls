# Code block ('')
# Macro block (``)

(local name "pedro" ) # set variable $name = "pedro"
(extern "last_name" "manse") # set variable $last_name = "manse" # notice it takes a string with the variable's name

(set BT Builtints )
(set T BT.Types ) # define T to = BT.Types
(set Mi BT.Macro.Composers.Default.Inputs)
(set . BT ) # special deinition, import every property of BT

(map!
	one ( 1 )
	two ( 2 )
	three ( 3 )
	four ( 4 )
)

(BT.Macro.def "defn!"
	(BT.Macro.Composers.Default.make
		(list
			(Mi.ident "fn_name" )
			(Mi.literal "[" )
			(Mi.many "inputs"
				(list
					(Mi.ident "input" )
					(Mi.maybe (Mi.type_atom "input_type" ) )
				)
			)
			(Mi.literal "]" )
			(Mi.maybe (Mi.type_atom "fn_out_type" ) )
			(Mi.macro_atom "code" )
		)
		('
			(local typed_inputs )
			(local typed_output (
				(if (Maybe.is_some $fn_out_type) ('
					(Maybe.unwrap $fn_out_type)
				') else ('
					(list (T.any))
				'))
			))
		')
		(`
			(Builtints.defn $fn_name $typed_inputs $typed_output (' ($code ) ') )
		`)
	)
)

(defn "index_array"
	(list (T.int "idx" ) (T.array T.any "arr" ) )
	(T.maybe T.any )
	('
		(return (if (>= (Array.len $arr ) $idx  ) ('
			(Maybe.none )
		') else ('
			(Maybe.some (Array.index $idx $arr ) )
		')) )
	')
)

! (defn index_araray [ idx(int) arr(array) ](maybe(int)) (
		(return (if (>= (Array.len $arr ) $idx  ) ('
			(None )
		') else ('
			(Some (Array.index $idx $arr ) )
		')) )
) )

(print "Hello")
