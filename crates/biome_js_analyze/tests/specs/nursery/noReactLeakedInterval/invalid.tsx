/* should generate diagnostics */
import { useEffect } from "react";

function Invalid1() {
  useEffect(() => {
    setInterval(() => {}, 1000);
  }, []);
}

function Invalid2() {
  useEffect(() => {
    window.setInterval(() => {}, 1000);
  }, []);
}

function Invalid3() {
  useEffect(() => {
    const intervalId = global.setInterval(() => {}, 1000);
  }, []);
}

function Invalid4() {
  useEffect(() => {
    const intervalId = globalThis.setInterval(() => {}, 1000);
  }, []);
}

function Invalid5() {
  useEffect(() => {
    const intervalId = setInterval(() => {}, 1000) as number;
  }, []);
}

function Invalid6() {
  useEffect(() => {
    (setInterval as any)(() => {}, 1000);
  }, []);
}
