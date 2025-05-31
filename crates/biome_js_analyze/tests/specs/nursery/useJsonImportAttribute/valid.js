/* should not generate diagnostics */

import foo from 'bar.json' with { type: 'json' };

import bar from 'baz.json' with { other: 'value', type: 'json' }
