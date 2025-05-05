import math

'''
T = [7, 15, 30]
d = [9, 40, 200]
'''
'''
d = [213,1168,1086,1248]
T = [35,69,68,87]
'''
'''
T = [71530]
d = [940200]
'''
d = [213116810861248]
T = [35696887]

l = len(T)

def dist(T, t):
    return T*t - t*t

num_options_all = []

for i in range(l):
    t1 = 0.5 * (T[i] - math.sqrt(T[i]**2 - 4*d[i]))
    t2 = 0.5 * (T[i] + math.sqrt(T[i]**2 - 4*d[i]))

    print(f'for T={T[i]} and d={d[i]} - intersections are t1={t1} and t2={t2}')

    if (math.ceil(t1) == t1):
        t1 = math.ceil(t1) + 1
    else:
        t1 = math.ceil(t1)

    if (math.floor(t2) == t2):
        t2 = math.floor(t2) - 1
    else:
        t2 = math.floor(t2)
    
    num_options = t2-t1+1

    num_options_all.append(num_options)
    
    print(f'for T={T[i]} and d={d[i]} - better solutions are from t1={t1} to t2={t2} ==> **{num_options}** options')

print(f'num options: {num_options_all}')
print(f'solution: {math.prod(num_options_all)}')