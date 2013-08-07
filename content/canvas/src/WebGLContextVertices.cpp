/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 4 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "WebGLContext.h"
#include "WebGLBuffer.h"
#include "WebGLVertexAttribData.h"
#include "WebGLVertexArray.h"
#include "WebGLTexture.h"
#include "WebGLRenderbuffer.h"
#include "WebGLFramebuffer.h"
#include "WebGLUniformInfo.h"
#include "WebGLShader.h"
#include "WebGLProgram.h"

using namespace mozilla;
using namespace dom;

// For a Tegra workaround.
static const int MAX_DRAW_CALLS_SINCE_FLUSH = 100;

void
WebGLContext::VertexAttrib1f(WebGLuint index, WebGLfloat x0)
{
    if (!IsContextStable())
        return;

    MakeContextCurrent();

    if (index) {
        gl->fVertexAttrib1f(index, x0);
    } else {
        mVertexAttrib0Vector[0] = x0;
        mVertexAttrib0Vector[1] = 0;
        mVertexAttrib0Vector[2] = 0;
        mVertexAttrib0Vector[3] = 1;
        if (gl->IsGLES2())
            gl->fVertexAttrib1f(index, x0);
    }
}

void
WebGLContext::VertexAttrib2f(WebGLuint index, WebGLfloat x0, WebGLfloat x1)
{
    if (!IsContextStable())
        return;

    MakeContextCurrent();

    if (index) {
        gl->fVertexAttrib2f(index, x0, x1);
    } else {
        mVertexAttrib0Vector[0] = x0;
        mVertexAttrib0Vector[1] = x1;
        mVertexAttrib0Vector[2] = 0;
        mVertexAttrib0Vector[3] = 1;
        if (gl->IsGLES2())
            gl->fVertexAttrib2f(index, x0, x1);
    }
}

void
WebGLContext::VertexAttrib3f(WebGLuint index, WebGLfloat x0, WebGLfloat x1, WebGLfloat x2)
{
    if (!IsContextStable())
        return;

    MakeContextCurrent();

    if (index) {
        gl->fVertexAttrib3f(index, x0, x1, x2);
    } else {
        mVertexAttrib0Vector[0] = x0;
        mVertexAttrib0Vector[1] = x1;
        mVertexAttrib0Vector[2] = x2;
        mVertexAttrib0Vector[3] = 1;
        if (gl->IsGLES2())
            gl->fVertexAttrib3f(index, x0, x1, x2);
    }
}

void
WebGLContext::VertexAttrib4f(WebGLuint index, WebGLfloat x0, WebGLfloat x1,
                             WebGLfloat x2, WebGLfloat x3)
{
    if (!IsContextStable())
        return;

    MakeContextCurrent();

    if (index) {
        gl->fVertexAttrib4f(index, x0, x1, x2, x3);
    } else {
        mVertexAttrib0Vector[0] = x0;
        mVertexAttrib0Vector[1] = x1;
        mVertexAttrib0Vector[2] = x2;
        mVertexAttrib0Vector[3] = x3;
        if (gl->IsGLES2())
            gl->fVertexAttrib4f(index, x0, x1, x2, x3);
    }
}


void
WebGLContext::VertexAttrib1fv_base(WebGLuint idx, uint32_t arrayLength,
                                   const WebGLfloat* ptr)
{
    if (!ValidateAttribArraySetter("VertexAttrib1fv", 1, arrayLength))
        return;

    MakeContextCurrent();
    if (idx) {
        gl->fVertexAttrib1fv(idx, ptr);
    } else {
        mVertexAttrib0Vector[0] = ptr[0];
        mVertexAttrib0Vector[1] = WebGLfloat(0);
        mVertexAttrib0Vector[2] = WebGLfloat(0);
        mVertexAttrib0Vector[3] = WebGLfloat(1);
        if (gl->IsGLES2())
            gl->fVertexAttrib1fv(idx, ptr);
    }
}

