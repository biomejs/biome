try { } catch (e) { e; e = 10; }
try {

} catch (error) {
  error = 100;
  {
    error = 10;
  }
}