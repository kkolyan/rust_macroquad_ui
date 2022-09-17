use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::iter::Map;
use std::slice::{Iter, IterMut};

#[derive(Clone, Debug)]
pub struct Composite {
    components: HashMap<TypeId, Vec<TypeBox>>,
}

impl Composite {
    pub fn new() -> Composite {
        Composite {
            components: HashMap::new()
        }
    }

    pub fn get<T: 'static>(&self) -> Option<Map<Iter<TypeBox>, for<'r> fn(&'r TypeBox) -> &'r T>> {
        let type_id = TypeId::of::<T>();
        let components = self.components.get(&type_id);
        if let Some(components) = components {
            return Some(components.iter()
                .map(|it| it.raw.downcast_ref::<T>().unwrap())
            );
        }
        None
    }

    pub fn get_mut<T: Default + Clone + Debug + 'static>(&mut self) -> Map<IterMut<TypeBox>, fn(&mut TypeBox) -> &mut T> {
        let type_id = TypeId::of::<T>();
        let entry = self.components
            .entry(type_id)
            .or_insert_with(Vec::new);
        entry.push(Self::create_type_box::<T>(Default::default()));
        Self::boxes_to_refs_mut::<T>(entry)
    }

    fn boxes_to_refs_mut<T: 'static>(it: &mut Vec<TypeBox>) -> Map<IterMut<TypeBox>, fn(&mut TypeBox) -> &mut T> {
        it.iter_mut().map(|it| it.raw.downcast_mut::<T>().unwrap())
    }

    fn create_type_box<T: Debug + Clone + 'static>(value: T) -> TypeBox {
        TypeBox {
            raw: Box::<T>::new(value),
            cloner: |it| {
                Self::create_type_box::<T>(it.raw.downcast_ref::<T>().unwrap().clone())
            },
            debugger: |it, f| it.raw.downcast_ref::<T>().unwrap().fmt(f),
        }
    }

    // pub fn remove<T: Clone + Debug + Sized + 'static>(&mut self) -> Option<T> {
    //     let type_id = TypeId::of::<T>();
    //     self.components.remove(&type_id).map(|it| *it.raw.downcast::<T>().unwrap())
    // }

    pub fn insert<T: Clone + Debug + 'static>(&mut self, value: T) {
        let type_id = TypeId::of::<T>();
        let entry = self.components
            .entry(type_id)
            .or_insert_with(Vec::new);
        entry.push(Self::create_type_box::<T>(value));
    }
}

pub struct TypeBox {
    raw: Box<dyn Any>,
    cloner: fn(&TypeBox) -> TypeBox,
    debugger: fn(&TypeBox, f: &mut Formatter<'_>) -> std::fmt::Result,
}

impl Clone for TypeBox {
    fn clone(&self) -> Self {
        let cloner = self.cloner;
        cloner(self)
    }
}

impl Debug for TypeBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let d = self.debugger;
        d(self, f)
    }
}

