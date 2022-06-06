# Język
Język służy do operacji na tablicach.
Głównymi inspiracjami były języki
 - Python - list comperhension
 - Rust - wszystko jest wyrażeniem

## Główne założenia
 - Silnie typowany
 - Dynamicznie typowany
 - Zmienne są mutowalne
 - Jednowątkowy, synchroniczny
 - Zmienne widoczne tylko w blokach kodu i ich zagnieżdżeniach
 - Prawie wszystko jest wyrażeniem

## Komentarze
Jednoliniowe `// ...`  
i wieloliniowe `/* ... */`

## Typy
Zawierać będzie 3 typy podstawowe: `bool`, `int`, `float`  
oraz typ listowy: `[]`.

Dostępny będzie też typ `string` zachowujący się jak tablica znaków.

W implementacji zaistnieją też typy `none`.  
Typ `none` posłuży do realizacji typów wyrażeń,
których nie można przypisać do zmiennych.

## Wyrażenia
Wszystko oprócz definicji funkcji jest wyrażeniem.
Znak `;` zamienia wyrażenia na instrukcje.

### Bloki kodu
Określone są w nawiasach klamrowych `{` `}`
i zawiera serię instrukcji
opcjonalnie zakończonych wyrażeniem.

Bloki kodu przyjmują wartość ostatniego wyrażenia.
```
{
    bar();
    10
}
```
ma typ `int` i wartość `10` oraz może być przypisany do zmiennej.

W przypadku braku wyrażenia
```
{
    bar();
}
```
ma typ `none` i nie ma on wartości, czyli nie da się go przypisać do zmiennej.

### Wyrażenia statyczne
Dla `bool` są to `true` lub `false`.
```
true
false
```

Dla `int` jest to zero, lub ciąg cyfr, który nie zaczyna się zerem.
```
123
0123    // niepoprawne
```

Dla `float` jest to `int` po którym występuje `.` oraz ciąg dowolnych cyfr (conajmniej jedna).
```
0.1
1.0
0.      // niepoprawne
```

Dla `string` jest to tekst zawarty w cytaty.
Może być wielo-linijkowy.
Możliwa jest ucieczka za pomocą znaku `\`.
```
"Hello world!"
"Hello
world!"
"Escape quotation\""
"Escape the escape\\"
```

### Deklaracje zmiennych
Zaczyna się od słowa `let`,
następnie podana jest nazwa zmiennej oraz jej typ,
oddzielony operatorem `:`.

Zmienna zawsze musi zostać zainicjowana,
więc od razu następuje przypisanie wartości.
```
let x: int = 10;
```

Deklaracja jest wyrażeniem, które zwraca wartość prawej strony,
czyli da się wykonać kilka deklaracji w następujący sposób:
```
let y: int = let x: int = 10
```

### Przypisanie do zmiennych
Zaczyna się od nazwy zmiennej,
a następnie operatora przypisania `=` i wartość odpowiedniego typu.

Przypisanie jest wyrażeniem, które przyjmuje wartość prawej strony przypisania.
```
x = -10
```
ma wartość `-10`.

```
let y: int = x = 10;
```
to poprawne wyrażenie.

Przykład przypisania stałej do typu listowego
```
xs = [1, 2, 3, 4, 7];
```

Dla listy `string`
```
let str: string = "Hello world!";
```
```
let str: string = "Hello
world!";
```

### Wyrażenie warunkowe
Zawiera słowo kluczowe `if`,
predykat typu `bool`,
następnie blok kodu.  
Opcjonalnie po bloku kodu można użyć słowa kluczowego `else`
oraz kolejnego bloku kodu.

W przypadku pary `if` `else` przyjmuje wartość bloku kodu gałęzi, która została wykonany.
```
let x: int = if y < 3 {
    1
} else {
    20
};
```
W przypadku samego `if` jest podobnie, wyrażenie przyjmie tutaj typ `none`, bo blok kodu kończy się `;`.
```
if y < 5 {
    foo();
};
```

### Operatory unarne
 - `int`, `float`  
    Operator `-` zwraca oryginalny typ

 - `bool`  
    Operator `!` zwraca typ `bool`

### Operatory binarne
 - `int`  
    Operatory `+`, `-`, `*`, `/`, `%` zwracają typ `int`.  
    Operatory `==`, `<`, `<=`, `>`, `>=` zwracają typ `bool`.

 - `float`  
    Operatory `+`, `-`, `*`, `/` zwracają typ `float`.  
    Operatory `==`, `<`, `<=`, `>`, `>=` zwracają typ `bool`.

 - `bool`  
   Operatory `==`, `!=`, `|`, `&` zwracają typ `bool`

### Operatory na listach
 - `[]`  
    Wykonują operacje na elementach list.  
    W przypadku operatorów binarnych wynikowa lista ma długośc większej listy wejściowej,
    a elementy bez pary nie zmieniają wartości.  
    Operator `[i]` zwraca pojedynczy element pod indeksem `a` (który jest wyrażeniem typu `int`).

 - `string`
    Operator `[i]`, który zwróci `string` z pojedynczym znakiem z danego indeksu.  
    Operator `+`, który zwraca konkatenacje łańcuchów wejściowych.  
    Operator `==`, `!=`, który zwraca `bool`

Operator `[a..b]`, który zwraca listę o elementach od indeksu `a` do indeksu `b` (które są wyrażeniami typu `int`).

### Nawiasy
Operacje mogą być zawarte w nawiasach `(` `)`, aby wymusić inny priorytet wykonania.

### Pętle
Dostępne są 2 typy.

Pętla 'dopóki' zaczyna się od słowa kluczowego `while`,  
następnie podany jest wyrażenie o typie `bool`,  
a na koniec blok kodu.

Pętla 'dla' zaczyna się od słowa kluczowego `for`,  
następnie podana jest nazwa zmiennej,
która przyjmie wartości kolejnych elementów listy,  
po niej wystąpi słowo kluczowe `in`
oraz wyrażenie o typie listy (ale nie `string`, bo nie posiada typu pojedynczego),  
na końcu jest blok kodu.

Pętle zwracają listę o ile typ zwrotny bloku jest typem bazowym: `int`, `float`, `bool`.
```
let a: int = 5;
let b: int = 10;

