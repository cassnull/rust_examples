Дерево модулей:

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

Корнем (отцом, главным предком) всего дерева модулей является неявный модуль с именем crate.

Модуль `hosting` является потомком (child) модуля `front_of_house`.

Модуль `front_of_house` является родителем (parent) модуля `hosting`.