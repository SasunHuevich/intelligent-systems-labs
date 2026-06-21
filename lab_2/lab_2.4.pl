% --- Факты: словарь английский -> французский ---
translate_word(hello, bonjour).
translate_word(world, monde).
translate_word(dog, chien).
translate_word(cat, chat).
translate_word(i, je).
translate_word(love, aime).
translate_word(you, tu).

% --- Правила ---

% Перевод предложения — один аргумент, результат в консоль
translate(Words) :-
    translate_list(Words, Result),
    write(Result), nl.

% Пустое предложение — пустой перевод
translate_list([], []).

% Перевод списка слов по одному
translate_list([Word | Tail], [French | Result]) :-
    translate_word(Word, French),
    translate_list(Tail, Result).
