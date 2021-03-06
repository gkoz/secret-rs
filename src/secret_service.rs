//=========================================================================
// public imports
//=========================================================================
pub use secret_collection::*;
pub use secret_item::*;
pub use secret_value::*;

//=========================================================================
// private imports
//=========================================================================
use std::ptr;
use glib::Error;
use glib::glib_container::GlibContainer;
use glib::object::{Ref, Wrapper};
use glib::types::{StaticType, Type};
use glib::translate::{FromGlibPtr, FromGlibPtrContainer};
use glib::ffi::{GObject};
use SecretResult;
use ffi;

/// A SecretService object represents the Secret Service implementation which runs as a D-Bus service.
/// In order to securely transfer secrets to the Sercret Service, a session is established. This will automatically be done when calling `SecretService::get()`
/// To search for items, use the `search()` method.
/// Multiple collections can exist in the Secret Service, each of which contains secret items. To access the list of Collections, use `get_collections()`.
/// Certain actions on the Secret Service require user prompting to complete, such as creating a collection, or unlocking a collection. When such a prompt is necessary, then a SecretPrompt object is created by libsecret, and passed to the secret_service_prompt() method. In this way it is handled automatically.
pub struct SecretService(Ref);

impl SecretService {

    /// Constructs a new SecretService which has established a session and whose collections are loaded.
    /// The underlying FFI object might be identical for multiple instances of this struct.
    pub fn get() -> SecretResult<Self>{
        SecretService::with_flags(SECRET_SERVICE_OPEN_SESSION | SECRET_SERVICE_LOAD_COLLECTIONS)
    }

    fn with_flags(flags: i32) -> SecretResult<Self> {
            let mut err = ptr::null_mut();
            let ptr = unsafe{ffi::secret_service_get_sync(flags, ptr::null_mut(), &mut err)};
            if err.is_null() {
                Ok(SecretService(Ref::from_glib_none(ptr as *mut GObject)))
            } else {
                Err(Error::wrap(err))
            }
    }

    #[inline]
    fn raw(&self) -> *mut ffi::SecretServiceFFI {
        self.0.to_glib_none() as *mut ffi::SecretServiceFFI
    }

    /// Returns if a session to the SecretService is currently established.
    pub fn is_session_established(&self) -> bool {
        let flags = unsafe {ffi::secret_service_get_flags(self.raw())};
        flags & SECRET_SERVICE_OPEN_SESSION != 0
    }

    /// Returns if the Service's collections are loaded.
    pub fn are_collections_loaded(&self) -> bool {
        let flags = unsafe {ffi::secret_service_get_flags(self.raw())};
        flags & SECRET_SERVICE_LOAD_COLLECTIONS != 0
    }

    /// Get the set of algorithms being used to transfer secrets between this secret service proxy and the Secret Service itself.
    /// Returns `None` if no session has been established yet.
    /// The contained String has the format "algorithm-algorithm-algorithm-..."
    pub fn get_session_algorithms(&self) -> Option<String> {
        unsafe{
            let res_c = ffi::secret_service_get_session_algorithms(self.raw());
            if res_c.is_null(){
                None
            } else {
                Some(FromGlibPtr::from_glib_none(res_c))
            }
        }
    }

    /// Get the collections of the Service.
    /// A collection contains multiple SecretItems.
    pub fn get_collections(&self) -> Option<Vec<SecretCollection>> {
        unsafe {
            let glist = ffi::secret_service_get_collections(self.raw());
            if glist.is_null(){
                None
            } else {
                Some(FromGlibPtrContainer::from_glib_none(glist))
            }
        }
    }

    /*
    pub fn search() -> Vec<SecretItem> {
        unimplemented!()
    }
    */

    /*
    pub fn store () -> bool {

    }
    */

    /*
    pub fn lookup () -> SecretValue {

    }
    */

    /*
    pub fn clear () -> bool {

    }
    */

    /*
    pub fn set_alias(&str alias, ) -> bool {
        //FIXME: actually we should put this into SecretCollection
    }
    */

    /// Ensures that a session is established.
    pub fn ensure_session(&self) -> SecretResult<()> {
        unsafe {
            let mut err = ptr::null_mut();
            ffi::secret_service_ensure_session_sync(self.raw(), ptr::null_mut(), &mut err);
            if err.is_null() {
                Ok(())
            } else {
                Err(Error::wrap(err))
            }
        }
    }

    /// Ensures that the collections are loaded.
    pub fn load_collections(&self) -> SecretResult<()> {
        unsafe {
            let mut err = ptr::null_mut();
            ffi::secret_service_load_collections_sync(self.raw(), ptr::null_mut(), &mut err);
            if err.is_null() {
                Ok(())
            } else {
                Err(Error::wrap(err))
            }
        }
    }
}

impl StaticType for SecretService {
    fn static_type() -> Type{
        Type::BaseObject //TODO get this from libsecret
    }
}

impl Wrapper for SecretService {
    type GlibType = ffi::SecretServiceFFI;

    unsafe fn wrap(r: Ref) -> Self{
        SecretService(r)
    }

    fn as_ref(&self) -> &Ref{
        &self.0
    }

    fn unwrap(self) -> Ref{
        self.0
    }
}

#[allow(dead_code)]
const SECRET_SERVICE_NONE: i32              = 0;
const SECRET_SERVICE_OPEN_SESSION: i32      = 1 << 1;
const SECRET_SERVICE_LOAD_COLLECTIONS: i32  = 1 << 2;

#[allow(dead_code)]
const SECRET_SEARCH_NONE: i32               = 0;
#[allow(dead_code)]
const SECRET_SEARCH_ALL: i32                = 1 << 1;
#[allow(dead_code)]
const SECRET_SEARCH_UNLOCK: i32             = 1 << 2;
#[allow(dead_code)]
const SECRET_SEARCH_LOAD_SECRETS: i32       = 1 << 3;
