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
// Information about a specific web plugin.
//
#[repr(C)]
pub struct _cef_web_plugin_info_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns the plugin name (i.e. Flash).
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_name: Option<extern "C" fn(
      this: *mut cef_web_plugin_info_t) -> types::cef_string_userfree_t>,

  //
  // Returns the plugin file path (DLL/bundle/library).
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_path: Option<extern "C" fn(
      this: *mut cef_web_plugin_info_t) -> types::cef_string_userfree_t>,

  //
  // Returns the version of the plugin (may be OS-specific).
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_version: Option<extern "C" fn(
      this: *mut cef_web_plugin_info_t) -> types::cef_string_userfree_t>,

  //
  // Returns a description of the plugin from the version information.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_description: Option<extern "C" fn(
      this: *mut cef_web_plugin_info_t) -> types::cef_string_userfree_t>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: u32,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_web_plugin_info_t = _cef_web_plugin_info_t;


//
// Information about a specific web plugin.
//
pub struct CefWebPluginInfo {
  c_object: *mut cef_web_plugin_info_t,
}

impl Clone for CefWebPluginInfo {
  fn clone(&self) -> CefWebPluginInfo{
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefWebPluginInfo {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefWebPluginInfo {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefWebPluginInfo {
  pub unsafe fn from_c_object(c_object: *mut cef_web_plugin_info_t) -> CefWebPluginInfo {
    CefWebPluginInfo {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_web_plugin_info_t) -> CefWebPluginInfo {
    if !c_object.is_null() &&
        c_object as usize != mem::POST_DROP_USIZE {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefWebPluginInfo {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_web_plugin_info_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_web_plugin_info_t {
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
  // Returns the plugin name (i.e. Flash).
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_name(&self) -> String {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_name.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the plugin file path (DLL/bundle/library).
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_path(&self) -> String {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_path.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the version of the plugin (may be OS-specific).
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_version(&self) -> String {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_version.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns a description of the plugin from the version information.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_description(&self) -> String {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_description.unwrap())(
          self.c_object))
    }
  }
} 

impl CefWrap<*mut cef_web_plugin_info_t> for CefWebPluginInfo {
  fn to_c(rust_object: CefWebPluginInfo) -> *mut cef_web_plugin_info_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_web_plugin_info_t) -> CefWebPluginInfo {
    CefWebPluginInfo::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_web_plugin_info_t> for Option<CefWebPluginInfo> {
  fn to_c(rust_object: Option<CefWebPluginInfo>) -> *mut cef_web_plugin_info_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_web_plugin_info_t) -> Option<CefWebPluginInfo> {
    if c_object.is_null() &&
       c_object as usize != mem::POST_DROP_USIZE {
      None
    } else {
      Some(CefWebPluginInfo::from_c_object_addref(c_object))
    }
  }
}


//
// Structure to implement for visiting web plugin information. The functions of
// this structure will be called on the browser process UI thread.
//
#[repr(C)]
pub struct _cef_web_plugin_info_visitor_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Method that will be called once for each plugin. |count| is the 0-based
  // index for the current plugin. |total| is the total number of plugins.
  // Return false (0) to stop visiting plugins. This function may never be
  // called if no plugins are found.
  //
  pub visit: Option<extern "C" fn(this: *mut cef_web_plugin_info_visitor_t,
      info: *mut interfaces::cef_web_plugin_info_t, count: libc::c_int,
      total: libc::c_int) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: u32,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_web_plugin_info_visitor_t = _cef_web_plugin_info_visitor_t;


//
// Structure to implement for visiting web plugin information. The functions of
// this structure will be called on the browser process UI thread.
//
pub struct CefWebPluginInfoVisitor {
  c_object: *mut cef_web_plugin_info_visitor_t,
}

impl Clone for CefWebPluginInfoVisitor {
  fn clone(&self) -> CefWebPluginInfoVisitor{
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefWebPluginInfoVisitor {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefWebPluginInfoVisitor {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefWebPluginInfoVisitor {
  pub unsafe fn from_c_object(c_object: *mut cef_web_plugin_info_visitor_t) -> CefWebPluginInfoVisitor {
    CefWebPluginInfoVisitor {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_web_plugin_info_visitor_t) -> CefWebPluginInfoVisitor {
    if !c_object.is_null() &&
        c_object as usize != mem::POST_DROP_USIZE {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefWebPluginInfoVisitor {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_web_plugin_info_visitor_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_web_plugin_info_visitor_t {
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
  // Method that will be called once for each plugin. |count| is the 0-based
  // index for the current plugin. |total| is the total number of plugins.
  // Return false (0) to stop visiting plugins. This function may never be
  // called if no plugins are found.
  //
  pub fn visit(&self, info: interfaces::CefWebPluginInfo, count: libc::c_int,
      total: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).visit.unwrap())(
          self.c_object,
          CefWrap::to_c(info),
          CefWrap::to_c(count),
          CefWrap::to_c(total)))
    }
  }
} 

impl CefWrap<*mut cef_web_plugin_info_visitor_t> for CefWebPluginInfoVisitor {
  fn to_c(rust_object: CefWebPluginInfoVisitor) -> *mut cef_web_plugin_info_visitor_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_web_plugin_info_visitor_t) -> CefWebPluginInfoVisitor {
    CefWebPluginInfoVisitor::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_web_plugin_info_visitor_t> for Option<CefWebPluginInfoVisitor> {
  fn to_c(rust_object: Option<CefWebPluginInfoVisitor>) -> *mut cef_web_plugin_info_visitor_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_web_plugin_info_visitor_t) -> Option<CefWebPluginInfoVisitor> {
    if c_object.is_null() &&
       c_object as usize != mem::POST_DROP_USIZE {
      None
    } else {
      Some(CefWebPluginInfoVisitor::from_c_object_addref(c_object))
    }
  }
}


//
// Structure to implement for receiving unstable plugin information. The
// functions of this structure will be called on the browser process IO thread.
//
#[repr(C)]
pub struct _cef_web_plugin_unstable_callback_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Method that will be called for the requested plugin. |unstable| will be
  // true (1) if the plugin has reached the crash count threshold of 3 times in
  // 120 seconds.
  //
  pub is_unstable: Option<extern "C" fn(
      this: *mut cef_web_plugin_unstable_callback_t,
      path: *const types::cef_string_t, unstable: libc::c_int) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: u32,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_web_plugin_unstable_callback_t = _cef_web_plugin_unstable_callback_t;


//
// Structure to implement for receiving unstable plugin information. The
// functions of this structure will be called on the browser process IO thread.
//
pub struct CefWebPluginUnstableCallback {
  c_object: *mut cef_web_plugin_unstable_callback_t,
}

impl Clone for CefWebPluginUnstableCallback {
  fn clone(&self) -> CefWebPluginUnstableCallback{
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefWebPluginUnstableCallback {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefWebPluginUnstableCallback {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefWebPluginUnstableCallback {
  pub unsafe fn from_c_object(c_object: *mut cef_web_plugin_unstable_callback_t) -> CefWebPluginUnstableCallback {
    CefWebPluginUnstableCallback {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_web_plugin_unstable_callback_t) -> CefWebPluginUnstableCallback {
    if !c_object.is_null() &&
        c_object as usize != mem::POST_DROP_USIZE {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefWebPluginUnstableCallback {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_web_plugin_unstable_callback_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_web_plugin_unstable_callback_t {
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
  // Method that will be called for the requested plugin. |unstable| will be
  // true (1) if the plugin has reached the crash count threshold of 3 times in
  // 120 seconds.
  //
  pub fn is_unstable(&self, path: &[u16], unstable: libc::c_int) -> () {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_unstable.unwrap())(
          self.c_object,
          CefWrap::to_c(path),
          CefWrap::to_c(unstable)))
    }
  }
} 

impl CefWrap<*mut cef_web_plugin_unstable_callback_t> for CefWebPluginUnstableCallback {
  fn to_c(rust_object: CefWebPluginUnstableCallback) -> *mut cef_web_plugin_unstable_callback_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_web_plugin_unstable_callback_t) -> CefWebPluginUnstableCallback {
    CefWebPluginUnstableCallback::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_web_plugin_unstable_callback_t> for Option<CefWebPluginUnstableCallback> {
  fn to_c(rust_object: Option<CefWebPluginUnstableCallback>) -> *mut cef_web_plugin_unstable_callback_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_web_plugin_unstable_callback_t) -> Option<CefWebPluginUnstableCallback> {
    if c_object.is_null() &&
       c_object as usize != mem::POST_DROP_USIZE {
      None
    } else {
      Some(CefWebPluginUnstableCallback::from_c_object_addref(c_object))
    }
  }
}