void
WebGLContext::VertexAttrib2fv_base(WebGLuint idx, uint32_t arrayLength,
                                   const WebGLfloat* ptr)
{
    if (!ValidateAttribArraySetter("VertexAttrib2fv", 2, arrayLength))
        return;

    MakeContextCurrent();
    if (idx) {
        gl->fVertexAttrib2fv(idx, ptr);
    } else {
        mVertexAttrib0Vector[0] = ptr[0];
        mVertexAttrib0Vector[1] = ptr[1];
        mVertexAttrib0Vector[2] = WebGLfloat(0);
        mVertexAttrib0Vector[3] = WebGLfloat(1);
        if (gl->IsGLES2())
            gl->fVertexAttrib2fv(idx, ptr);
    }
}

void
WebGLContext::VertexAttrib3fv_base(WebGLuint idx, uint32_t arrayLength,
                                   const WebGLfloat* ptr)
{
    if (!ValidateAttribArraySetter("VertexAttrib3fv", 3, arrayLength))
        return;

    MakeContextCurrent();
    if (idx) {
        gl->fVertexAttrib3fv(idx, ptr);
    } else {
        mVertexAttrib0Vector[0] = ptr[0];
        mVertexAttrib0Vector[1] = ptr[1];
        mVertexAttrib0Vector[2] = ptr[2];
        mVertexAttrib0Vector[3] = WebGLfloat(1);
        if (gl->IsGLES2())
            gl->fVertexAttrib3fv(idx, ptr);
    }
}

void
WebGLContext::VertexAttrib4fv_base(WebGLuint idx, uint32_t arrayLength,
                                   const WebGLfloat* ptr)
{
    if (!ValidateAttribArraySetter("VertexAttrib4fv", 4, arrayLength))
        return;

    MakeContextCurrent();
    if (idx) {
        gl->fVertexAttrib4fv(idx, ptr);
    } else {
        mVertexAttrib0Vector[0] = ptr[0];
        mVertexAttrib0Vector[1] = ptr[1];
        mVertexAttrib0Vector[2] = ptr[2];
        mVertexAttrib0Vector[3] = ptr[3];
        if (gl->IsGLES2())
            gl->fVertexAttrib4fv(idx, ptr);
    }
}

void
WebGLContext::EnableVertexAttribArray(WebGLuint index)
{
    if (!IsContextStable())
        return;

    if (!ValidateAttribIndex(index, "enableVertexAttribArray"))
        return;

    MakeContextCurrent();
    InvalidateBufferFetching();

    gl->fEnableVertexAttribArray(index);
    mBoundVertexArray->mAttribBuffers[index].enabled = true;
}

void
WebGLContext::DisableVertexAttribArray(WebGLuint index)
{
    if (!IsContextStable())
        return;

    if (!ValidateAttribIndex(index, "disableVertexAttribArray"))
        return;

    MakeContextCurrent();
    InvalidateBufferFetching();

    if (index || gl->IsGLES2())
        gl->fDisableVertexAttribArray(index);

    mBoundVertexArray->mAttribBuffers[index].enabled = false;
}


