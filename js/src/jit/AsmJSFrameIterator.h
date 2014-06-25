/* -*- Mode: C++; tab-width: 8; indent-tabs-mode: nil; c-basic-offset: 4 -*-
 * vim: set ts=8 sts=4 et sw=4 tw=99:
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef jit_AsmJSFrameIterator_h
#define jit_AsmJSFrameIterator_h

#include "mozilla/NullPtr.h"

#include <stdint.h>

class JSAtom;

namespace js {

class AsmJSActivation;
class AsmJSModule;
namespace jit { struct CallSite; }

// Iterates over the frames of a single AsmJSActivation.
class AsmJSFrameIterator
{
    const AsmJSModule *module_;
    const jit::CallSite *callsite_;
    uint8_t *sp_;

    void settle(uint8_t *returnAddress);

  public:
    explicit AsmJSFrameIterator(const AsmJSActivation *activation);
    void operator++();
    bool done() const { return !module_; }
    JSAtom *functionDisplayAtom() const;
    unsigned computeLine(uint32_t *column) const;
};

} // namespace js

#endif // jit_AsmJSFrameIterator_h
