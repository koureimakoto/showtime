#!/usr/bin/env python3

# import py_compile
# py_compile.compile('control_flow.py')

def bl( ):
    print( end='\n' )

users = { 'Hambaga': 'activia', 'Eleonard': 'inactivia', '景太郎': 'activia' }

for user, status in users.copy().items():
    if status == 'inactivia':
        print( users )
        del users[user]

activia_danone_users = {}
for user, status in users.items():
    if status == 'activia':
        activia_danone_users[user] = status


print( users )
print( activia_danone_users )

bl()

for num in range(0, 10):
    if num % 2 == 0:
        print( 'Found an even number', num )
        continue # != pass
    print( 'Found an odd number', num )


bl()
# Python match vs Rust match
def http_error( status: int ):
    match status:
        case 400:
            return 'Bad Request'
        case 404: 
            return 'Not Found'
        case _  :
            return 'Something\'s wrong with the internet'

def http_error_combined( status: int ):
    match status:
        case 401 | 403 | 404:
            return 'Not allowed'
        case _ :
            return 'Mesmo do Http_error... quer saber?? executa o outro também'

print( 'Http_error 400: ', http_error(400) )
print( 'Http_error 401: ', http_error(401) )
print( 'Http_error 401: ', http_error_combined(401) )
print( 'Http_error 400: ', http_error_combined(400) )

bl()
class Point:
    x: int
    y: int
    def __init__(self, x, y) -> None:
        self.x = x
        self.y = y

def check_point( point: Point ):
    match point:
        case ( 0, 0 ):
            print( 'Origin' )
        case ( 0, y ):
            print( f'Y={y}' )
        case ( x, 0 ):
            print( f'X={x}' )
        case (x, y):
            print( f'X={x}, Y={y}' )
        case _:
            raise ValueError( 'Not a point' )






def where_is(point):
    match point:
        case Point(x=0, y=0):
            print("Origin")
        case Point(x=0, y=y):
            print(f"Y={y}")
        case Point(x=x, y=0):
            print(f"X={x}")
        case Point():
            print("Somewhere else")
        case _:
            print("Not a point")


point: Point = Point(0,0)
nested:list[Point] = []

nested.append(point)

where_is(point)
check_point((2,2))


bl()
from enum import Enum
class Color(Enum):
    RED = 'red'
    BLUE = 'blue'
    GREEN = 'green'

color = Color( input( 'Digita uma cor, mas adivinha ao mesmo tempo, malandro: Dica, eh inglez: '))

match color:
    case Color.RED:
        print('Vermelho MALUCO')
    case Color.GREEN:
        print('Verde DOIDAO')
    case Color.BLUE:
        print('Azul MANOLO')
    case _:
        print( 'Errou sem frango' )