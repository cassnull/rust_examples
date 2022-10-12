### Quicksort. ###

Быстрый метод сортировки функционирует по принципу "разделяй и властвуй".

- Массив $a[l \dots r]$ типа Tразбивается на два (возможно пустых) под массива $a[l \dots q]$ и $a[q + 1 \dots r]$, таких, что каждый элемент $a[l \dots q]$ меньше или равен $a[q]$, который в свою очередь, не превышает любой элемент под массива $a[q + 1 \dots r]$. Индекс вычисляется в ходе процедуры разбиения.
- Под массивы $a[l \dots q]$ и $a[q + 1 \dots r]$ сортируются с помощью рекурсивного вызова процедуры быстрой сортировки.
- Поскольку под массивы сортируются на месте, для их объединения не требуются никакие действия: весь массив $a[l \dots r]$ оказывается отсортированным.

| лучшее время | среднее | худшее | память |
|:------------:|:-------:|:------:|:------:|
| $O(n \times log(n))$ | $O(n \times log(n))$ | $O(n^2)$ | $O(log(n))$ |