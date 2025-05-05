import 'bare-forbidden';
import 'default-allowed';
import 'default-forbidden';
import 'namespace-allowed';
import 'namespace-forbidden';
import 'member-allowed';
import 'member-forbidden';

import * as n1 from 'namespace-forbidden';
import * as n2 from 'default-allowed';
import * as n3 from 'member-allowed';
import * as n4 from 'bare-allowed';

import type d0 from 'default-forbidden';
import type d1 from 'member-allowed';
import type d2 from 'namespace-allowed';
import type d3 from 'bare-allowed';

import { type default as d4, type allowed1, type allowed2 as a2 } from 'default-forbidden';
import { type default as d5, type allowed2, type allowed3 as a3 } from 'member-allowed';
import { type default as d6, type allowed3 } from 'namespace-allowed';
import { type default as d7, type allowed4 } from 'bare-allowed';

import type { default as d8, allowed5, allowed6 as a6 } from 'default-forbidden';
import type { default as d9, allowed6, allowed7 as a7 } from 'member-allowed';
import type { default as d10, allowed7 } from 'namespace-allowed';
import type { default as d11, allowed8 } from 'bare-allowed';

import { type default as d12, type forbidden1, type forbidden2 as f2 } from 'member-forbidden';
import { type forbidden2, type forbidden3 as f3 } from 'member-allowed';
import type { default as d13, forbidden3, forbidden4 as f4 } from 'member-forbidden';
import type { default as d14, forbidden4, forbidden5 as f5 } from 'member-allowed';
import type { default as d15, forbidden5, forbidden6 as f6 } from 'namespace-allowed';
import type { default as d16, forbidden6, forbidden7 as f7 } from 'bare-allowed';
