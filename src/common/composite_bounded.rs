
#[macro_export] macro_rules! make_bounded_composite {
    ($vis: vis, $struct_name: ident $(<$struct_name_param:ident>)?, $bound: ident $(<$bound_param:ident>)?) => {

        #[derive(core::clone::Clone, std::fmt::Debug)]
        $vis struct $struct_name$(<$struct_name_param>)? {
            target: crate::common::composite::Composite,
            ptrs: Vec<*const dyn $bound$(<$bound_param>)?>,
        }

        impl $(<$struct_name_param>)? $struct_name$(<$struct_name_param>)? {
            pub fn new() -> $struct_name$(<$struct_name_param>)? {
                $struct_name {
                    target: crate::common::composite::Composite::new(),
                    ptrs: vec![],
                }
            }

            pub fn get<T: $bound$(<$bound_param>)? + 'static>(&self) -> Option<
                std::iter::Map<
                    std::slice::Iter<crate::common::composite::TypeBox>,
                    for<'r> fn(&'r crate::common::composite::TypeBox) -> &'r T
                >
            > {
                self.target.get::<T>()
            }

            // pub fn remove<T: $bound$(<$bound_param>)? + Clone + std::fmt::Debug + Sized + 'static>(&mut self) -> Option<T> {
            //     if let Some(prev_index) = self.ptr_indices.get(&std::any::TypeId::of::<T>()).cloned() {
            //         self.ptr_indices.remove(&std::any::TypeId::of::<T>());
            //         self.ptrs.remove(prev_index);
            //         self.target.remove::<T>()
            //     } else {
            //         None
            //     }
            // }

            // pub fn put<T: $bound$(<$bound_param>)? + Clone + std::fmt::Debug + 'static>(&mut self, value: T) {
            //     self.remove::<T>();
            //     self.target.put::<T>(value);
            //     self.ptrs.push(self.target.get::<T>().unwrap());
            //     self.ptr_indices.insert(std::any::TypeId::of::<T>(), self.ptrs.len() - 1);
            // }

            pub fn insert<T: $bound$(<$bound_param>)? + Clone + std::fmt::Debug + 'static>(&mut self, value: T) {
                self.target.insert::<T>(value);
                self.ptrs.push(self.target.get::<T>().unwrap().last().unwrap());
            }

            pub fn iter<'a>(&'a self) -> std::iter::Map<
                std::slice::Iter<
                    'a,
                    *const dyn $bound$(<$bound_param>)?
                >,
                fn(
                    &'a *const dyn $bound$(<$bound_param>)?
                ) -> &'a dyn $bound$(<$bound_param>)?
            > {
                self.ptrs.iter().map(|value| unsafe { value.as_ref().unwrap() })
            }
        }
    };
}

// trait TestTrait {}

// make_bounded_composite!(,TestTraitComposite, TestTrait);