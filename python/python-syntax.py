# ==================================================================================================== 
# 
# Every single "instance" in python is an object.
# Every object's lifetime is controlled by valid reference counting ~ shared system mechanics.
# Some objects are fixed muttable or immutable.
#
#
#
# main scope
#   another scope
# ^^
# this is tabulation
#
#
#
# Link on lectures
# https://github.com/MelLain/mipt-python/tree/spring-2024/lectures
#
# ==================================================================================================== 

from __future__ import annotations                  # has to be here for forward references block

# ==================================================================================================== 

#
# Types and operations
#


a = 5                           # a is object, 5 is also separate object. a now points on 5. rn 2 objects are alive
b = a

id(b) == id(a)                  # -> False, a and b are not the same (and/or) constant objects
a is b                          # -> True, both points at 5


del a                           # manually delete a, if there are no objects pointing on 5 -> 5 would be deleted as well, but b points on 5
# b would still point on 5, 5 is still alive

text_1 = 'text'
text_2 = "Is not = isn't"       # "" ~ '', but such cases requires ""

print({a:=55})                  # := is walrus operator that returns assigned value

x, y, z = 1, 2, 3               # [Feature]

5 // 2              # = 2
5 % 2               # = 1

type(1)             # int

bool_value = True or (False and True)

complex_number = 1 + 4j

none_object = None

annotation_value: int = 5       # strong typisation
annot_lst: list[int] = [1, 2, 3]

# ==================================================================================================== 

#
# Some math
#

pow(2, 4) == 2 ** 4     # 2^4
abs(-2)
sum((1, 2, 3))
min(54, 2346, 234, 6)

import math
math.sqrt(2)
math.sin(10)

math.trunc(1.4)             # move current val to 0  -> 1
math.floor(-5.2)            # move to lowest integer -> -5
round(1.5)                  # standart round         -> 2

# ==================================================================================================== 

#
# Input / Output
#

input_value = input('Input ur value here:')     # this func print text + returns user input
print('Congrats')

# ==================================================================================================== 

#
# [Feature] String evaluation as code
#

eval('1 + 2')
exec(
'''
str_a = 123
str_b = 111
print(str_a + str_b)
'''
)

# ==================================================================================================== 

#
# None-type
#

None == None    # True
None or True    # True
# 5 * None      # error

if none_object is None:                 # prefer is-checking instead of == or !=
    pass

# ==================================================================================================== 

#
# String interactions
# [Feature] Slicing
# [Feature] Formating
#

dummy_text = 'abcdefg'  # operations bellow will NOT MODIFY dummy_text
dummy_text[-1]          # bound jumping             -> 'g'
dummy_text[0: 3]        # slicing [0, 3)            -> 'abc'    
dummy_text[:3]          # slicing [0, 3]            -> 'abc'
dummy_text[slice(1,3)]  # slicing (1, 3)            -> 'bc'
dummy_text[:10:2]       # till 10 pos with step 2   -> 'aceg'
dummy_text[::-1]        # reverse                   -> 'gfedcba'
dummy_text[::-2]        # reverse, but 1/2

dummy_text = "qw{}ty".format('er')                      # 'qwerty'
'x = %d, y = %d' % (10, 11.5)
'x = {1}, y = {0}'.format(000, 111)                     # 'x = 111, y = 000'
f'x = {10}, y = {20}'                                   # 'x = 10, y = 20'
'x = {val1:f}, y = {val2:f}'.format(val1=10, val2=20)
f'{dummy_text=}, also dummy_text is {dummy_text}'       # 'dummy_text=qwerty, also dummy text is qwerty'

# ==================================================================================================== 

#
# Containers / collections
# [Feature] list-generators
#

lst = [1, None, 'abc']                  # Simple array-like type [] - MUTABLE - any types inside and dynamicly resizable
lst[0] = 111                            # Mutable
lst[-1]                                 # Same bound logic -> 'abc'
# lst[5]                                # Error

even_qrt_lst = [x ** 2 for x in [1, 3, 5, 7, 9] if x <= 5]    # list generator -> [1, 9, 25]

lst1 = [1, 2, 3]
lst2 = lst1[:]                  # copy
lst2 = lst1.copy()              # also copy
lst3 = lst1                     # not copy, lst3 now also points on [1, 2, 3]... as well as lst1

import copy

lst1 = [[1, 2, 3], 4, 5]    
lst2 = copy.deepcopy(lst1)          # also copy nested muttable objects! any nested-nested objects require more deepcopy calls =)

tupple_object = (1, 'abc', True)                    # Tuple - IMMUTABLE - just like list


dictionary = {1: 'One',
              2: 'Two',
              3: 'Three',
              4: 'Four'}            # Dictonary {key : value} - MUTABLE - based on hash-tables
del dictionary[4]                   # removes object with key - 4

