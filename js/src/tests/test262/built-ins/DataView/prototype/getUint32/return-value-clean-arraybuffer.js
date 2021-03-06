// Copyright (C) 2016 the V8 project authors. All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.

/*---
esid: sec-dataview.prototype.getuint32
es6id: 24.2.4.12
description: >
  Return value from Buffer using a clean ArrayBuffer
info: |
  24.2.4.12 DataView.prototype.getUint32 ( byteOffset [ , littleEndian ] )

  1. Let v be the this value.
  2. If littleEndian is not present, let littleEndian be false.
  3. Return ? GetViewValue(v, byteOffset, littleEndian, "Uint32").

  24.2.1.1 GetViewValue ( view, requestIndex, isLittleEndian, type )

  ...
  14. Let bufferIndex be getIndex + viewOffset.
  15. Return GetValueFromBuffer(buffer, bufferIndex, type, isLittleEndian).
  ...

  24.1.1.5 GetValueFromBuffer ( arrayBuffer, byteIndex, type [ , isLittleEndian
  ] )

  ...
  8. If isLittleEndian is false, reverse the order of the elements of rawValue.
  ...
---*/

var buffer = new ArrayBuffer(8);
var sample = new DataView(buffer, 0);

assert.sameValue(sample.getUint32(0, true), 0, "sample.getUint32(0, true)");
assert.sameValue(sample.getUint32(1, true), 0, "sample.getUint32(1, true)");
assert.sameValue(sample.getUint32(2, true), 0, "sample.getUint32(2, true)");
assert.sameValue(sample.getUint32(3, true), 0, "sample.getUint32(3, true)");
assert.sameValue(sample.getUint32(4, true), 0, "sample.getUint32(4, true)");
assert.sameValue(sample.getUint32(0, false), 0, "sample.getUint32(0, false)");
assert.sameValue(sample.getUint32(1, false), 0, "sample.getUint32(1, false)");
assert.sameValue(sample.getUint32(2, false), 0, "sample.getUint32(2, false)");
assert.sameValue(sample.getUint32(3, false), 0, "sample.getUint32(3, false)");
assert.sameValue(sample.getUint32(4, false), 0, "sample.getUint32(4, false)");

reportCompare(0, 0);
