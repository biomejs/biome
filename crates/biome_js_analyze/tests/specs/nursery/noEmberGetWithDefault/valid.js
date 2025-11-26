// should not generate diagnostics
import { get } from '@ember/object';

const value = get(obj, 'key') ?? 'default';
