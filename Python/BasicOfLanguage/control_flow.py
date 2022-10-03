#!/usr/bin/env python3

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