JS::Value
WebGLContext::GetVertexAttrib(JSContext* cx, WebGLuint index, WebGLenum pname,
                              ErrorResult& rv)
{
    if (!IsContextStable())
        return JS::NullValue();

    if (!mBoundVertexArray->EnsureAttribIndex(index, "getVertexAttrib"))
        return JS::NullValue();

    MakeContextCurrent();

    switch (pname) {
        case LOCAL_GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING:
        {
            return WebGLObjectAsJSValue(cx, mBoundVertexArray->mAttribBuffers[index].buf.get(), rv);
        }

        case LOCAL_GL_VERTEX_ATTRIB_ARRAY_STRIDE:
        {
            return JS::Int32Value(mBoundVertexArray->mAttribBuffers[index].stride);
        }

        case LOCAL_GL_VERTEX_ATTRIB_ARRAY_SIZE:
        {
            if (!ValidateAttribIndex(index, "getVertexAttrib"))
                return JS::NullValue();
            
            if (!mBoundVertexArray->mAttribBuffers[index].enabled)
                return JS::Int32Value(4);
            
            // Don't break; fall through.
        }
        case LOCAL_GL_VERTEX_ATTRIB_ARRAY_TYPE:
        {
            GLint i = 0;
            gl->fGetVertexAttribiv(index, pname, &i);
            if (pname == LOCAL_GL_VERTEX_ATTRIB_ARRAY_SIZE)
                return JS::Int32Value(i);
            MOZ_ASSERT(pname == LOCAL_GL_VERTEX_ATTRIB_ARRAY_TYPE);
            return JS::NumberValue(uint32_t(i));
        }

        case LOCAL_GL_VERTEX_ATTRIB_ARRAY_DIVISOR:
        {
            if (IsWebGL2())
            {
                return JS::Int32Value(mBoundVertexArray->mAttribBuffers[index].divisor);
            }
            break;
        }

        case LOCAL_GL_CURRENT_VERTEX_ATTRIB:
        {
            WebGLfloat vec[4] = {0, 0, 0, 1};
            if (index) {
                gl->fGetVertexAttribfv(index, LOCAL_GL_CURRENT_VERTEX_ATTRIB, &vec[0]);
            } else {
                vec[0] = mVertexAttrib0Vector[0];
                vec[1] = mVertexAttrib0Vector[1];
                vec[2] = mVertexAttrib0Vector[2];
                vec[3] = mVertexAttrib0Vector[3];
            }
            JSObject* obj = Float32Array::Create(cx, this, 4, vec);
            if (!obj) {
                rv.Throw(NS_ERROR_OUT_OF_MEMORY);
            }
            return JS::ObjectOrNullValue(obj);
        }

        case LOCAL_GL_VERTEX_ATTRIB_ARRAY_ENABLED:
        {
            return JS::BooleanValue(mBoundVertexArray->mAttribBuffers[index].enabled);
        }

        case LOCAL_GL_VERTEX_ATTRIB_ARRAY_NORMALIZED:
        {
            return JS::BooleanValue(mBoundVertexArray->mAttribBuffers[index].normalized);
        }

        default:
            break;
    }

    ErrorInvalidEnumInfo("getVertexAttrib: parameter", pname);

    return JS::NullValue();
}

WebGLsizeiptr
WebGLContext::GetVertexAttribOffset(WebGLuint index, WebGLenum pname)
{
    if (!IsContextStable())
        return 0;

    if (!ValidateAttribIndex(index, "getVertexAttribOffset"))
        return 0;

    if (pname != LOCAL_GL_VERTEX_ATTRIB_ARRAY_POINTER) {
        ErrorInvalidEnum("getVertexAttribOffset: bad parameter");
        return 0;
    }

    return mBoundVertexArray->mAttribBuffers[index].byteOffset;
}

void
WebGLContext::VertexAttribPointer(WebGLuint index, WebGLint size, WebGLenum type,
                                  WebGLboolean normalized, WebGLsizei stride,
                                  WebGLintptr byteOffset)
{
    if (!IsContextStable())
        return;

    if (mBoundArrayBuffer == nullptr)
        return ErrorInvalidOperation("vertexAttribPointer: must have valid GL_ARRAY_BUFFER binding");

    WebGLsizei requiredAlignment = 1;
    switch (type) {
        case LOCAL_GL_BYTE:
        case LOCAL_GL_UNSIGNED_BYTE:
            requiredAlignment = 1;
            break;
        case LOCAL_GL_SHORT:
        case LOCAL_GL_UNSIGNED_SHORT:
            requiredAlignment = 2;
            break;
            // XXX case LOCAL_GL_FIXED:
        case LOCAL_GL_FLOAT:
            requiredAlignment = 4;
            break;
        default:
            return ErrorInvalidEnumInfo("vertexAttribPointer: type", type);
    }

    // requiredAlignment should always be a power of two.
    WebGLsizei requiredAlignmentMask = requiredAlignment - 1;

    if ( !mBoundVertexArray->EnsureAttribIndex(index, "vertexAttribPointer") ) {
        return;
    }

    if (size < 1 || size > 4)
        return ErrorInvalidValue("vertexAttribPointer: invalid element size");

    if (stride < 0 || stride > 255) // see WebGL spec section 6.6 "Vertex Attribute Data Stride"
        return ErrorInvalidValue("vertexAttribPointer: negative or too large stride");

    if (byteOffset < 0)
        return ErrorInvalidValue("vertexAttribPointer: negative offset");

    if (stride & requiredAlignmentMask) {
        return ErrorInvalidOperation("vertexAttribPointer: stride doesn't satisfy the alignment "
                                     "requirement of given type");
    }

    if (byteOffset & requiredAlignmentMask) {
        return ErrorInvalidOperation("vertexAttribPointer: byteOffset doesn't satisfy the alignment "
                                     "requirement of given type");
        
    }

    InvalidateBufferFetching();

    /* XXX make work with bufferSubData & heterogeneous types
     if (type != mBoundArrayBuffer->GLType())
     return ErrorInvalidOperation("vertexAttribPointer: type must match bound VBO type: %d != %d", type, mBoundArrayBuffer->GLType());
     */

    WebGLVertexAttribData &vd = mBoundVertexArray->mAttribBuffers[index];

    vd.buf = mBoundArrayBuffer;
    vd.stride = stride;
    vd.size = size;
    vd.byteOffset = byteOffset;
    vd.type = type;
    vd.normalized = normalized;

    MakeContextCurrent();

    gl->fVertexAttribPointer(index, size, type, normalized,
                             stride,
                             reinterpret_cast<void*>(byteOffset));
}

