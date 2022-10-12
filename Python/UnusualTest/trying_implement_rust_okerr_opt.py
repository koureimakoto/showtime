#!/usr/bin/env python3

from enum import Enum
from signal import SIGKILL, raise_signal, signal
from typing import Generic, TypeVar

O = TypeVar('O')
E = TypeVar('E')

class ResultName(Generic[O, E]):
    def __init__(self) -> None:
        super().__init__()
        self.result: O | E

    def mat(self, opt: O | E):
        self.result = opt
        return self.result

class Sequence(ResultName):
    def __init__(self) -> None:
        super().__init__()
        self.ls: dict[int, str] = {}
        self.need_register: str  = 0

    def __call__(value):
        pass

    def register(self, reg: int ):
        if reg <= 0:
            self.mat( 'Nao contem elemento' )
        else:
            self.mat([reg, ''])
        return self

    def remove(self):
        if self.need_register <= 0:
            return None
        del self.ls[ self.need_register ]
        self.need_register = 0
        return self.result.mat(opt = self)

    def add(self, value: str):
        if type(self.result) == str :
            print('xii')
            return None
        self.ls[ self.result[0] ] = value
        return self

    def print(self):
        if type(self.result) == str :
            print('xii')
            return None
        print(self.ls[ self.result[0] ])

def handler(signum, frame):
    OSError('Insecury approach. Add in the final the exeception()')

x = Sequence()
x.register(1010).add('add_some')
x.register(1010).print()

y = x.register(1010).remove()
print( x.register(1010).remove() )