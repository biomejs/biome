// should not generate diagnostics
// pauseTest from a different package should not trigger
import { pauseTest } from 'some-other-package';

pauseTest();
