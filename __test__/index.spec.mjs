import test from "ava";

import { dummyTest } from "../index.js";

test("dummyTest from native", (t) => {
    t.is(dummyTest("This is a dummy test."), "This is a dummy test.");
});
