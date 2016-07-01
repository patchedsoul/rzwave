macro_rules! def_any {
    ($name:ident : $bound:ident) => {
        pub struct $name {
            type_id: ::std::any::TypeId,
            object: Box<$bound>,
        }

        impl $name {
            pub fn new<T: $bound>(object: T) -> Self {
                $name {
                    type_id: ::std::any::TypeId::of::<T>(),
                    object: Box::new(object),
                }
            }

            pub fn is<T: $bound>(&self) -> bool {
                ::std::any::TypeId::of::<T>() == self.type_id
            }

            pub fn downcast_ref<T: $bound>(&self) -> Option<&T> {
                if self.is::<T>() {
                    unsafe {
                        let fat_ptr: ::std::raw::TraitObject = ::std::mem::transmute(::std::ops::Deref::deref(self));

                        Some(&*(fat_ptr.data as *const T))
                    }
                }
                else {
                    None
                }
            }

            pub fn downcast<T: $bound>(self) -> Result<Box<T>, Self> {
                if self.is::<T>() {
                    unsafe {
                        let raw = Box::into_raw(self.object);
                        let fat_ptr: ::std::raw::TraitObject = ::std::mem::transmute(raw);

                        Ok(Box::from_raw(fat_ptr.data as *mut T))
                    }
                }
                else {
                    Err(self)
                }
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.object, fmt)
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = $bound;

            fn deref(&self) -> &$bound {
                ::std::ops::Deref::deref(&self.object)
            }
        }

        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $bound {
                ::std::ops::DerefMut::deref_mut(&mut self.object)
            }
        }

        impl ::std::borrow::Borrow<$bound> for $name {
            fn borrow(&self) -> &$bound {
                ::std::borrow::Borrow::borrow(&self.object)
            }
        }

        impl ::std::borrow::BorrowMut<$bound> for $name {
            fn borrow_mut(&mut self) -> &mut $bound {
                ::std::borrow::BorrowMut::borrow_mut(&mut self.object)
            }
        }
    }
}
