// Test Bun.env usage
console.log(Bun.env.DEBUG);
const apiKey = Bun.env.API_KEY;
if (Bun.env.NODE_ENV === 'production') {
  console.log('prod');
}
