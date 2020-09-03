:- use_module(library(iso_ext)).
:- use_module(library(format)).

game_loop(Input, Output) :-
    read_term(Input, Term, []),
    ( Term = end_of_file -> (close(Output), true)
    ; !,
      forall(call(Term), portray_clause(Output, Term)),
      game_loop(Input, Output)
    ).

engine :-
    argv([InPath, OutPath]),
    atom_chars(InFile, InPath),
    atom_chars(OutFile, OutPath),
    open(InFile, read, Input),
    open(OutFile, write, Output),
    game_loop(Input, Output),
    !.

:- initialization(engine).
