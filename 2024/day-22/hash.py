from z3 import *

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
print(secrets)

sum = 0
for x in secrets:
    k = x
    for i in range(2000):
        k = hash(k)
    sum += k
print(sum)
