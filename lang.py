strpass = {
    'Expression@sl',
    'Expression@i',
    'Integer@i'
}

class main:
    def m(*ts):
        code = ''.join(str(t()) for t in ts)
        open('l.js', 'w').write(code)
        print(code)

class Expression:
    def next(x):
        return str(x())
    def ki(x):
        return str(x())
    def zv(x):
        return str(x())
    def true(x):
        return 'true'
    def false(x):
        return 'false'
    def new(_, cname, __, *el):
        if len(el) == 2:
            return 'new C_'+ cname() + '(' + el[0]() + ')'
        else:
            return 'new C_'+ cname() + '()'
    def p(_, x, __):
        return '(' + x() + ')'
    def assigna(_, a, _1, _2, _3, b):
        return f'{a()} = {b()}'
    def assignb(a, _, __, b):
        return f'{a()} = {b()}'
    def assignc(a, _, b):
        return f'{a()} = {b()}'
    def add(a, _, b):
        return f'{a()} + {b()}'
    def eq(a, _, b):
        return f'{a()} == {b()}'
    def gt(a, _, b):
        return f'{a()} > {b()}'
    def lt(a, _, b):
        return f'{a()} < {b()}'
    def mul(a, _, b):
        return f'{a()} * {b()}'
    def arridx(a, _, b):
        return f'{a()}[{b()}]'
    def fcall(a, _, b, __):
        return f'{a()}({b()})'
    def pos(a, _, b):
        return f'{a()}.{b()}'
    def ui(a, _):
        return a() + '++'

class ExpressionList:
    def se(exp):
        return exp()
    def exps(a, _, b):
        return a() + ', ' + b()

class ImportName:
    def name(exp):
        return exp()
    def names(a, _, b):
        return a() + '.' + b()

class ParameterList:
    def sl(exp):
        return exp()
    def params(a, _, b):
        return a() + ', ' + b()

class Ident:
    def name(*chars):
        s = ''.join(str('_' if c() == '-' else c()) for c in chars)
        s = s.replace('법', '목욕탕')
        objdict = {
            '콘솔': 'console',
            '로그': 'log'
        }
        if s in objdict.keys():
            return objdict[s]
        else:
            return s

class KIdent:
    def name(k, _, i):
        return 'K_' + i()

class FunctionDefinition:
    def deff(_, _1, _2, _3, funcname, _4, _5, _6, params, _7, body):
        return f'function {funcname()}({params()}) {body()}'

class Statement:
    def ifs(_, _1, exp, _2, s, *l):
        return f'if ({exp()}) {s()}' + ''.join(t() for t in l)
    def whiles(_, _1, exp, _2, s):
        return f'while ({exp()}) {s()}'
    def exp(*s):
        return s[0]() + ';'
    def var(_, name, _1, _2, exp, _3):
        return 'var ' + name() + ' = ' + exp() + ';'
    def expect(_, _0, _1, _2, _3, _4, _5, block1, _6, _7, _8, _9, block2):
        return 'try ' + block1() + 'catch(e)' + block2()
    def itscat(*a):
        return ''
    def acomplete(*a):
        return 'break;'
    def comment(*c):
        return ''

class Else:
    def elses(_, s):
        return f'else {s()}'

class TopStatement:
    def a_society(*s):
        return s[-1]()
    def z_struct(_, name, block):
        return 'class C_' + name() + block()
    def imports(_, name):
        return 'import ' + name() + '.'
    def fdef(fd):
        return fd()
    def comment(*c):
        return ''

class Statements:
    def sts(*s):
        return ''.join([str(t()) for t in s])

class StatementBlock:
    def block(*s):
        if len(s) == 2:
            return '{}'
        elif len(s) == 3:
            return '{' + s[1]() + '}'

class ClassSpecifyBlock:
    def b(*m):
        return '{constructor(){' + ''.join(str(t()) for t in m[1:-1]) + '}}'

class Members:
    def vari(_, name, _1, _2, exp):
        return 'this.' + name() + ' = ' + exp() + ';'
    def varl(_, name, __):
        return 'this.' + name() + '=[];'
    def var(_, name, __):
        return 'this.' + name() + '=0;'
    def comment(*c):
        return ''

class Comment:
    def cmt(*s):
        return ''