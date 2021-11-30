#Print match patterns.

def check(a, b):
    res = False
    for i in [2,3,4,5]:
        res += a % i == 0 and b % i == 0
    return not res

i = 0
fracts = []
for n in range(1,10):
    for d in range(2,11):
        if n < d and check(n,d) or n == 1:
            i += 1
            fracts.append((n,d))

print("   ")
fracts.sort(key=lambda v: v[0]/v[1])

def closest(n):
    f = fracts
    f.sort(key=lambda v: abs(v[0]/v[1] - n))
    return f[0]

matches = []
cur = [fracts[0], [1]]

k = 100
for i in range(1, k):
    f = closest(i/k)
    if f == cur[0]:
        cur[1].append(i)
    else:
        matches.append(cur)
        cur = [f, [i]]    
      
matches.append(cur)
        
for v in matches:
    print(str(v[1][0]) + "..=" + str(v[1][-1]) + " => " + str(v[0]) + ",")