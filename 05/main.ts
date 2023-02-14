const decoder = new TextDecoder('utf-8');
const inputs = decoder.decode(Deno.readFileSync('input.txt'));

type IMove = {
  from: number;
  to: number;
  size: number;
};

const [rawStack, rawMoves] = inputs
  .split('\n\n')
  .map(s => s.split('\n').filter(s => s.trim() !== ''));

const stackLen = rawStack.pop()?.replaceAll(/\s/g, '')?.length;

const parseStack = rawStack.map(s => {
  return Array(stackLen)
    .fill(null)
    .map((_, i) => {
      const idx = i * 3 + 1 + 1 * i;

      return s[idx];
    });
});

const mainStack = Array(stackLen)
  .fill(null)
  .map(() => Array<string>());

parseStack.forEach(s => {
  s.forEach((c, i) => {
    if (c !== ' ') {
      mainStack[i].unshift(c);
    }
  });
});

const moves = rawMoves.map<IMove>(m => {
  const splitted = m.split(' ');

  return {
    size: Number(splitted[1]),
    from: Number(splitted[3]),
    to: Number(splitted[5])
  };
});

moves.forEach(m => {
  const from = mainStack[m.from - 1];
  const to = mainStack[m.to - 1];

  for (let i = 0; i < m.size; i++) {
    to.push(from.pop()!);
  }
});

const result = mainStack.map(s => s.pop()).join('');

console.log(result);
