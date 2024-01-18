strpass = { '' }

class main:
    def code(*t):
        p='let stack=[[],[],[],[],[],[],[],[],[],[],[]];let tmp;'
        open('l.js', 'w').write(p+t[-1]())

class Block:
    def block(*ts):
        return ''.join(str(t()) for t in ts)
    
class Statement:
    def push(st, _, _1, _2, exp, *_3):
        return st() + '.push(' + exp() + ');'
    def pop(st, *_):
        return st() + '.pop();'
    def whileneq(_, a, _1, b, *t):
        return f'while ({a()} != {b()})' + '{' + t[-4]() + '}'
    def whileeq(_, a, _1, b, *t):
        return f'while ({a()} == {b()})' + '{' + t[-4]() + '}'
    def stdouti(s, *_):
        st = s()
        return 'if(' + st + '.length==0){process.stdout.write("\\n");}else{process.stdout.write(`${' + st + '[' + st + '.length-1]}`);' + st + '.pop();}'
    def stdouta(s, *_):
        st = s()
        return 'if(' + st + '.length==0){process.stdout.write("\\n");}else{process.stdout.write(String.fromCharCode(' + st + '[' + st + '.length-1]));' + st + '.pop();}'
    def swap(a, _, _1, b, *_2):
        av = a()
        bv = b()
        return f'tmp={av}[{av}.length-1];{av}[{av}.length-1]={bv}[{bv}.length-1];{bv}[{bv}.length-1]=tmp;'
    def psw(a, _, _1, b, *_2):
        av = a()
        bv = b()
        return f'{bv}.push({av}[{av}.length-1]);'
    
class Stack:
    def s0(*_):
        return 'stack[0]'
    def s1(*_):
        return 'stack[1]'
    def s2(*_):
        return 'stack[2]'
    def s3(*_):
        return 'stack[3]'
    def s4(*_):
        return 'stack[4]'
    def s5(*_):
        return 'stack[5]'
    def s6(*_):
        return 'stack[6]'
    def s7(*_):
        return 'stack[7]'
    
class Numeric:
    def n0(*_):
        return '0'
    def n1(*_):
        return '1'
    def n2(*_):
        return '2'
    def n3(*_):
        return '3'
    def n4(*_):
        return '4'
    def n5(*_):
        return '5'
    def n6(*_):
        return '6'
    def n7(*_):
        return '7'
    def n8(*_):
        return '8'
    def n9(*_):
        return '9'
    
class Expression:
    def num(n):
        return n()
    def st(n):
        st = n()
        return st + '[' + st + '.length-1]'
    def add(*tk):
        return '(' + tk[0]() + '+' + tk[-1]() + ')'
    def sub(*tk):
        return '(' + tk[0]() + '-' + tk[-1]() + ')'
    def mul(*tk):
        return '(' + tk[0]() + '*' + tk[-1]() + ')'
    def div(*tk):
        return '(' + tk[0]() + '/' + tk[-1]() + ')'