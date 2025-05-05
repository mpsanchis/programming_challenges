import * as fs from "fs";

class NumStr {
  name: string;
  value: number;

  constructor(name: number) {
    this.value = name;

    if (name == 1)
      this.name = 'one';
    else if (name == 2)
      this.name = 'two';
    else if (name == 3)
      this.name = 'three';
    else if (name == 4)
      this.name = 'four';
    else if (name == 5)
      this.name = 'five';
    else if (name == 6)
      this.name = 'six';
    else if (name == 7)
      this.name = 'seven';
    else if (name == 8)
      this.name = 'eight';
    else if (name == 9)
      this.name = 'nine';
    else  throw new Error('NumStr can only be created with numbers 1-9');
  }

  public isEqualToForward(line: string, pos: number): boolean {
    let posInLine = pos;
    let posInThis = 0;

    let letterInLine = line[posInLine];
    let letterInThis = this.name[posInThis];

    // is this number
    if (Number(line[posInLine]) == this.value)
      return true;

    // is this string representing a number
    while(posInThis < this.name.length && posInLine < line.length) {
      letterInLine = line[posInLine];
      letterInThis = this.name[posInThis];

      if (letterInLine != letterInThis)
        return false;

      ++posInLine;
      ++posInThis
    }

    return true;
  }

  public isEqualToBackward(line: string, pos: number): boolean {
    let posInLine = pos;
    let posInThis = this.name.length - 1;

    let letterInLine = line[posInLine];
    let letterInThis = this.name[posInThis];

    // is this number
    if (Number(line[posInLine]) == this.value)
      return true;

    // is this string representing a number
    while(posInThis >= 0 && posInLine >= 0) {
      letterInLine = line[posInLine];
      letterInThis = this.name[posInThis];

      if (letterInLine != letterInThis)
        return false;

      --posInLine;
      --posInThis
    }

    return true
  }


}

const numbers: NumStr[] = [1,2,3,4,5,6,7,8,9].map(k => new NumStr(k));

class Line {
  private line: string

  constructor(line: string) {
    this.line = line;
  }

  private whichNumberIsForward(pos: number): NumStr | null {
    for (let num of numbers) {
      if (num.isEqualToForward(this.line, pos))
        return num;
    }
    return null;
  }

  private whichNumberIsBackward(pos: number): NumStr | null {
    for (let num of numbers) {
      if(num.isEqualToBackward(this.line, pos))
        return num;
    }
    return null;
  }

  public getTens(): number {
    let posInLine = 0;
  
    while(posInLine < this.line.length) {
      const numInPos = this.whichNumberIsForward(posInLine);
      if (!!numInPos)
        return numInPos.value;

      ++posInLine;
    }
    
    throw new Error(`No tens found in this line: ${this.line}`);
  }
  
  public getUnits() {
    let posInLine = this.line.length - 1;
    
    while(posInLine >= 0) {
      const numInPos = this.whichNumberIsBackward(posInLine);
      if (!!numInPos)
        return numInPos.value;
    
      --posInLine;
    }

    throw new Error(`No units found in this line: ${this.line}`);
  }

}

function numberify(line: string): number {
  const l = new Line(line);
  return 10*l.getTens() + l.getUnits()
}

const fileName = process.argv[2];
const fileContent = fs.readFileSync(fileName).toString();

const result = [];

fileContent.split('\n').forEach(line => {
  const line2Num = numberify(line);
  console.log(`line ${line} is transformed to: ${line2Num}`);
  result.push(line2Num)
})

console.log(`result: ${result.reduce((acc,curr) => acc + curr, 0)}`);