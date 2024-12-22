from z3 import *

from collections import Counter

def mix(secret, x):
    return secret ^ x

def prune(secret):
    return secret % 16777216
    
def hash(secret):
    a = secret * 64
    secret = prune(mix(secret, a))
    a = secret // 32
    secret = prune(mix(secret, a))
    a = secret * 2048
    secret = prune(mix(secret, a))
    return secret


lines = open(0).read().splitlines()
secrets = list(map(int, lines))
#secrets = [123]

def sequences(secrets):
    seqs = []
    changes = []
    for number in secrets:
        x = number
        cur = 0
        seq = []
        chg = [0]
        for i in range(2000):
            seq.append(x % 10)
            if i > 0:
                chg.append(x % 10 - cur)
            cur = x % 10
            x = hash(x)
        seqs.append(seq)
        changes.append(chg)

    return list(zip(seqs,changes))

def candidate_sequences(s, profit):
    candidates = Counter()
    for (seq, changes) in s:
        for i in range(4, len(seq)):
            if seq[i] == profit:
               window = []
               for j in range(0, 4):
                   window.append(changes[i-j])
               candidates[tuple(reversed(window))] += 1

    return candidates


        
    
print(candidate_sequences(sequences(secrets), 7))

