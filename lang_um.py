strpass = {''}

class main:
    def code(*ts):
        c = 'const readline = require("readline");const rl = readline.createInterface({  input: process.stdin,  output: process.stdout});rl.pause();function ask(question="", cb = () => void 0) {return new Promise(resolve => {rl.question(question, (...args) => {rl.pause();resolve(...args);cb(...args);});});}'
        c += 'let mem=new Array(100);let jmp = -1;let head = 0;(async()=>{let inst='
        c += str(ts[1]())
        c += ';while(head < inst.length){await inst[head]();if (jmp >= 0) {head=jmp;jmp=-1;}else{head+=1;}}})();'
        open('um.js', 'w').write(c)

class Body:
    def stmt(*st):
        return '[' + ','.join(['(async()=>{' + str(s()) + '})' for s in st]) + ']'

class Statement:
    def umm(*tokens):
        ptr = len(tokens) - 1
        return f'mem[{ptr}]=' + str(tokens[-1]()) + ';'
    def siki(*tokens):
        return f'mem[{len(tokens) - 1}]=await ask();'
    def siko(_, exp, _1):
        return f'console.log({exp()});'
    def sika(_, exp, _1):
        return f'console.log(String.fromCharCode({exp()}));'
    def sikk(_):
        return f'console.log();'
    def dongtan(_, exp, _1, stmt):
        return f'if(({exp()})==0)' + '{' + str(stmt()) + '}'
    def jun(_, exp):
        return f'jmp = {exp()}-1;'
    def fight(_, exp):
        return f'process.exit({exp()})'

class Expression:
    def exp(*exps):
        return '*'.join([f'({e()})' for e in exps])

class DU:
    def exp(*exps):
        return '+'.join([f'({e()})' for e in exps])

class Unitary:
    def inc(*dots):
        return len(dots)
    def dec(*dots):
        return -len(dots)
    def uuu(*umms):
        return f'mem[{len(umms)}]'