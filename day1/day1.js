import path from 'path';
import fs from 'fs';

const inputFilePath = path.join('input.txt');

const lines = fs
  .readFileSync(inputFilePath, 'utf8')
  .trim()
  .split('\n\n')
  .map(l => l.split('\n'));

const sum = lines.map(l => {
  return l.reduce((acc, curr) => {
    return acc + parseInt(curr, 10);
  }, 0);
});

const sorted = sum.sort((a, b) => a - b).reverse();

const [top1, top2, top3] = sorted;

console.log({ top1, top2, top3 });

console.log('Max:', Math.max(...sum));
console.log('Max:', top1 + top2 + top3);

// sum.forEach(s => console.log(s));
