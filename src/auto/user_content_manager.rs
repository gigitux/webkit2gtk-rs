// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_8", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use std::mem::transmute;
use webkit2_sys;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use JavascriptResult;
#[cfg(any(feature = "v2_6", feature = "dox"))]
use UserScript;
#[cfg(any(feature = "v2_6", feature = "dox"))]
use UserStyleSheet;

glib_wrapper! {
    pub struct UserContentManager(Object<webkit2_sys::WebKitUserContentManager, webkit2_sys::WebKitUserContentManagerClass, UserContentManagerClass>);

    match fn {
        get_type => || webkit2_sys::webkit_user_content_manager_get_type(),
    }
}

impl UserContentManager {
    #[cfg(any(feature = "v2_6", feature = "dox"))]
    pub fn new() -> UserContentManager {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(webkit2_sys::webkit_user_content_manager_new()) }
    }
}

#[cfg(any(feature = "v2_6", feature = "dox"))]
impl Default for UserContentManager {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_USER_CONTENT_MANAGER: Option<&UserContentManager> = None;

pub trait UserContentManagerExt: 'static {
    //#[cfg(any(feature = "v2_24", feature = "dox"))]
    //fn add_filter(&self, filter: /*Ignored*/&UserContentFilter);

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn add_script(&self, script: &UserScript);

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn add_style_sheet(&self, stylesheet: &UserStyleSheet);

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn register_script_message_handler(&self, name: &str) -> bool;

    #[cfg(any(feature = "v2_22", feature = "dox"))]
    fn register_script_message_handler_in_world(&self, name: &str, world_name: &str) -> bool;

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    fn remove_all_filters(&self);

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn remove_all_scripts(&self);

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn remove_all_style_sheets(&self);

    //fn remove_filter(&self, filter: /*Ignored*/&UserContentFilter);

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    fn remove_filter_by_id(&self, filter_id: &str);

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn unregister_script_message_handler(&self, name: &str);

    #[cfg(any(feature = "v2_22", feature = "dox"))]
    fn unregister_script_message_handler_in_world(&self, name: &str, world_name: &str);

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_script_message_received<F: Fn(&Self, &JavascriptResult) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<UserContentManager>> UserContentManagerExt for O {
    //#[cfg(any(feature = "v2_24", feature = "dox"))]
    //fn add_filter(&self, filter: /*Ignored*/&UserContentFilter) {
    //    unsafe { TODO: call webkit2_sys:webkit_user_content_manager_add_filter() }
    //}

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn add_script(&self, script: &UserScript) {
        unsafe {
            webkit2_sys::webkit_user_content_manager_add_script(
                self.as_ref().to_glib_none().0,
                script.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn add_style_sheet(&self, stylesheet: &UserStyleSheet) {
        unsafe {
            webkit2_sys::webkit_user_content_manager_add_style_sheet(
                self.as_ref().to_glib_none().0,
                stylesheet.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn register_script_message_handler(&self, name: &str) -> bool {
        unsafe {
            from_glib(
                webkit2_sys::webkit_user_content_manager_register_script_message_handler(
                    self.as_ref().to_glib_none().0,
                    name.to_glib_none().0,
                ),
            )
        }
    }

    #[cfg(any(feature = "v2_22", feature = "dox"))]
    fn register_script_message_handler_in_world(&self, name: &str, world_name: &str) -> bool {
        unsafe {
            from_glib(
                webkit2_sys::webkit_user_content_manager_register_script_message_handler_in_world(
                    self.as_ref().to_glib_none().0,
                    name.to_glib_none().0,
                    world_name.to_glib_none().0,
                ),
            )
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    fn remove_all_filters(&self) {
        unsafe {
            webkit2_sys::webkit_user_content_manager_remove_all_filters(
                self.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn remove_all_scripts(&self) {
        unsafe {
            webkit2_sys::webkit_user_content_manager_remove_all_scripts(
                self.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn remove_all_style_sheets(&self) {
        unsafe {
            webkit2_sys::webkit_user_content_manager_remove_all_style_sheets(
                self.as_ref().to_glib_none().0,
            );
        }
    }

    //fn remove_filter(&self, filter: /*Ignored*/&UserContentFilter) {
    //    unsafe { TODO: call webkit2_sys:webkit_user_content_manager_remove_filter() }
    //}

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    fn remove_filter_by_id(&self, filter_id: &str) {
        unsafe {
            webkit2_sys::webkit_user_content_manager_remove_filter_by_id(
                self.as_ref().to_glib_none().0,
                filter_id.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn unregister_script_message_handler(&self, name: &str) {
        unsafe {
            webkit2_sys::webkit_user_content_manager_unregister_script_message_handler(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_22", feature = "dox"))]
    fn unregister_script_message_handler_in_world(&self, name: &str, world_name: &str) {
        unsafe {
            webkit2_sys::webkit_user_content_manager_unregister_script_message_handler_in_world(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                world_name.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_script_message_received<F: Fn(&Self, &JavascriptResult) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn script_message_received_trampoline<
            P,
            F: Fn(&P, &JavascriptResult) + 'static,
        >(
            this: *mut webkit2_sys::WebKitUserContentManager,
            js_result: *mut webkit2_sys::WebKitJavascriptResult,
            f: glib_sys::gpointer,
        ) where
            P: IsA<UserContentManager>,
        {
            let f: &F = &*(f as *const F);
            f(
                &UserContentManager::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(js_result),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"script-message-received\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    script_message_received_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for UserContentManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserContentManager")
    }
}
