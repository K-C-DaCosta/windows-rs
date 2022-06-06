use super::*;
//use bindings::*;
use windows_sys::Win32::System::WinRT::*;
use core::marker::PhantomData;

/// A type representing an agile reference to a COM/WinRT object.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct AgileReference<T>(ComPtr<IAgileReference>, PhantomData<T>);

impl<T: Interface> AgileReference<T> {
    /// Creates an agile reference to the object.
    pub fn new(object: &T) -> Result<Self> {
        //let unknown: &IUnknown = unsafe { std::mem::transmute(object) };
        //unsafe { RoGetAgileReference(AGILEREFERENCE_DEFAULT, &T::IID, unknown).map(|reference| Self(reference, Default::default())) }
        unsafe {
            let mut reference = ComPtr::<IAgileReference>::null();
            let code = RoGetAgileReference(AGILEREFERENCE_DEFAULT,  (&T::IID).into(), std::mem::transmute(object), reference.put()).into();
            if reference.is_null() {
                Err(Error{code, info: None })
            }   
             else {
                 Ok(Self(reference, Default::default()))
             }
        }
    }

    /// Retrieves a proxy to the target of the `AgileReference` object that may safely be used within any thread context in which get is called.
    pub fn resolve(&self) -> Result<T> {
       //unsafe { self.0.Resolve(self.0) }
       todo!()
    }
}

unsafe impl<T: Interface> Send for AgileReference<T> {}
unsafe impl<T: Interface> Sync for AgileReference<T> {}
