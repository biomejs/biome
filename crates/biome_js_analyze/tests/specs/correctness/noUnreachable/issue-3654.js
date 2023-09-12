function testFAIL() {
  try {
    return fn();
  } catch {
    log();
  } finally {
    log();
  }
  return null;
}

function testOK() {
  try {
    return fn();
  } catch {
    log();
  }
  return null;
}

function fn() {
  throw new Error('nope!');
}

function log() {}
