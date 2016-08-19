# https://www.hackerrank.com/challenges/bigger-is-greater

import sys

def next_greater(w):
    l = list(w);
    
    result = None
    skipped_letters = []
    for idx in range(len(l)-2, -1, -1):
        if l[idx] < l[idx+1]:
            for s_idx in range(len(l)-1, -1, -1):
                if (l[idx] < l[s_idx]):
                    temp = l[idx]
                    l[idx] = l[s_idx]
                    l[s_idx] =  temp
                    result = l[:idx+1] + l[-1:idx:-1]
                    break
        if result: break
    
    result = 'no answer' if not result else "".join(result)
    
    return result

t = raw_input()
for case in range(0, int(t)):
    w = raw_input()
    print next_greater(w)