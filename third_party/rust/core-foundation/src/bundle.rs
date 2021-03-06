// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation Bundle Type

pub use core_foundation_sys::bundle::*;
use core_foundation_sys::base::{CFRelease, kCFAllocatorDefault};

use base::TCFType;
use url::CFURL;
use dictionary::CFDictionary;

/// A Bundle type.
pub struct CFBundle(CFBundleRef);

impl Drop for CFBundle {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl CFBundle {
    pub fn new(bundleURL: CFURL) -> Option<CFBundle> {
        unsafe {
            let bundle_ref = CFBundleCreate(kCFAllocatorDefault, bundleURL.as_concrete_TypeRef());
            if bundle_ref.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(bundle_ref))
            }
        }
    }

    pub fn main_bundle() -> CFBundle {
        unsafe {
            let bundle_ref = CFBundleGetMainBundle();
            TCFType::wrap_under_get_rule(bundle_ref)
        }
    }

    pub fn info_dictionary(&self) -> CFDictionary {
        unsafe {
            let info_dictionary = CFBundleGetInfoDictionary(self.0);
            TCFType::wrap_under_get_rule(info_dictionary)
        }
    }

    pub fn executable_url(&self) -> Option<CFURL> {
        unsafe {
            let exe_url = CFBundleCopyExecutableURL(self.0);
            if exe_url.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(exe_url))
            }
        }
    }

    pub fn private_frameworks_url(&self) -> Option<CFURL> {
        unsafe {
            let fw_url = CFBundleCopyPrivateFrameworksURL(self.0);
            if fw_url.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(fw_url))
            }
        }
    }
}

impl_TCFType!(CFBundle, CFBundleRef, CFBundleGetTypeID);

#[test]
fn safari_executable_url() {
    use string::CFString;
    use url::{CFURL, kCFURLPOSIXPathStyle};

    let cfstr_path = CFString::from_static_string("/Applications/Safari.app");
    let cfurl_path = CFURL::from_file_system_path(cfstr_path, kCFURLPOSIXPathStyle, true);
    let cfurl_executable = CFBundle::new(cfurl_path)
        .expect("Safari not present")
        .executable_url();
    assert!(cfurl_executable.is_some());
    assert_eq!(cfurl_executable
                   .unwrap()
                   .absolute()
                   .get_file_system_path(kCFURLPOSIXPathStyle)
                   .to_string(),
               "/Applications/Safari.app/Contents/MacOS/Safari");
}

#[test]
fn safari_private_frameworks_url() {
    use string::CFString;
    use url::{CFURL, kCFURLPOSIXPathStyle};

    let cfstr_path = CFString::from_static_string("/Applications/Safari.app");
    let cfurl_path = CFURL::from_file_system_path(cfstr_path, kCFURLPOSIXPathStyle, true);
    let cfurl_executable = CFBundle::new(cfurl_path)
        .expect("Safari not present")
        .private_frameworks_url();
    assert!(cfurl_executable.is_some());
    assert_eq!(cfurl_executable
                   .unwrap()
                   .absolute()
                   .get_file_system_path(kCFURLPOSIXPathStyle)
                   .to_string(),
               "/Applications/Safari.app/Contents/Frameworks");
}

#[test]
fn non_existant_bundle() {
    use string::CFString;
    use url::{CFURL, kCFURLPOSIXPathStyle};

    let cfstr_path = CFString::from_static_string("/usr/local/foo");
    let cfurl_path = CFURL::from_file_system_path(cfstr_path, kCFURLPOSIXPathStyle, true);
    assert!(CFBundle::new(cfurl_path).is_none());
}
