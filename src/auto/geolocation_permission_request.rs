// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;
use webkit2_sys;
use PermissionRequest;

glib_wrapper! {
    pub struct GeolocationPermissionRequest(Object<webkit2_sys::WebKitGeolocationPermissionRequest, webkit2_sys::WebKitGeolocationPermissionRequestClass, GeolocationPermissionRequestClass>) @implements PermissionRequest;

    match fn {
        get_type => || webkit2_sys::webkit_geolocation_permission_request_get_type(),
    }
}

impl GeolocationPermissionRequest {}

pub const NONE_GEOLOCATION_PERMISSION_REQUEST: Option<&GeolocationPermissionRequest> = None;

impl fmt::Display for GeolocationPermissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GeolocationPermissionRequest")
    }
}
