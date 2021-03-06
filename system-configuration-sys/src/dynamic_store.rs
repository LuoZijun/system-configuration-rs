// automatically generated by rust-bindgen

// Generated using:
// bindgen 0.32.3
// macOS SDK 10.13.

use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::{Boolean, CFAllocatorRef, CFIndex, CFTypeID};
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::propertylist::CFPropertyListRef;
use core_foundation_sys::runloop::CFRunLoopSourceRef;
use core_foundation_sys::string::CFStringRef;

use dispatch_queue_t;
use libc::c_void;

pub type __SCDynamicStore = c_void;
pub type SCDynamicStoreRef = *const __SCDynamicStore;

#[repr(C)]
pub struct SCDynamicStoreContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<unsafe extern "C" fn(info: *const c_void) -> *const c_void>,
    pub release: Option<unsafe extern "C" fn(info: *const c_void)>,
    pub copyDescription: Option<unsafe extern "C" fn(info: *const c_void) -> CFStringRef>,
}

pub type SCDynamicStoreCallBack = Option<
    unsafe extern "C" fn(store: SCDynamicStoreRef, changedKeys: CFArrayRef, info: *mut c_void),
>;

#[link(name = "SystemConfiguration", kind = "framework")]
extern "C" {
    pub static mut kSCDynamicStoreUseSessionKeys: CFStringRef;

    pub fn SCDynamicStoreGetTypeID() -> CFTypeID;

    pub fn SCDynamicStoreCreate(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        callout: SCDynamicStoreCallBack,
        context: *mut SCDynamicStoreContext,
    ) -> SCDynamicStoreRef;

    pub fn SCDynamicStoreCreateWithOptions(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        storeOptions: CFDictionaryRef,
        callout: SCDynamicStoreCallBack,
        context: *mut SCDynamicStoreContext,
    ) -> SCDynamicStoreRef;

    pub fn SCDynamicStoreCreateRunLoopSource(
        allocator: CFAllocatorRef,
        store: SCDynamicStoreRef,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;

    pub fn SCDynamicStoreSetDispatchQueue(
        store: SCDynamicStoreRef,
        queue: dispatch_queue_t,
    ) -> Boolean;

    pub fn SCDynamicStoreCopyKeyList(store: SCDynamicStoreRef, pattern: CFStringRef) -> CFArrayRef;

    pub fn SCDynamicStoreAddValue(
        store: SCDynamicStoreRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;

    pub fn SCDynamicStoreAddTemporaryValue(
        store: SCDynamicStoreRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;

    pub fn SCDynamicStoreCopyValue(store: SCDynamicStoreRef, key: CFStringRef)
        -> CFPropertyListRef;

    pub fn SCDynamicStoreCopyMultiple(
        store: SCDynamicStoreRef,
        keys: CFArrayRef,
        patterns: CFArrayRef,
    ) -> CFDictionaryRef;

    pub fn SCDynamicStoreSetValue(
        store: SCDynamicStoreRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;

    pub fn SCDynamicStoreSetMultiple(
        store: SCDynamicStoreRef,
        keysToSet: CFDictionaryRef,
        keysToRemove: CFArrayRef,
        keysToNotify: CFArrayRef,
    ) -> Boolean;

    pub fn SCDynamicStoreRemoveValue(store: SCDynamicStoreRef, key: CFStringRef) -> Boolean;

    pub fn SCDynamicStoreNotifyValue(store: SCDynamicStoreRef, key: CFStringRef) -> Boolean;

    pub fn SCDynamicStoreSetNotificationKeys(
        store: SCDynamicStoreRef,
        keys: CFArrayRef,
        patterns: CFArrayRef,
    ) -> Boolean;

    pub fn SCDynamicStoreCopyNotifiedKeys(store: SCDynamicStoreRef) -> CFArrayRef;
}
