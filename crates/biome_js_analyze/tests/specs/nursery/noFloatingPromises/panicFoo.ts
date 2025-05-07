// Removing this fixes the problem.
import type elliptic from 'elliptic';

export class Foo {
  // Removing this also fixes the problem.
  prop: string;

  // Turning this into a method declaration also fixes the problem.
  doSomething = (): void => {};
}
