#!/usr/bin/env python3

from operator import le


def bl( ):
    print( end='\n' )

square = [1,2,3,4,5]

print( 'Square[0]  :', square[0] )
print( 'Square[1]  :', square[1] )
print( 'Square[3]  :', square[2] )
print( 'Square[3]  :', square[3] )
print( 'Square[4]  :', square[4] )
print( 'Square[-5:]:', square[-5:] )
print( 'Square[:]  :', square[:] )

bl()
print( 'Appned 125: ', square.append(125) )
print( 'Appned 125: ', square[5] )

bl()
letters = ['m','a','k','o','t','o']
print( 'Letters: ', letters[:] )

bl()
letters[2:5] = ['K','O', 'T'] 
print( 'Letters: ', letters[:] )
print( 'Letters Len(): ', len(letters) )