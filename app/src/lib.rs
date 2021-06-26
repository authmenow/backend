#![deny(warnings)]
#![forbid(unsafe_code)]

mod oauth;
mod registrator;
mod session;

use hashbrown::HashMap;
use std::any::{Any, TypeId};
use std::hash::{BuildHasherDefault, Hasher};

use std::ops::Deref;
use std::sync::Arc;

type ServiceMap = HashMap<TypeId, Arc<dyn Any + Send + Sync>, BuildHasherDefault<TypeIdHasher>>;

#[derive(Debug, Clone, Default)]
struct TypeIdHasher(u64);

impl Hasher for TypeIdHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }

    #[inline]
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }
}

#[derive(Debug)]
pub struct Service<T: ?Sized>(Arc<T>);

impl<T> Service<T> {
    pub fn new(state: T) -> Service<T> {
        Service(Arc::new(state))
    }

    pub fn get_ref(&self) -> &T {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: ?Sized> Deref for Service<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Arc<T> {
        &self.0
    }
}

impl<T: ?Sized> Clone for Service<T> {
    fn clone(&self) -> Service<T> {
        Service(self.0.clone())
    }
}

impl<T: ?Sized> From<Arc<T>> for Service<T> {
    fn from(arc: Arc<T>) -> Self {
        Service(arc)
    }
}

#[derive(Debug, Default)]
pub struct AppBuilder {
    services: ServiceMap,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            services: ServiceMap::with_hasher(BuildHasherDefault::default()),
        }
    }

    pub fn with_service<T: 'static + Send + Sync + ?Sized>(mut self, service: Service<T>) -> Self {
        self.services
            .insert(TypeId::of::<Service<T>>(), Arc::new(service));
        self
    }

    pub fn build(self) -> App {
        App {
            services: self.services,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct App {
    services: ServiceMap,
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::new()
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        tracing::trace!("Trying to get {}", std::any::type_name::<T>());
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|rc| (&*rc).downcast_ref())
    }
}
