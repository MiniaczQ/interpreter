# Język
Język służy do operacji na tablicach.
Głównymi inspiracjami były języki
 - Python - list comperhension
 - Rust - wszystko jest wyrażeniem

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

## Typy
Zawierać będzie 3 typy podstawowe: `bool`, `int`, `float`  
oraz ich tablicowe warianty: `bool[]`, `int[]`, `float[]`.

Dostępny będzie też typ `string` zachowujący się jak tablica znaków.

W implementacji zaistnieją też typy `none` i `any`.  
Typ `none` posłuży do realizacji typów wyrażeń,
których nie można przypisać do zmiennych.
Z kolei `any` zostanie wykorzystywany w funkcjach
standardowych do obsługi różnych typów argumentów.

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

Deklaracja jest wyrażeniem, które zawsze ma typ `none`,
czyli nie da się go przypisać do zmiennej.
```
let y: int = let x: int = 10
```
nie jest poprawne

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

Przykład przypisania stałej do typu listowego (nie `string`)
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
wyrażenie o typie `bool`,
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
W przypadku samego `if` wyrażenie ma typ `none`.
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
   Operatory `==`, `|`, `&` zwracają typ `bool`

### Operatory na listach
 - `int[]`, `float[]`, `bool[]`  
    Wykonują operacje na elementach list.  
    W przypadku operatorów binarnych wynikowa lista ma długośc większej listy wejściowej,
    a elementy bez pary nie zmieniają wartości.   
    Operator `[a]` zwraca pojedynczy element pod indeksem `a` (który jest wyrażeniem typu `int`).

 - `string`
    Operator `+`, który zwraca konkatenacje łańcuchów wejściowych.
    Operator `==`, który zwraca `bool`

 - Wszystkie
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

let a_b_range: int[] = while a <> b {
    a = a + 1;
    a
};
```
```
let xs: int[] = 

let incremented_xs: int[] = for x in xs {
    x + 1
};
```

### Wywołania funkcji
Mają postać 

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

### Funkcje o zmiennym typie parametrów
Może istnieć wiele funkcji o tej samej nazwie, ale różnych typach parametrów:
```
print(0)
print(1.0)
print(true)
print("Hello world!")
```
Do realizacji tego zostanie wykorzystany wewnętrzny typ `any`
i będzie to dostępne tylko w funkcjach standardowych.

### Instrukcja `return`
Funkcja zwróci wartość wyrażenia bloku kodu ciała funkcji,
ale możliwe jest wcześniejsze zwrócenie wartości `x`
poprzez instrukcję `return x`.

W przypadku funkcji, które nie zwracają wartości (zwracają typ `none`)
można zastosować samo słowo kluczowe `return`.

## Zgodność typów
Typy muszą być zgodne w m.in.:
 - operacjach arytmetycznych, logicznych, przypisania
 - wartości bloku kodu ciała funkcji i instrukcjach `return`
 - wywołaniach funkcji
 - przypisaniu wartości do zmiennej
 - wyrażeniach pętli
 - wyrażeniach warunkowych

## Kolejność operatorów
Nawiasy `(` `)` mogą wymusić inną kolejnośc.

    Priorytet       Operator/-y         Opis                    -arność     Łączność        Pozycja

    9               (a, b, ...)         wywołanie funkcji       N-nary      -               suffix
                    [a..b]              dostęp do pod-listy     Trinary     -               suffix
                    [a]                 dostęp do indeksu       Binary      -               suffix

    8               -                   negacja arytmetyczna    Unarny      prawostronna    prefix
                    !                   negacja logiczna        Unarny      prawostronna    prefix

    7               *                   mnożenie                Binarny     lewostronna     infix
                    /                   dzielenie               Binarny     lewostronna     infix
                    %                   reszta z dzielenia      Binarny     lewostronna     infix
    
    6               +                   dodawanie               Binarny     lewostronna     infix
                    -                   odejmowanie             Binarny     lewostronna     infix
                
    5               ==                  równość                 Binarny     lewostronna     infix
                    !=                  nierówność              Binarny     lewostronna     infix
                    <                   mniejszość              Binarny     lewostronna     infix
                    <=                  mniejszość lub równość  Binarny     lewostronna     infix
                    >                   większość               Binarny     lewostronna     infix
                    >=                  większośc lub równość   Binarny     lewostronna     infix
    
    4               &                   koniunkcja logiczna     Binarny     lewostronna     infix

    3               |                   alternatywa logiczna    Binarny     lewostronna     infix

    2               =                   przypisanie wartości    Binarny     prawostronna    infix

    1               return              wyjście z funkcji       Unarny      prawostronna    prefix
                    let                 deklaracja zmiennej     Binarny     prawostronna    prefix/suffix
   
## Biblioteka standardowa
Zaoferuje metody:
 - `print(arg1)`  
    wypisze wartość argumentu do strumienia wyjściowego
 - `float(arg1)`  
    zamieni argument na typ `float`
    (poprawne tylko dla argumentów typu `string` i `int`)
 - `int(arg1)`  
    zamieni argument na typ `int`
    (poprawne tylko dla argumentów typu `string` i `float`)
 - `append(arg1, arg2)`  
    doda element do końca listy
    (poprawne tylko dla argumentów typu `int[]`, `float[]`, `bool[]`
    i odpowiadającym im typom `int`, `float`, `bool`)
 - `len(arg1)`  
    zwróci długość listy
    (poprawne tylko dla argumentów typu `int[]`, `float[]`, `bool[]`, `string`)

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
Błędy będą ignorowane w czasie analizy (leksykalnej, składniowej, semantycznej),
ale ich wystąpienie uniemożliwi wykonanie programu.
Wiadomości o wszystkich błędach zostaną wypisane do strumienia błędów.

# Obsługa błędów dynamiczna
Wystąpienie błędu w czasie wykonania kodu, np.:
 - overflow
 - dzielenie przez 0
 - błąd zamiany typu (np. `string` w `float`)
 - dostęp do nieistniejącego indeksu

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