void
WebGLContext::VertexAttribDivisor(WebGLuint index, WebGLuint divisor)
{
    if (!IsContextStable())
        return;

    if ( !mBoundVertexArray->EnsureAttribIndex(index, "vertexAttribDivisor") ) {
        return;
    }

    WebGLVertexAttribData& vd = mBoundVertexArray->mAttribBuffers[index];
    vd.divisor = divisor;

    InvalidateBufferFetching();

    MakeContextCurrent();

    gl->fVertexAttribDivisor(index, divisor);
}

bool WebGLContext::DrawArrays_check(WebGLint first, WebGLsizei count, WebGLsizei primcount, const char* info)
{
    if (first < 0 || count < 0) {
        ErrorInvalidValue("%s: negative first or count", info);
        return false;
    }

    if (primcount < 0) {
        ErrorInvalidValue("%s: negative primcount", info);
        return false;
    }

    if (!ValidateStencilParamsForDrawCall()) {
        return false;
    }

    // If count is 0, there's nothing to do.
    if (count == 0 || primcount == 0) {
        return false;
    }

    // If there is no current program, this is silently ignored.
    // Any checks below this depend on a program being available.
    if (!mCurrentProgram) {
        return false;
    }

    if (!ValidateBufferFetching(info)) {
        return false;
    }

    CheckedInt<GLsizei> checked_firstPlusCount = CheckedInt<GLsizei>(first) + count;

    if (!checked_firstPlusCount.isValid()) {
        ErrorInvalidOperation("%s: overflow in first+count", info);
        return false;
    }

    if (uint32_t(checked_firstPlusCount.value()) > mMaxFetchedVertices) {
        ErrorInvalidOperation("%s: bound vertex attribute buffers do not have sufficient size for given first and count", info);
        return false;
    }

    if (uint32_t(primcount) > mMaxFetchedInstances) {
        ErrorInvalidOperation("%s: bound instance attribute buffers do not have sufficient size for given primcount", info);
        return false;
    }

    MakeContextCurrent();

    if (mBoundFramebuffer) {
        if (!mBoundFramebuffer->CheckAndInitializeRenderbuffers()) {
            ErrorInvalidFramebufferOperation("%s: incomplete framebuffer", info);
            return false;
        }
    }

    if (!DoFakeVertexAttrib0(checked_firstPlusCount.value())) {
        return false;
    }
    BindFakeBlackTextures();

    return true;
}

void
WebGLContext::DrawArrays(GLenum mode, WebGLint first, WebGLsizei count)
{
    if (!IsContextStable())
        return;

    if (!ValidateDrawModeEnum(mode, "drawArrays: mode"))
        return;

    if (!DrawArrays_check(first, count, 1, "drawArrays"))
        return;

    SetupContextLossTimer();
    gl->fDrawArrays(mode, first, count);

    Draw_cleanup();
}

void
WebGLContext::DrawArraysInstanced(GLenum mode, WebGLint first, WebGLsizei count, WebGLsizei primcount)
{
    if (!IsContextStable())
        return;

    if (!ValidateDrawModeEnum(mode, "drawArraysInstanced: mode"))
        return;

    if (!DrawArrays_check(first, count, primcount, "drawArraysInstanced"))
        return;

    SetupContextLossTimer();
    gl->fDrawArraysInstanced(mode, first, count, primcount);

    Draw_cleanup();
}

