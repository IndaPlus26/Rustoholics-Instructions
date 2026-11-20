# Övnings exercises for task 11!

## True or False?!
| Statement | Value | Why |
|---|---|---|
| (n + 12) * (n + 1) / 2 = O(n<sup>4</sup>) | | |
| n^2 (n^2 + n) / 2 = O(n<sup>3</sup>) | | |
| n (n^2 + n) / 2 = Θ(n<sup>3</sup>) | | |
| n (n^2 + 1) / 2 = Ω(n) | | |

## Iterative algorithms

```python
Algorithm1(n):
    e = 8
    for i in 0..8n
        e -= j

    return e

Algorithm2(n):
    e = 0
    for i in 0..n^2
        for j = 0 to i
            e += j
    return e

Algorithm3(n):
    e = 0

    if n % 2 == 0:
        for i = 0 to n:
            e += 1
    else:
        for i = 0 to n^2:
            e += 1
    return e

Algorithm4(n):
    list = []
    e = 0
    for i in 0..n/10
        e += i
        list.add(e)
                
    return e
```

| Algorithm | Worst case time complexity | Worst case space complexity |
|---|---|---|
| Algorithm1 | | |
| Algorithm2 | | |
| Algorithm3 | | |
| Algorithm4 | | |

Bonus: What is Algorithm3's Ω(?):

## Time for recursion!

```python
Algorithm5(n):
    if n <= 1:
        return n

    return n * Algorithm4(n - 1)

Algorithm6(list):
    if list.len <= 1:
        return list

    mid = list.len // 2
    left = Algorithm6(list[bottom half])
    right = Algorithm6(list[top half])

    return merge(left, right)
```

| Algorithm | Worst case time complexity | Worst case space complexity |
|---|---|---|
| Algorithm5 | | |
| Algorithm6 | | |