let a_b_range: [] = while a < b {
    a = a + 1;
    a
};
```
```
let xs: [] = 

let incremented_xs: [] = for x in xs {
    x + 1
};
```

### Wywołania funkcji
Mają postać identyfikatora, a następnie `(` `)`, w których zawarte są argumenty.

Naturalnie przyjmują wartość obliczoną z wywołania funkcji.
Jeżeli funkcja nie definiowała typu zwrotnego, to zwraca typ `none`.
```
fn foo() -> int {
    10
}

fn main() {
    let x: int = foo();
}
```
```
fn bar() {
    foo();
}

fn main() {
    let x: ??? = bar();  // Nie można przypisać, bo typ `none`
}
```

### Return
Słowo kluczowe `return` jest wyrażeniem, które zawsze zwraca `none`.
Więcej o nim później.

## Definicja funkcji
### Postać
Zaczyna się od słowa kluczowego `fn`,
a następnie nazwy funkcji.  
Potem w nawiasach `(` `)` podane są parametry oddzielone przecinkiem.  
Każdy parametr to jego nazwa oraz typ przedzielone `:`.  
Po parametrach możliwe jest dodanie typu zwrotnego po operatorze `->`.  
Na końcu podane jest ciało funkcji w postaci bloku kodu.

Przykładowe definicje funkcji:
```
fn negative_together(
    x: int,
    y: int
) -> bool {
    let z: int = x + y;
    z < 0
}
```
```
fn do_nothing() {}
```
```
fn print_but_dont_return(x: int) {
    print(x);
}
```

### Funkcje
Może istnieć tylko jedna funkcja o danej nazwie, wyjątkami są funkcje wbudowane:
```
print(0)
print(1.0)
print(true)
print("Hello world!", "Hello again!")
```

### Instrukcja `return`
Funkcja zwróci wartość wyrażenia bloku kodu ciała funkcji,
ale możliwe jest wcześniejsze zwrócenie wartości `x`
poprzez instrukcję `return x`.

W przypadku funkcji, które nie zwracają wartości (zwracają typ `none`)
można zastosować samo słowo kluczowe `return`.

## Kolejność operatorów
Nawiasy `(` `)` mogą wymusić inną kolejnośc.

    Priorytet       Operator/-y         Opis                    -arność     Łączność        Pozycja

    9               (a, b, ...)         wywołanie funkcji       N-nary      -               suffix
                    [a..b]              dostęp do pod-listy     Trynary     -               suffix
                    [a]                 dostęp do indeksu       Binary      -               suffix

    8               -                   negacja arytmetyczna    Unarny      -               prefix
                    !                   negacja logiczna        Unarny      -               prefix

    7               *                   mnożenie                Binarny     lewostronna     -
                    /                   dzielenie               Binarny     lewostronna     -
                    %                   reszta z dzielenia      Binarny     lewostronna     -
    
    6               +                   dodawanie               Binarny     lewostronna     -
                    -                   odejmowanie             Binarny     lewostronna     -
                
    5               ==                  równość                 Binarny     lewostronna     -
                    !=                  nierówność              Binarny     lewostronna     -
                    <                   mniejszość              Binarny     lewostronna     -
                    <=                  mniejszość lub równość  Binarny     lewostronna     -
                    >                   większość               Binarny     lewostronna     -
                    >=                  większośc lub równość   Binarny     lewostronna     -
    
    4               &                   koniunkcja logiczna     Binarny     lewostronna     -

    3               |                   alternatywa logiczna    Binarny     lewostronna     -

    2               =                   przypisanie wartości    Binarny     prawostronna    -

    1               return              wyjście z funkcji       Unarny      -               prefix
                    let                 deklaracja zmiennej     Binarny     prawostronna    -
   
## Biblioteka standardowa
Zaoferuje metody:
 - `print(arg1)`  
    wypisze wartość argumentu do strumienia wyjściowego
 - `cast_int(arg1)`, `cast_float(arg1)`, `cast_bool(arg1)`, `cast_string(arg1)`  
    spróbuje zamienić wartość jednego typu na drugi
 - `push(arg1, arg2)`  
    doda element do końca listy i ją zwróci
    (poprawne tylko dla arg1 typu `[]`)
 - `length(arg1)`  
    zwróci długość
    (poprawne tylko dla argumentów typów `[]`, `string`)

## Punkt wejściowy
Język jako pierwszą wywoła funkcję `main`,
która nie przyjmuje żadnych argumentów
i nie zwraca żadnej wartości.

# Sposób wykonania
Wykorzystam język Rust z paczką `utf8-chars`,
która pozwala na czytanie pojedynczych znaków utf-8 z bufora,
w celu realizacji skanera.

Projekt będzie podzielony na kilka modułów,
każdy będący w stanie działać niezależnie,
co ułatwi tworzenie testów.  
Moduły, które na pewno się znajdą to:
 - analizator leksykalny
 - analizator składniowy
 - interpreter

# Obsługiwane wejścia
Język pozwoli na interpretacje pliku lub strumienia wejściowego w formacie utf-8.

Obsługa wielu plików może być łatwo zaimplementowana,
ale taka naiwna implementacja pogorszyłaby czytelność języka.  
(funkcja `main` tylko w jednym pliku,
nie wiadomo gdzie zdefiniowane zostały funkcje)

# Uruchomienie
Język będzie uruchamiany z wiersza poleceń.  
Uruchomienie bez flag wyświetli informacje pomocnicze.  
Uruchomienie z flagą `-f <path>` wykorzysta podany plik jako wejście.  
Uruchomienie z flagą `-i` wykorzysta strumień wejściowy procesu.

# Obsługa błędów statyczna
Błędy będą ignorowane w czasie analizy (leksykalnej, składniowej), ale ich wystąpienie uniemożliwi wykonanie programu.
Wiadomości o wszystkich błędach zostaną wypisane do strumienia błędów.

# Obsługa błędów dynamiczna
Wystąpienie błędu w czasie wykonania kodu, np.:
 - dzielenie przez 0
 - błąd zamiany typu (np. `string` w `float`)
 - dostęp do nieistniejącego indeksu, zmiennej

spowoduje zatrzymanie wykonania i zwrócenie komunikatu o błędzie do strumienia błędów.

# Testy
Automatyczne z wykorzystaniem wbudowanej architektury języka Rust, `cargo test`.
Testowanie działania poprawnego i obsługi błędów.

Testy jednostkowe:
 - wykrywanie pojedynczych tokenów
 - wykrywanie fragmentu składni
 - wywoływanie pojedynczych tokenów

Testy integracyjne:
 - wykrywanie serii tokenów
 - wykrywanie całego drzewa składniowego

Testy akceptacyjne:
 - wykonanie przykładowych fragmentów kodu
