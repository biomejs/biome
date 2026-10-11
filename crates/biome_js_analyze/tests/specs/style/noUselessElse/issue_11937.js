function g(a) {
  const v = 1;
  if (a) {
    return v;
  } else {
    const v = 2;
    return v;
  }
}
console.log(g(false));

function f(a) {
  {
    if (a) {
      var x;
      return;
    } else {
      let x;
    }
  }
}

function h(a) {
  if (a) {
    var x;
    return;
  } else {
    let x;
  }
}

function k(a) {
  {
    while (true) {
      var x;
    }
    if (a) {
      return;
    } else {
      let x;
    }
  }
}

function m(a) {
  var x;
  {
    if (a) {
      return;
    } else {
      let x;
    }
  }
}

function catchTest(a) {
  try {
  } catch (e) {
    if (a) {
      return;
    } else {
      let e;
    }
  }
}

function validFix(a) {
  let v = 1;
  if (a) {
    return v;
  } else {
    v = 2;
    return v;
  }
}