bool
WebGLContext::DrawElements_check(WebGLsizei count, WebGLenum type, WebGLintptr byteOffset, WebGLsizei primcount, const char* info)
{
    if (count < 0 || byteOffset < 0) {
        ErrorInvalidValue("%s: negative count or offset", info);
        return false;
    }

    if (primcount < 0) {
        ErrorInvalidValue("%s: negative primcount", info);
        return false;
    }

    if (!ValidateStencilParamsForDrawCall()) {
        return false;
    }

    // If count is 0, there's nothing to do.
    if (count == 0 || primcount == 0) {
        return false;
    }

    CheckedUint32 checked_byteCount;

    WebGLsizei first = 0;

    if (type == LOCAL_GL_UNSIGNED_SHORT) {
        checked_byteCount = 2 * CheckedUint32(count);
        if (byteOffset % 2 != 0) {
            ErrorInvalidOperation("%s: invalid byteOffset for UNSIGNED_SHORT (must be a multiple of 2)", info);
            return false;
        }
        first = byteOffset / 2;
    }
    else if (type == LOCAL_GL_UNSIGNED_BYTE) {
        checked_byteCount = count;
        first = byteOffset;
    }
    else if (type == LOCAL_GL_UNSIGNED_INT && IsExtensionEnabled(OES_element_index_uint)) {
        checked_byteCount = 4 * CheckedUint32(count);
        if (byteOffset % 4 != 0) {
            ErrorInvalidOperation("%s: invalid byteOffset for UNSIGNED_INT (must be a multiple of 4)", info);
            return false;
        }
        first = byteOffset / 4;
    }
    else {
        ErrorInvalidEnum("%s: type must be UNSIGNED_SHORT or UNSIGNED_BYTE", info);
        return false;
    }

    if (!checked_byteCount.isValid()) {
        ErrorInvalidValue("%s: overflow in byteCount", info);
        return false;
    }

    // If there is no current program, this is silently ignored.
    // Any checks below this depend on a program being available.
    if (!mCurrentProgram) {
        return false;
    }

    if (!mBoundVertexArray->mBoundElementArrayBuffer) {
        ErrorInvalidOperation("%s: must have element array buffer binding", info);
        return false;
    }

    if (!mBoundVertexArray->mBoundElementArrayBuffer->ByteLength()) {
        ErrorInvalidOperation("%s: bound element array buffer doesn't have any data", info);
        return false;
    }

    CheckedInt<GLsizei> checked_neededByteCount = checked_byteCount.toChecked<GLsizei>() + byteOffset;

    if (!checked_neededByteCount.isValid()) {
        ErrorInvalidOperation("%s: overflow in byteOffset+byteCount", info);
        return false;
    }

    if (uint32_t(checked_neededByteCount.value()) > mBoundVertexArray->mBoundElementArrayBuffer->ByteLength()) {
        ErrorInvalidOperation("%s: bound element array buffer is too small for given count and offset", info);
        return false;
    }

    if (!ValidateBufferFetching(info))
        return false;

    if (!mMaxFetchedVertices ||
        !mBoundVertexArray->mBoundElementArrayBuffer->Validate(type, mMaxFetchedVertices - 1, first, count))
    {
        ErrorInvalidOperation(
                              "%s: bound vertex attribute buffers do not have sufficient "
                              "size for given indices from the bound element array", info);
        return false;
    }

    if (uint32_t(primcount) > mMaxFetchedInstances) {
        ErrorInvalidOperation("%s: bound instance attribute buffers do not have sufficient size for given primcount", info);
        return false;
    }

    MakeContextCurrent();

    if (mBoundFramebuffer) {
        if (!mBoundFramebuffer->CheckAndInitializeRenderbuffers()) {
            ErrorInvalidFramebufferOperation("%s: incomplete framebuffer", info);
            return false;
        }
    }

    if (!DoFakeVertexAttrib0(mMaxFetchedVertices)) {
        return false;
    }
    BindFakeBlackTextures();

    return true;
}

void
WebGLContext::DrawElements(WebGLenum mode, WebGLsizei count, WebGLenum type,
                               WebGLintptr byteOffset)
{
    if (!IsContextStable())
        return;

    if (!ValidateDrawModeEnum(mode, "drawElements: mode"))
        return;

    if (!DrawElements_check(count, type, byteOffset, 1, "drawElements"))
        return;

    SetupContextLossTimer();
    gl->fDrawElements(mode, count, type, reinterpret_cast<GLvoid*>(byteOffset));

    Draw_cleanup();
}

