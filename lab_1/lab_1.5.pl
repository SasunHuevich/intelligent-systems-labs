% --- Факты о поле ---
female(pam).
female(lis).
female(ann).

male(tom).
male(bob).
male(pat).
male(jim).

% --- Факты о родителе ---
parent(pam, bob).
parent(tom, bob).
parent(tom, lis).
parent(bob, ann).
parent(bob, pat).
parent(pat, jim).

% --- Правила ---

% Ребёнок - обратное от родителя
child(Y, X) :- parent(X, Y).

% --- Родитель родителя ---
grandparent(G, C) :- parent(G, P), parent(P, C).

% --- Потомки ---
child_root(A, X) :- child(A, X).
child_root(A, X) :- child(P, X), child_root(A, P).

% --- Мать ---
mother(M, C) :- parent(M, C), female(M).