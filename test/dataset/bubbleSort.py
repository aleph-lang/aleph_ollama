def bubbleSort(t):
    i = len(t) - 1
    while(i > 0):
        j = 0
        while(i > j):
            if(t[j] > t[j + 1]):
                tmp = t[j + 1]
                t[j + 1] = t[j]
                t[j] = tmp
        j = j + 1
    i = i - 1

t = [4,6,3,7,8,1,2,9,0,5]
print(t)
bubbleSort(t)
print(t)
