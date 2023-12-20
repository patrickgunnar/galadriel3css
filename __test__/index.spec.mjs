import test from "ava";

import { dummyTest } from "../index.js";

test("dummyTest from native", (t) => {
    t.true(dummyTest());
});
