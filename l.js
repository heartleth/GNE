let stack=[[],[],[],[],[],[],[],[],[],[],[]];let tmp;stack[1].push(2);while (stack[1][stack[1].length-1] != (9+1)){stack[7].push(stack[1][stack[1].length-1]);stack[1].push((stack[7][stack[7].length-1]+1));stack[3].push(2);while (stack[3][stack[3].length-1] != (9+1)){stack[7].push(stack[7][stack[7].length-1]);if(stack[7].length==0){process.stdout.write("\n");}else{process.stdout.write(`${stack[7][stack[7].length-1]}`);stack[7].pop();}stack[1].push((stack[1][stack[1].length-1]-1));stack[7].push((stack[1][stack[1].length-1]*stack[3][stack[3].length-1]));stack[1].pop();stack[2].push((8*4));if(stack[2].length==0){process.stdout.write("\n");}else{process.stdout.write(String.fromCharCode(stack[2][stack[2].length-1]));stack[2].pop();}stack[2].push((8*4));stack[2].push((6*7));if(stack[2].length==0){process.stdout.write("\n");}else{process.stdout.write(String.fromCharCode(stack[2][stack[2].length-1]));stack[2].pop();}stack[2].push((8*4));if(stack[2].length==0){process.stdout.write("\n");}else{process.stdout.write(String.fromCharCode(stack[2][stack[2].length-1]));stack[2].pop();}stack[2].push(((stack[2][stack[2].length-1]*2)-3));stack[2].push((8*4));stack[7].push(stack[3][stack[3].length-1]);stack[3].push((stack[7][stack[7].length-1]+1));if(stack[7].length==0){process.stdout.write("\n");}else{process.stdout.write(`${stack[7][stack[7].length-1]}`);stack[7].pop();}if(stack[2].length==0){process.stdout.write("\n");}else{process.stdout.write(String.fromCharCode(stack[2][stack[2].length-1]));stack[2].pop();}if(stack[2].length==0){process.stdout.write("\n");}else{process.stdout.write(String.fromCharCode(stack[2][stack[2].length-1]));stack[2].pop();}stack[2].push((8*4));if(stack[2].length==0){process.stdout.write("\n");}else{process.stdout.write(String.fromCharCode(stack[2][stack[2].length-1]));stack[2].pop();}if(stack[7].length==0){process.stdout.write("\n");}else{process.stdout.write(`${stack[7][stack[7].length-1]}`);stack[7].pop();}if(stack[2].length==0){process.stdout.write("\n");}else{process.stdout.write(String.fromCharCode(stack[2][stack[2].length-1]));stack[2].pop();}}if(stack[6].length==0){process.stdout.write("\n");}else{process.stdout.write(String.fromCharCode(stack[6][stack[6].length-1]));stack[6].pop();}}