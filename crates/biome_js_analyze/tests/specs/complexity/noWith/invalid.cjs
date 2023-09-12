function f() {
  with (point) {
    r = Math.sqrt(x * x + y * y); // is r a member of point?
  }
}