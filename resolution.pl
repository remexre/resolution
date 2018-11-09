% This was the first prototype, done mostly experimentally in a couple hours with Dalton Hildreth.

sym(A, [A|_], p).
sym(A, [not(A)|_], n).
sym(A, [_|T], S) :- sym(A, T, S).

symOpp(A, X, Y) :- sym(A, X, p), sym(A, Y, n).
symOpp(A, X, Y) :- sym(A, X, n), sym(A, Y, p).

without(_, [], []).
without(A, [A|T], T2) :- !, without(A, T, T2).
without(A, [not(A)|T], T2) :- !, without(A, T, T2).
% Not really sound for non-ground terms, but it doesn't matter.
without(A, [B|T], [B|T2]) :-
	B \= A,
	B \= not(A),
	!,
	without(A, T, T2).

ann(A, X, Y, Z) :-
	symOpp(A, X, Y),
	without(A, X, X2), without(A, Y, Y2),
	simp(X2, Y2, Z).

simp(X, Y, true) :- symOpp(_, X, Y), !.
simp(true, Y, Y) :- !.
simp(X, true, X) :- !.
simp(X, Y, Z) :- union(X, Y, Z).

assumeFalse([]).
assumeFalse([A|T]) :-
	atom(A),
	assert(provable([not(A)], assumption)),
	assumeFalse(T).
assumeFalse([not(A)|T]) :-
	assert(provable([A], assumption)),
	assumeFalse(T).

provable(X, kb) :- kb(X).

combine(X, Y, Z) :- ann(_, X, Y, Z), !.
combine(X, Y, Z) :- union(X, Y, Z).

forwardChain(Result) :-
	provable(X, _), provable(Y, _),
	combine(X, Y, Z),
	writeSequent(X, Y, Z),
	assert(provable(Z, X-Y)),
	% We don't check if we actually got something useful until here, so we're
	% actually searching rather than checking (i.e. forward rather than
	% backward chaining).
	Z = Result.

kb([not(a), b, c, not(d)]).
kb([b, not(c), d]).
kb([a, b, not(d)]).
kb([a, not(b), c]).
kb([a, not(c), not(e)]).
kb([not(a), c]).
kb([not(b), d]).

tlProve(Ds) :- assumeFalse(Ds), forwardChain([]).

str(A, A1, A2) :- atom_concat(A1, A, A2).
strChs(_, 0, A, A).
strChs(C, X, A1, A2) :- !, X > 0, str(C, A1, Tmp), X2 is X - 1, strChs(C, X2, Tmp, A2).
strExpr(not(A)) --> !, str('!'), str(A).
strExpr(A) --> str(A).
strDisj([]) --> str('F').
strDisj([X]) --> strExpr(X).
strDisj([H|T]) --> str('('), strExpr(H), str(' V '), strDisjTl(T).
strDisjTl([X]) --> strExpr(X), str(')').
strDisjTl([H|T]) --> strExpr(H), str(' V '), strDisjTl(T).
strSequent(X, Y, Z) -->
	{
	strDisj(X, '', AX), atom_length(AX, LX),
	strDisj(Y, '', AY), atom_length(AY, LY),
	strDisj(Z, '', AZ), atom_length(AZ, LZ),
	L is LX + LY + 3,
	P is (L - LZ) // 2
	},
	strDisj(X), strChs(' ', 3), strDisj(Y), str('\n'),
	strChs('-', L), str('\n'),
	strChs(' ', P), strDisj(Z), str('\n\n').
writeSequent(X, Y, Z) :- strSequent(X, Y, Z, '', A), write(A).
