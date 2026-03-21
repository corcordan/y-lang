x = []
y = [1, 2, 3]
y@ |> p # prints 1, @ has a default 0
y@1 |> p # prints 2, index with numbers
y@- |> p # prints 3, getting the last using -
y@-2 |> p # prints 2, index using negatives too

z = [2,4,6,8]
z<< |> p
z>> |> p
z<<2 |> p
z>>3 |>p
