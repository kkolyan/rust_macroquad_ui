use std::any::{Any};
use std::fmt::{Debug, Formatter};

pub struct AnyBox {
    // type_id: TypeId,
    raw: Box<dyn Any>,
    cloner: fn(&AnyBox) -> AnyBox,
    debugger: fn(&AnyBox, f: &mut Formatter<'_>) -> std::fmt::Result,
}

impl AnyBox {
    pub fn new<T: Debug + Clone + 'static>(value: T) -> AnyBox {
        AnyBox {
            // type_id: TypeId::of::<T>(),
            raw: Box::<T>::new(value),
            cloner: |it| {
                AnyBox::new::<T>(it.raw.downcast_ref::<T>().unwrap().clone())
            },
            debugger: |it, f| it.raw.downcast_ref::<T>().unwrap().fmt(f),
        }
    }

    pub fn cast_ref<T: Debug + Clone + 'static>(&self) -> Option<&T> {
        self.raw.downcast_ref::<T>()
    }

    pub fn cast_mut<T: Debug + Clone + 'static>(&mut self) -> Option<&mut T> {
        self.raw.downcast_mut::<T>()
    }

    pub fn cast<T: Debug + Clone + 'static>(self) -> Option<T> {
        self.raw.downcast::<T>().ok().map(|it| *it)
    }
}

impl Clone for AnyBox {
    fn clone(&self) -> Self {
        let cloner = self.cloner;
        cloner(self)
    }
}

impl Debug for AnyBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let d = self.debugger;
        d(self, f)
    }
}

#[macro_export] macro_rules! make_bounded_any_box {
    ($vis: vis, $struct_name: ident $(<$struct_name_param:ident>)?, $bound: ident $(<$bound_param:ident>)?) => {

        #[derive(core::clone::Clone)]
        $vis struct $struct_name$(<$struct_name_param>)? {
            $(pd: std::marker::PhantomData<$bound_param>,)?
            target: crate::any_box::AnyBox,
            dyn_caster: fn(&$struct_name$(<$struct_name_param>)?) -> &dyn $bound$(<$bound_param>)?,
        }

        impl $(<$struct_name_param>)? std::fmt::Debug for $struct_name$(<$struct_name_param>)? {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.target.fmt(f)
            }
        }

        impl $(<$struct_name_param>)? $struct_name$(<$struct_name_param>)? {
            pub fn new<T: Debug + Clone + 'static + $bound$(<$bound_param>)?>(value: T) -> $struct_name$(<$struct_name_param>)? {
                $struct_name {
                    $(pd: std::marker::PhantomData::<$bound_param>::default(),)?
                    target: crate::any_box::AnyBox::new(value),
                    dyn_caster: |it| unsafe { &*(it.cast_ref::<T>().unwrap() as *const dyn $bound$(<$bound_param>)?) },
                }
            }

            pub fn cast_ref<T: Debug + Clone + 'static + $bound$(<$bound_param>)?>(&self) -> Option<&T> {
                self.target.cast_ref::<T>()
            }
            //
            // pub fn cast_mut<T: Debug + Clone + 'static + $bound$(<$bound_param>)?>(&mut self) -> Option<&mut T> {
            //     self.target.cast_mut::<T>()
            // }
            //
            // pub fn cast<T: Debug + Clone + 'static + $bound$(<$bound_param>)?>(self) -> Option<T> {
            //     self.target.cast::<T>()
            // }

            pub fn as_ref(&self) -> &dyn $bound$(<$bound_param>)? {
                let caster = self.dyn_caster;
                caster(self)
            }
        }
    };
}
