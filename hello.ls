# Code block ('')
# Macro block (``)

# Definition :: Named Object={ [Named Properties]: Values }
# Value :: Definition | Literal
# Literal :: Number, String, ...
(local name "pedro" ) # set variable $name = "pedro"
(extern "last_name" "manse") # set variable $last_name = "manse" # notice it takes a string with the variable's name

(set BT Builtints )
(set T BT.Types ) # define T to = BT.Types
(set Mi BT.Macro.Composer.Inputs)
(set . BT ) # special deinition, import every property of BT

(map!
	( one . 1 )
	( two . 2 )
	( three . 3 )
	( four . 4 )
)

(BT.Macro.def "defn!"
	(BT.Macro.Composer.simple
		(list
			(Mi.ident "fn_name" )
			(Mi.literal "[" )
			(Mi.many "inputs"
				(list
					(Mi.ident "input" )
					(Mi.maybe (Mi.type "input_type" ) )
				)
			)
			(Mi.literal "]" )
			(Mi.maybe (Mi.type "fn_out_type" ) )
			(Mi.atom "code" )
		)
		('
			(local typed_inputs ...)
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
	(list (T.maybe T.any ) )
	('
		(return (if (>= (Array.len $arr ) $idx  ) ('
			(None )
		') else ('
			(Some (Array.index $idx $arr ) )
		')) )
	')
)

(defn! index_araray [ idx(int) arr(array) ](maybe int) (
		(return (if (>= (Array.len $arr ) $idx  ) ('
			(None )
		') else ('
			(Some (Array.index $idx $arr ) )
		')) )
) )

