/* should generate diagnostics */
import { useEffect } from "react";

function Invalid1() {
  useEffect(() => {
    setTimeout(() => {}, 1000);
  }, []);
}

function Invalid2() {
  useEffect(() => {
    window.setTimeout(() => {}, 1000);
  }, []);
}

function Invalid3() {
  useEffect(() => {
    const timeoutId = global.setTimeout(() => {}, 1000);
  }, []);
}

function Invalid4() {
  useEffect(() => {
    const timeoutId = globalThis.setTimeout(() => {}, 1000);
  }, []);
}

function Invalid5() {
  useEffect(() => {
    const timeoutId = setTimeout(() => {}, 1000) as number;
  }, []);
}

function Invalid6() {
  useEffect(() => {
    (setTimeout as any)(() => {}, 1000);
  }, []);
}
