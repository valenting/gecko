// Copyright (c) 2015 Marshall A. Greenblatt. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the name Chromium Embedded
// Framework nor the names of its contributors may be used to endorse
// or promote products derived from this software without specific prior
// written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// ---------------------------------------------------------------------------
//
// This file was generated by the CEF translator tool and should not be edited
// by hand. See the translator.README.txt file in the tools directory for
// more information.
//

#![allow(non_snake_case, unused_imports)]

use eutil;
use interfaces;
use types;
use wrappers::CefWrap;

use libc;
use std::collections::HashMap;
use std::mem;
use std::ptr;

//
// Implement this structure to receive geolocation updates. The functions of
// this structure will be called on the browser process UI thread.
//
#[repr(C)]
pub struct _cef_get_geolocation_callback_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Called with the 'best available' location information or, if the location
  // update failed, with error information.
  //
  pub on_location_update: Option<extern "C" fn(
      this: *mut cef_get_geolocation_callback_t,
      position: *const interfaces::cef_geoposition_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: u32,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_get_geolocation_callback_t = _cef_get_geolocation_callback_t;


//
// Implement this structure to receive geolocation updates. The functions of
// this structure will be called on the browser process UI thread.
//
pub struct CefGetGeolocationCallback {
  c_object: *mut cef_get_geolocation_callback_t,
}

impl Clone for CefGetGeolocationCallback {
  fn clone(&self) -> CefGetGeolocationCallback{
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefGetGeolocationCallback {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefGetGeolocationCallback {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefGetGeolocationCallback {
  pub unsafe fn from_c_object(c_object: *mut cef_get_geolocation_callback_t) -> CefGetGeolocationCallback {
    CefGetGeolocationCallback {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_get_geolocation_callback_t) -> CefGetGeolocationCallback {
    if !c_object.is_null() &&
        c_object as usize != mem::POST_DROP_USIZE {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefGetGeolocationCallback {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_get_geolocation_callback_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_get_geolocation_callback_t {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null() || self.c_object as usize == mem::POST_DROP_USIZE
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null() && self.c_object as usize != mem::POST_DROP_USIZE
  }

  //
  // Called with the 'best available' location information or, if the location
  // update failed, with error information.
  //
  pub fn on_location_update(&self, position: &interfaces::CefGeoposition) -> (
      ) {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_location_update.unwrap())(
          self.c_object,
          CefWrap::to_c(position)))
    }
  }
} 

impl CefWrap<*mut cef_get_geolocation_callback_t> for CefGetGeolocationCallback {
  fn to_c(rust_object: CefGetGeolocationCallback) -> *mut cef_get_geolocation_callback_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_get_geolocation_callback_t) -> CefGetGeolocationCallback {
    CefGetGeolocationCallback::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_get_geolocation_callback_t> for Option<CefGetGeolocationCallback> {
  fn to_c(rust_object: Option<CefGetGeolocationCallback>) -> *mut cef_get_geolocation_callback_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_get_geolocation_callback_t) -> Option<CefGetGeolocationCallback> {
    if c_object.is_null() &&
       c_object as usize != mem::POST_DROP_USIZE {
      None
    } else {
      Some(CefGetGeolocationCallback::from_c_object_addref(c_object))
    }
  }
}

