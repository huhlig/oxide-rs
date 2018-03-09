//
// Copyright 2017 Hans W. Uhlig.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//!
//!
//!

/// Return value of [`Resources::fetch`].
///
/// [`Resources::fetch`]: struct.Resources.html#method.fetch
pub struct Fetch<'a, T: 'a> {
    inner: Ref<'a, Box<Resource>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Deref for Fetch<'a, T>
    where
        T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.inner.downcast_ref_unchecked() }
    }
}

/// Return value of [`Resources::fetch_mut`].
///
/// [`Resources::fetch_mut`]: struct.Resources.html#method.fetch_mut
pub struct FetchMut<'a, T: 'a> {
    inner: RefMut<'a, Box<Resource>>,
    phantom: PhantomData<&'a mut T>,
}

impl<'a, T> Deref for FetchMut<'a, T>
    where
        T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.inner.downcast_ref_unchecked() }
    }
}

impl<'a, T> DerefMut for FetchMut<'a, T>
    where
        T: Resource,
{
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.inner.downcast_mut_unchecked() }
    }
}


///
pub struct Resources {
    resources: FxHashMap<ResourceId, TrustCell<Box<Resource>>>,
}

impl Resources {
    /// Creates an empty new resource container.
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add<R>(&mut self, r: R)
        where
            R: Resource,
    {
        use std::collections::hash_map::Entry;

        let entry = self.resources.entry(ResourceId::new::<R>());

        if let Entry::Vacant(e) = entry {
            e.insert(TrustCell::new(Box::new(r)));
        } else {
            panic!("Tried to add a resource though \
                    an instance of this type already exists in `Resources`");
        }
    }


    pub fn exists(&self, id: ResourceId) -> bool {
        self.resources.contains_key(&id)
    }


    pub fn entry<R>(&mut self) -> Entry<R>
        where
            R: Resource,
    {
        create_entry(self.resources.entry(ResourceId::new::<R>()))
    }

    /// Like try_fetch except panics if unable to
    pub fn fetch<T>(&self) -> Fetch<T>
        where
            T: Resource,
    {
        self.try_fetch().expect(RESOURCE_NOT_FOUND)
    }

    pub fn fetch_mut<T>(&self) -> FetchMut<T>
        where
            T: Resource,
    {
        self.try_fetch_mut().expect(RESOURCE_NOT_FOUND)
    }

    pub fn try_fetch<T>(&self) -> Option<Fetch<T>>
        where
            T: Resource,
    {
        self.try_fetch_internal(TypeId::of::<T>()).map(|r| {
            Fetch {
                inner: r.borrow(),
                phantom: PhantomData,
            }
        })
    }

    /// Like `fetch_mut`, but returns an `Option` instead of panicking in the case of the resource
    /// being accessed mutably.
    pub fn try_fetch_mut<T>(&self) -> Option<FetchMut<T>>
        where
            T: Resource,
    {
        self.try_fetch_internal(TypeId::of::<T>()).map(|r| {
            FetchMut {
                inner: r.borrow_mut(),
                phantom: PhantomData,
            }
        })
    }

    /// Like `fetch_mut`, but returns an `Option` instead of panicking in the case of the resource
    /// being accessed mutably.
    pub fn try_fetch_mut_by_id<T>(&self) -> Option<FetchMut<T>>
        where
            T: Resource,
    {
        self.try_fetch_internal(TypeId::of::<T>()).map(|r| {
            FetchMut {
                inner: r.borrow_mut(),
                phantom: PhantomData,
            }
        })
    }

    fn try_fetch_internal(&self, id: TypeId) -> Option<&TrustCell<Box<Resource>>> {
        self.resources.get(&ResourceId(id))
    }
}