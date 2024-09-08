import 'node:fs'
import { join } from 'node:path'

import('node:fs/promises')

const require = () => {}

require('node:fs')

const module = {}
module.exports = 'path'

const exports = {}
exports.path = 'path'