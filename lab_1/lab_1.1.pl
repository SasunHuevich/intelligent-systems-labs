% --- Факты о видах ---
dog(flash).
dog(rover).
cat(butsy).
horse(star).

% --- Факты о цвете ---
color(flash, black).
color(butsy, brown).
color(rover, red).
color(star, white).

% --- Правила ---

% Домашнее животное — собака или кошка
pet(X) :- dog(X).
pet(X) :- cat(X).

% Животное — домашнее животное или лошадь
animal(X) :- pet(X).
animal(X) :- horse(X).

% У Тома есть собака не чёрного цвета
has(tom, X) :- dog(X), color(X, Color), Color \= black.

% У Кейта есть лошадь или что-то чёрного цвета
has(kate, X) :- horse(X).
has(kate, X) :- color(X, black).
