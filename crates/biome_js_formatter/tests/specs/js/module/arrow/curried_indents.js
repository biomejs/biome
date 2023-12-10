
// Parameters that break over multiple lines should match indention
// with the opening parenthesis.
// The second signature and onward are indented a second level.
a.b(
  (a) =>
    (b) =>
    (
      c = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
      d = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
    ) =>
    (e) =>
      0,
  );

// In assignments, the first signature is indented, and all subsequent
// signatures indent to the same level, not a second level in.
// Block body statements also indent an additional level from the last
// signature.
x = (bifornCringerMoshedPerplexSawder) => ((askTrovenaBeenaDependsRowans, glimseGlyphsHazardNoopsTieTie) => (f00) => {
  averredBathersBoxroomBuggyNurl();
} // BOOM
)

// Chained assignments with arrow functions indent an additional level from the
// last assignment, then all subsequent signatures are at the same level
const bifornCringer1 =
  askTrovenaBeenaDepends =
  glimseGlyphs =
    (argumentOne, argumentTwo) => (restOfTheArguments12345678) => {
      return "baz";
    };

const bifornCringer2 =
    askTrovenaBeenaDepends =
    glimseGlyphs =
      (argumentOne, argumentTwo) => (restOfTheArguments12345678) => (yetAnotherArgumentPastLineWidth) => {
        return "baz";
      };

// Same-line bodies that end up expanding only indent a single time.
import('./someComponent').then(({default: TheComponent}) => (props) => (
  <TheComponent {...someInjectedProps} />
));