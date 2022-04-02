# Idea języka
Język będzie służył do operacji na tablicach.
Bazuje na ideii `list comperhension` z języka Python.

# Założenia TODO
## Języka
 - Silnie typowany
 - Statycznie typowany
 - Zmienne mutowalne
 - Jednowątkowy, synchroniczny
 - Bezstosowy

# Składnia TODO
## Deklaracja zmiennej i przypisanie wartości
let <IDENTIFIER>: <IDENTIFIER> = <EXPRESSION>;
<IDENTIFIER> = <EXPRESSION>;

## Pętle
for <IDENTIFIER> in <EXPRESSION> {<CODE_BLOCK>}

# Sposób wykonania
Wykorzystam język Rust

# Obsługiwane wejścia
Język pozwoli na interpretacje pliku lub strumienia wejściowego w formacie utf-8.

# Uruchomienie
Język będzie uruchamiany z wiersza poleceń.
Uruchomienie bez flag wyświetli informacje pomocnicze.
Uruchomienie z flagą `-f <path>` wykorzysta podany plik jako wejście.
Uruchomienie z flagą `-i` wykorzysta strumień wejściowy procesu.

# Obsługa błędów statyczna
Błędy nie będą ignorowane w czasie analizy (leksykalnej, składniowej),
ale ich wystąpienie uniemożliwi wykonanie programu.
Wiadomości o wszystkich błędach będą wypisywane do strumienia błędów.

# Obsługa błędów dynamiczna
Wystąpienie błędu w czasie wykonania kodu, np.:
 - overflow
 - dzielenie przez 0
 - błąd zamiany typu
 - dostęp do nieistniejącego indeksu

spowoduje zatrzymanie wykonania i zwrócenie komunikatu o błędzie do strumienia błędów.

# Testy
Automatyczne z wykorzystaniem wbudowanej architektury języka Rust.
Testowanie działania poprawnego i obsługi błędów (testy nieprzechodzące).

Testy jednostkowe:
 - wykrywanie pojedynczych tokenów
 - wykrywanie fragmentu składni
 - wywoływanie pojedynczych tokenów

Testy akceptacyjne:
 - wykrywanie serii tokenów
 - wykrywanie całego drzewa składniowego

Testy wdrożeniowe:
 - wykonanie przykładowych fragmentów kodu