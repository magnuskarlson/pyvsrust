import rust
import random
import time

# PYTHON MERGESORT
def mergesort(m):
    _mergesort(m, 0, len(m))

def _mergesort(m, i, j):
    k = (i + j) // 2
    if j - i > 2:
        _mergesort(m,i,k)
        _mergesort(m,k,j)
    _merge(m, i, k, j)

def _merge(m, i, k, j):
    m1 = m[i:k]
    m2 = m[k:j]
    mn1 = 0
    mn2 = 0
    # Big optimization
    l1 = len(m1)
    l2 = len(m2)
    for n in range(i,j): 
        if mn1 < l1 and mn2 < l2:
            if m1[mn1] < m2[mn2]:
                m[n] = m1[mn1]
                mn1 += 1
            else:
                m[n] = m2[mn2]
                mn2 += 1
        elif mn1 < l1:
            m[n] = m1[mn1]
            mn1 += 1
        else:
            m[n] = m2[mn2]
            mn2 += 1

# PYTHON QUIKCSORT
def quicksort(m):
    _quicksort(m, 0, len(m) - 1)

def _quicksort(m, lo, hi):
    if lo < hi:
        p = _partition(m, lo, hi)
        _quicksort(m, lo, p - 1)
        _quicksort(m, p + 1, hi)

def _partition(m, lo, hi):
    pivot = m[hi]
    i = lo - 1
    for j in range(lo,hi):
        if m[j] < pivot:
            i += 1
            m[j], m[i] = m[i], m[j]
    i += 1
    m[hi], m[i] = m[i], m[hi]
    return i

# TESTING
format_time = lambda x : round(x * 1000)
gen_numbers = lambda n : [random.randint(0, 100000) for _ in range(0, n)]
RESULTS = {}

for size in [25000, 100000, 250000, 1000000]: # 1000000
    RESULTS[size] = {}
    numbers = gen_numbers(size)
    print(f"TEST SIZE {size}")
    for func, name in zip([mergesort, quicksort, rust.mergesort, rust.quicksort], 
                    ['PY MERGESORT', 'PY QUICKSORT', 'RUST MERGESORT', 'RUST QUICKSORT']):
        start = time.perf_counter()
        func(numbers[:])
        end = time.perf_counter()
        RESULTS[size][name] = format_time(end - start)
        print(f"{name}\t{format_time(end - start)}MS")
    print('')

for x in RESULTS:
    temp = {}
    for y in sorted(RESULTS[x], key=RESULTS[x].get, reverse=False):
        temp[y] = RESULTS[x][y]
    RESULTS[x] = temp

print(RESULTS)
        
