#!/usr/bin/env python3

def bl( ):
    print( end='\n' )

print( 'No raw string: C:\win\name' )
print(r'Raw string   : C:\win\name' )

bl()
print("""\
export function
fn( <Option> ) {
     return simulacro
}
""")

bl()
txt_01 = 'Makoto'
txt_02 = 'World'
txt_concat = txt_01 + txt_02
print( 'Concatenated String:', txt_concat )

bl()
print( 'Auto Concatenate: <' '3' )

bl()
print( 'Vector: pos[0]: ', txt_01[0] )
print( 'Vector: pos[1]: ', txt_01[1] )
print( 'Vector: pos[2]: ', txt_01[2] )
print( 'Vector: pos[3]: ', txt_01[3] )
print( 'Vector: pos[4]: ', txt_01[4] )
print( 'Vector: pos[5]: ', txt_01[5] )

print( 'InVector: pos[-1]: ', txt_01[-1] )
print( 'InVector: pos[-2]: ', txt_01[-2] )
print( 'InVector: pos[-3]: ', txt_01[-3] )
print( 'InVector: pos[-4]: ', txt_01[-4] )
print( 'InVector: pos[-5]: ', txt_01[-5] )
print( 'InVector: pos[-6]: ', txt_01[-6] )

print( 'Range [:4]:', txt_01[:4] )
print( 'Range [4:]:', txt_01[4:] )

# Python string are immutable : I need to create a new strig to modify

