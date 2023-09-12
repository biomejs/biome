// @ts-expect-error
const bar = case1!!.bar;

function case2(bar: number | undefined) {
  const bar1: number = bar!!!;
}

function case3(bar?: { n: number }) {
  return bar!?.n;
}

function case4(bar?: { n: number }) {
  return bar!?.();
}

const bar2 = (case5!)!.bar;

function case6(bar?: { n: number }) {
  return (bar!)?.n;
}

function case7(bar?: { n: number }) {
  return (bar)!?.n;
}

function case8(bar?: { n: number }) {
  return ((bar!))?.();
}

class Case9 {
  method() {
    this.property!!;
  }
}

case10!!.prop = null;

case11!?.[computedField];

case12!?.[a.b!!];

case13!!! = null

case14!! = null

if (case15!!) {}

if (!case16!!) {}