d = dict.fromkeys(list('aaaabbcccc'))           # {'a': None, 'b': None, 'c': None}

d1 = {4: 'four', 5: 'five'}
dictionary | d1                     # Dictionary concatenation (no assign)
dictionary |= d1                    # Concatenation & assign

set_object = {1, 2, 2, 3, 3, 3, 3, 3, '4', '4', '5'}            # Set - MUTABLE - just like dictionary, but no keys, i.e. no repeat values -> {1, 2, 3, '4', '5'}

z = zip([1, 2, 3], 'abc')           # 'Compress by iterators 1 to 'a', 2 to 'b', 3 to 'c'
for a, b in z:
    print(a, b, end=' ')
print()                             # skip line

# ==================================================================================================== 

#
# Working with files
#

file_to_read = open('some_file.txt', 'r')           # Open file on read('r') | 'a' - append - to write
file_to_read.read()
file_to_read.close()
file_to_read.closed                                 # check if closed

with open('some_file.txt', 'a') as file_to_write:           # file context-manager, better to use this all the time
    file_to_write.write('additional string')

# ==================================================================================================== 

#
# if / while / for and e.t.c.
#

if 5 % 2 == 0:
    pass                                # simple buddy for empty scope
else:
    pass

a = 5 if None is None else 5555         # ternar

a = 10
while a is not None:
    a += 1
    if a == 15: break
    if a != 13: continue 
else:                                   # not necessary, but a sign that it can be here
    pass

lst = [1, 2, 3]
for element in lst:                     # range based
    pass

for index in range(len(lst)):           # index based, range() is a generator that operates with iterators
    if lst[index] is not None: break
else:
    pass

for index, element in enumerate(lst):   # index + range based
    pass

# ==================================================================================================== 

#
# Modules
#

import numbers

numbers.Number()                                # numbers - module namespace

import numbers as nbrs                          # second import won't do anything

nbrs.Number()                                   # nbrs - synonym to numbers module namespace

from importlib import reload as reload_analog   # import part of the module

reload_analog(numbers)                          # manually reload import (for example if it somehow changed)

# ==================================================================================================== 

#
# Functions
# functions are also object, they are also symbol links
#

def function(arg1, arg2=None):                  # arg1 & arg2 are symbol links -> they are REFERENCES, arg2 - has default param
    pass

function(arg1=5)                                # [Feature] function call, arg1 - used as keyword


def VarArgFunction(*args, a):                 # args as var-arg - args here - TUPLE
    pass

VarArgFunction(1, 4, 5, 2, 3, 5, 6, 7, a=2)


def VarArgFunction(a, *args, **kwargs):                                             # [Feature] kwargs here - DICTIONARY
    pass

VarArgFunction(5, *(1,2,3,4,5,6), **{'1': 'One', '2': 'Two', '3': 'Three'})         # could not write a=5


def PositionSpecificationFunction(pos_only_param1, pos_only_param2, /, pos_or_kword_param, *, kword_only_param1, kword_only_param2): 
    pass

PositionSpecificationFunction(1, 2,                         # position only params
                              5,                            # allow either position param or keyword
                              kword_only_param1=4,          # only keyword param
                              kword_only_param2=10)         # only keyword param

PositionSpecificationFunction(1, 2,                         # position only params
                              pos_or_kword_param=5,         # allow either position param or keyword
                              kword_only_param1=4,          # only keyword param
                              kword_only_param2=10)         # only keyword param

# PositionSpecificationFunction(1, pos_or_kword_param=2, 
#                               5, 
#                               kword_only_param1=4, 
#                               kword_only_param2=10)      # error

# PositionSpecificationFunction(1, pos_or_kword_param=2, 5, 4, 10)      # error


to_glob_var = 10
def Func():
    global to_glob_var                      # "capture" global value
    to_glob_var += 1

def Func1():
    a = 10
    def FuncInner():
        nonlocal a                          # "capture" value from not local scope
        a += 100

def StrongTypeFunc(obj: str, *args: int, **kwargs: float) -> None:
    pass

# ==================================================================================================== 

#
# Lambda
#

lambda_sqrt = lambda x: x ** 2              # input -> x, execution block -> x ** 2

# ==================================================================================================== 

#
# Iterators
#

it = iter([1, 2, 3])

# ==================================================================================================== 

#
# Generators
#

generator = (x*2 for x in [1, 2, 3])        # Iterator subtype, gets iterator on first element

print(next(generator), next(generator))     # -> 1, 2   -   generator could store states

def my_range(n):                            # -> First entry 0 1 2 3 4 5 ...    - depends on n input
    yield 'First entry'                     # like return, but func will store it's state, so.. this yield return for 1st call, but skipped for any next call

    i = -1
    while i < n:
        i += 1
        yield i                             # this yield would be called for 2nd my_range() call, then for 3rd call but with changed i value and e.t.c.

