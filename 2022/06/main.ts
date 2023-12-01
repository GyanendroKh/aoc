const decoder = new TextDecoder('utf-8');
const inputs = decoder.decode(Deno.readFileSync('input.txt')).trim();

for (let i = 4; i < inputs.length; i++) {
  const scope = inputs.substring(i - 14, i);

  if (new Set(scope.split('')).size == 14) {
    console.log(scope, i);
    break;
  }
}