void
WebGLContext::DrawElementsInstanced(WebGLenum mode, WebGLsizei count, WebGLenum type,
                                        WebGLintptr byteOffset, WebGLsizei primcount)
{
    if (!IsContextStable())
        return;

    if (!ValidateDrawModeEnum(mode, "drawElementsInstanced: mode"))
        return;

    if (!DrawElements_check(count, type, byteOffset, primcount, "drawElementsInstanced"))
        return;

    SetupContextLossTimer();
    gl->fDrawElements(mode, count, type, reinterpret_cast<GLvoid*>(byteOffset));

    Draw_cleanup();
}

void WebGLContext::Draw_cleanup()
{
    UndoFakeVertexAttrib0();
    UnbindFakeBlackTextures();

    if (!mBoundFramebuffer) {
        Invalidate();
        mShouldPresent = true;
        mIsScreenCleared = false;
    }

    if (gl->WorkAroundDriverBugs()) {
        if (gl->Renderer() == gl::GLContext::RendererTegra) {
            mDrawCallsSinceLastFlush++;
            
            if (mDrawCallsSinceLastFlush >= MAX_DRAW_CALLS_SINCE_FLUSH) {
                gl->fFlush();
                mDrawCallsSinceLastFlush = 0;
            }
        }
    }
}

/*
 * Verify that state is consistent for drawing, and compute max number of elements (maxAllowedCount)
 * that will be legal to be read from bound VBOs.
 */

bool
WebGLContext::ValidateBufferFetching(const char *info)
{
#ifdef DEBUG
    GLint currentProgram = 0;
    MakeContextCurrent();
    gl->fGetIntegerv(LOCAL_GL_CURRENT_PROGRAM, &currentProgram);
    MOZ_ASSERT(GLuint(currentProgram) == mCurrentProgram->GLName(),
               "WebGL: current program doesn't agree with GL state");
#endif

    if (mBufferFetchingIsVerified) {
        return true;
    }

    uint32_t maxVertices = UINT32_MAX;
    uint32_t maxInstances = UINT32_MAX;
    uint32_t attribs = mBoundVertexArray->mAttribBuffers.Length();

    for (uint32_t i = 0; i < attribs; ++i) {
        const WebGLVertexAttribData& vd = mBoundVertexArray->mAttribBuffers[i];

        // If the attrib array isn't enabled, there's nothing to check;
        // it's a static value.
        if (!vd.enabled)
            continue;

        if (vd.buf == nullptr) {
            ErrorInvalidOperation("%s: no VBO bound to enabled vertex attrib index %d!", info, i);
            return false;
        }

        // If the attrib is not in use, then we don't have to validate
        // it, just need to make sure that the binding is non-null.
        if (!mCurrentProgram->IsAttribInUse(i))
            continue;

        // the base offset
        CheckedUint32 checked_byteLength = CheckedUint32(vd.buf->ByteLength()) - vd.byteOffset;
        CheckedUint32 checked_sizeOfLastElement = CheckedUint32(vd.componentSize()) * vd.size;

        if (!checked_byteLength.isValid() ||
            !checked_sizeOfLastElement.isValid())
        {
            ErrorInvalidOperation("%s: integer overflow occured while checking vertex attrib %d", info, i);
            return false;
        }

        if (checked_byteLength.value() < checked_sizeOfLastElement.value()) {
            maxVertices = 0;
            maxInstances = 0;
            break;
        }

        CheckedUint32 checked_maxAllowedCount = ((checked_byteLength - checked_sizeOfLastElement) / vd.actualStride()) + 1;

        if (!checked_maxAllowedCount.isValid()) {
            ErrorInvalidOperation("%s: integer overflow occured while checking vertex attrib %d", info, i);
            return false;
        }

        if (vd.divisor == 0)
            maxVertices = std::min(maxVertices, checked_maxAllowedCount.value());
        else
            maxInstances = std::min(maxInstances, checked_maxAllowedCount.value() / vd.divisor);
    }

    mBufferFetchingIsVerified = true;
    mMaxFetchedVertices = maxVertices;
    mMaxFetchedInstances = maxInstances;

    return true;
}

