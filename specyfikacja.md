## Wszystko
- [ ] różne sklepy
- [ ] możliwość edycji sklepu
- [ ] wysyłanie gotowego przejścia na telegram
- [ ] różne sposoby wyznaczania najlepszej drogi
  - [ ] na start sam wpisuję z palca przejście przez cały sklep
- [ ] standardowa lista zakupów
- [ ] liczba produktów do kupienia
- [ ] dopisek do produktu (np produkt: herbata; dopisek: jakaś inna niż zawsze)
- [ ] Jak najbardziej fuzzy dopasowywanie? W sensie że jak wpiszę hrbata, a na liście mam Herbata to mi wybaczy literówkę i zmienną wielkość liter
- [ ] UI
- [ ] 

## Zadania
1. Wymyślić architekturę programu 
2. Sklep (czyli gdzie co jest w sklepie) powinien być jsonem
3. Czytanie jsona
4. Do sklepu powinna też być przyłączona trasa, ale powinno się dać łatwo zmienić implementację
5. Czytanie listy zakupów
6. Standardowa lista zakupów
   1. Możliwość edycji standardowej listy zakupów
   2. Jak będzie UI to checkboxy, żeby zaznaczyć, które produkty ze standardowej listy chcę. Razem z ilością.
   3. Liczba produktów
7. Usuwanie duplikatów
8. Szynka nie powinna być dopasowana do słowa maszynka itp. Może produkt powinien mieć swoją core nazwę np herbata, do której można dawać komentarze
9. Stworzenie trasy z produktami 
   1. Tu będzie dopasowywanie podanego produktu do produktów w sklepie. Powinno być zrobione tak, żeby łatwo zmienić implementację
   2. opcjonalna liczba produktów i opcjonalne dopiski (np produkt: herbata; dopisek: jakaś inna niż zawsze)
10. Wysłanie gotowej trasy na telegram
11. Możliwość wyboru sklepu z listy
12. Możliwość edycji sklepu z listy
13. Fuzzy dopasowywanie produktu do kupienia do produktów w sklepie
14. Zrobienie WASM
    1. Jakby ludzie korzystali to coś w stylu głosowania na nowe funkcjonalności
    2. Ludzie wrzucający mapy sklepów
       1. Możliwość edycji mapy sklepu. Jakaś forma akceptacji przez ludzi. 
       2. Skąd wiadomo jaki sklep? Geolokalizacja i nazwa
    3. Każdy wrzuca swoje mapy sklepów (sam sobie oznacza jak chce). Żeby inni mogli korzystać z mapy sklepu kogoś innego to musiałaby być opcja wyświetlenia mapy sklepu  i oznaczeń.
       1. Może to nie musiałoby być super graficznie -> tylko obrazek ASCII
    4. Kody kreskowe produktów -> np kody kreskowe produktów i każdy produkt jest dokładnie umiejscowiony (a nie tylko ogólnie np herbata)
15. Zrobienie UI

## Architektura
- API Sklepu
  - ma miejsca
  - ma produkty na miejscach
  - połączenia między miejscami?
  - mapa?
- API listy zakupów
  - produkty
    - mógłbym zrobić produkt w taki sposób, że ma jakiś kod. Może coś takiego, żeby znaleźć sposób, żeby każde słowo dostało unikatową liczbę pierwszą. Kilka słów byłoby iloczynem liczb pierwszych. W ten sposób jak np herbata jest 7, a herbata czarna jest 21 to bym wiedział, że herbata czarna jest herbatą
      - to nie jest jednak dobre, bo hrbata dostanie inny kod niż herbata
    - czyli po prostu po nazwach. fzf w przyszłości
    - komentarz do produktu
    - liczba produktów
- API trasy po sklepie
  - miejsca w kolejności, w której powinny być przechodzone
- API gotowego rezultatu (czyli co kupić i gdzie i w jakiej kolejności)
    - uporządkowana lista w kolejności chodzenia [(produkt, liczba, komentarz)]
