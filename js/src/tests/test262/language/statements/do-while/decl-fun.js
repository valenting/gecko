// |reftest| error:SyntaxError
// Copyright (C) 2016 the V8 project authors. All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.
/*---
description: Class declaration not allowed in statement position
esid: sec-do-while-statement
es6id: 13.7.2
negative:
  phase: early
  type: SyntaxError
---*/

do function f() {} while (false)
