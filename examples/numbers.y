x = []
y = [1, 2, 3]

x |> p
y |> p


z = 3
z *= 3
z /= 3
z |> p # This should print 3

t = 4
t *=
t += 2
t /=
t |> p # This should print 5