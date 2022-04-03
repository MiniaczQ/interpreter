# Język
Język będzie służył do operacji na tablicach.

## Główne założenia
 - Silnie typowany
 - Statycznie typowany
 - Zmienne są mutowalne
 - Zmienne to (mutowalne) referencje
 - Jednowątkowy, synchroniczny
 - Zmienne widoczne tylko w blokach kodu i ich zagnieżdżeniach
 - Prawie wszystko jest wyrażeniem

## Komentarze
Jednoliniowe `// ...`

i wieloliniowe `/* ... */`

zostaną wykorzystane w przykładowych kawałkach kodu.

## Typy
Zawierać będzie 4 typy podstawowe: `bool`, `int`, `float`, `string`
oraz ich tablicowe warianty: `bool[]`, `int[]`, `float[]`, `string[]`.

W implementacji zaistnieje też typ `none`.
Posłuży on do realizacji typów wyrażeń,
których nie można przypisać do zmiennych.

## Wyrażenia
Wszystko oprócz definicji funkcji jest wyrażeniem.
Znak `;` kończy wyrażenia.

### Bloki kodu
Przyjmują wartość ostatniego wyrażenia, np.
```
{
    let x: int = bar();
    10
}
```
ma typ `int` i wartość `10` oraz może być przypisany do zmiennej.

Typ wyrażenia
```
{
    bar();
}
```
to `none` i nie ma on wartości, czyli nie da się go przypisać do zmiennej.

### Deklaracje zmiennych
Jest wyrażeniem, które zawsze ma typ `none`, czyli nie da się go przypisać do zmiennej.
```
let x: int = 10;
```

### Przypisanie do zmiennych
Jest wyrażeniem, które przyjmuje wartość prawej strony przypisania.
```
x = 10
```
ma wartość 10.

Na przykład:
```
let y: int = x = 10;
```
to poprawne wyrażenie.

### Wyrażenie warunkowe
W przypadku pary `if` `else` przyjmuje wartość bloku gałęzi, który został wykonany.
```
let x: int = if y < 3 {
    1
} else {
    20
};
```
W przypadku samego `if` ma typ `none`.
```
if y < 5 {
    foo();
};
```

### Operatory
Wszystkie operacje arytmetyczne i logiczne 

## Operacje
Bazuje na ideii `list comperhension` z języka Python,
czyli pozwala na wyrażenia w stylu `[x + 1 for x in xs]`.

co oznacza, że
`if foo() {true} else {false}`
lub
`{foo(); bar()}`
może być przypisane do zmiennej.

# Sposób wykonania
Wykorzystam język Rust, edytor VSC.

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