#!/usr/bin/env python3


import os
import sys


def bl( ):
    print( end='\n' )

# Tutorial version
a,b = 0,1
while a < 10:
    print(a, end=',')
    a, b = b, a+b


bl()
# Traditional versiona
a,b = 0,1
while a < 10:
    print(a, ',', end='')
    buf = a
    a = b
    b =  buf+b
# force stdout
print('')

words = ['cat', 'dognight', 'bird']
for w in words:
    print('Word: ', w, '::len::', len(w))

for idx in range(2,5):
    print( 'Santo Index: ', idx )

print( list( range(5, 10) ) )