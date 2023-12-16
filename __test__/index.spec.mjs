import test from 'ava'

import { dummySum } from '../index.js'

test('dummySum from native', (t) => {
  t.is(dummySum(4, 2), "The current sum is equal to 6")
})
