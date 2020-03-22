# Algorytmy i Struktury Danych (Laboratoria - Politechnika Poznańska - Informatyka II semestr 2019/2020)

## Sortowanie

Kod źródłowy jest w języku [Rust](https://www.rust-lang.org/).
Każdy algorytm sortowania ma własny plik.

## Obsługa programu

Plik wykonywalny nazywa się **sorting** (rozszerzenie zależy od systemu).
Program przyjmuje następujące parametry:
`-g --generate` - program sam generuje losową tablicę liczb do posortowania.
`-s --size` - rozmiar sortowanej tablicy, domyślnie 10 000.
`-a --append` - dopisuje wyniki na końcu pliku wyjściowego zamiast nadpisywać jego zawartość
`-h --help` - wyświetla pomoc
`-i --input=[FILE]` - określa ścieżkę do pliku z danymi wejściowymi (dane muszą być w postaci wiersza liczb oddzielonych spacją).
`-o --output=[FILE]` - określa ścieżkę do pliku wyjściowego (domyślnie ./results.txt)

### Insertion Sort

Kod źródłowy znajduje się w pliku [insertion.rs](sorting/src/insertion.rs)

### Shell Sort z przyrostami Knutha

Kod źródłowy znajduje się w pliku [shell.rs](sorting/src/shell.rs).

### Heap Sort

Kod źródłowy znajduje się w pliku [heap.rs](sorting/src/heap.rs).

### Merge Sort

Kod źródłowy znajduje się w pliku [merge.rs](sorting/src/merge.rs).

### Quick Sort

Kod źródłowy znajduje się w pliku [quick.rs](sorting/src/quick.rs).