# ==================================================================================================== 

#
# Classes
#

class Color:
    brown: str = 'brown'                        # class member

clr = Color()
clr.brown
Color.brown
clr.brown = 'Brown'

clr.red = 'red'                                 # [Feature] We could add members to class from 'anywhere' -_-


class Kitty:
    def __init__(self):                         # Entry point, NOT A CONSTRUCTOR! It gets object and init it's state. __new__ is responsible to create obj and it's a static member
        self.bite_power = 9999

    def Bite(self, bite_power) -> None:         # any class's stuff should notice pointer on class - self
        pass

    def Bite1(idk_what_is_self, bite_power):    # is also possible... don't do that*
        pass

    def Cuteonize(howStrong):                   # no self = static member
        pass

    def __magic_method__(self):                  # magic method
        pass

class Dummy:
    def __init__(self):
        self.public_field = 1                   # like public
        self._private_field = -1                # like protected, but visible as public
        self.__mangled_field = -999             # like private, but still could be used somehow.......

    def DummyDo(self):
        pass

class DummyBro:
    pass

class DerivedDummy(Dummy, DummyBro):            # inheritance (multiple)
    def DummyDo(self):                          # method override
        return super().DummyDo()                # call parent's method implementation

dmy = Dummy()
#print(dmy.__mangled_field)                     # error -> can't see from outside
print(dmy._Dummy__mangled_field)                # ...... i meant this somehow.....


x = 5
if isinstance(x, int):                          # check type, also could check if it's inherited
    pass

class DerivedInt(int):
    pass

if issubclass(DerivedInt, int):                 # the same but for types
    pass

# ==================================================================================================== 

#
# Attributes
# Some special params or methods, that should not be public, but are helpfull sometimes
#

print(dir(5)[:4])                   # special attributes are __magic_field__

class Cls:
    pass

cls = Cls()

setattr(cls, 'some_attr', 'some')   # add attribute

print(getattr(cls, 'some_attr'))

delattr(cls, 'some_attr')           # delete attribute

#print(getattr(cls, 'some_attr'))   # error

# ==================================================================================================== 

#
# [Feature] Decorators
# Not the same as programming pattern!
#

class DecoratedInnocent:
    def __init__(self): pass

    @property                       # Decoration
    def field(self):                # makes field read-only
        return self._field

    @staticmethod
    def StaticMethod():         
        pass

    @classmethod
    def ClassMethod():
        pass


hs_prp = DecoratedInnocent()
DecoratedInnocent.StaticMethod()
#hs_prp.field = 10                  # error -> field is read-only property


from dataclasses import dataclass

@dataclass(frozen=True, order=True)         # it's like a const structure of data, and it could be iteratable as tuple
class Data:
    int_field: int
    str_field: str


# ==================================================================================================== 

#
# throw, try-catch
#

try:
    pass
except ValueError:
    raise                           # ~ throw -> here raise is like passing error further (for example, to next except: block)
except RuntimeError as error:
    raise LookupError from error    # could pass further with keeping previous exceptions
except:                             # catch any exception
    pass
else:                               # if no exception
    pass
finally:                            # would be called anyway eventually
    pass

# ==================================================================================================== 

#
# Abstract classes
#

class AbstractDummy1():
    def method(self):
        raise NotImplementedError

from abc import ABC, abstractmethod

class AbstractDummy2(ABC):
    @abstractmethod
    def method():
        ...

# ==================================================================================================== 

#
# Typing stuff
#

from typing import Union, Any, Optional

def FuncUnion(x: Union[list[int], set[int]]) -> None: ...       # could get x as list[int] or as set[int]

def FuncAny(x: Any) -> None: x.nothing()                        # less checks than dynamic typing

def FuncOptional(x: Optional[list[int]]) -> None: ...           # x as None or list[int]

# ==================================================================================================== 

#
# Generics
#

from typing import Generic, TypeVar

T = TypeVar('T')                                                # Ref to type

class Stack(Generic[T]):
    def __init__(self):
        self.items: list[T] = []

    def push(self, item: T):
        self.items.append(item)


from typing import Callable                                             # Protocols are like concepts

def ProtocolFunc(f: Callable[[int, int], bool]) -> bool:                # f should be func with signature - func(int, int) -> bool ~~ f should have __call__ and could be called in such extent
    return f(1,2)

ProtocolFunc(lambda x, y: x == y)

# ==================================================================================================== 

#
# Forward references
# ~ like using currently undefined types
#

class ForwardDummy1:
    def frwd(self) -> 'ForwardDummy1':              # without '...' there would be error
        return ForwardDummy1()


class ForwardDummy2:                                
    def frwd(self) -> ForwardDummy2:                # requires from __future__ import annotations (in the beginning)
        return ForwardDummy2()