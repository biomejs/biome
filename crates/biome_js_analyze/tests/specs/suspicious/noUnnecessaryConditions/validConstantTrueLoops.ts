// should not generate diagnostics

export async function drain(read: () => Promise<boolean>): Promise<void> {
  while (true) {
    if (await read()) {
      return;
    }
  }
}

export function poll(next: () => boolean): void {
  do {
    if (next()) {
      break;
    }
  } while (true);
}

export function spin(done: () => boolean): void {
  for (; true;) {
    if (done()) {
      break;
    }
  }
}
