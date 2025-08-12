use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    ops::{Deref, DerefMut},
    sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError},
};

pub trait Inner {
    type Inner;
    type Guard<'a>: Deref<Target = Self::Inner>
    where
        Self: 'a,
        Self::Inner: 'a;
    type Error<'a>
    where
        Self: 'a,
        Self::Inner: 'a;

    fn inner(&self) -> Result<Self::Guard<'_>, Self::Error<'_>>;
}

pub trait InnerMut: Inner {
    type GuardMut<'a>: DerefMut<Target = Self::Inner>
    where
        Self: 'a,
        Self::Inner: 'a;
    type Error<'a>
    where
        Self: 'a,
        Self::Inner: 'a;

    fn inner_mut(&self) -> Result<Self::GuardMut<'_>, <Self as InnerMut>::Error<'_>>;
}

impl<T> Inner for RefCell<T> {
    type Inner = T;
    type Guard<'a>
        = Ref<'a, Self::Inner>
    where
        Self: 'a,
        Self::Inner: 'a;
    type Error<'a>
        = BorrowError
    where
        Self: 'a,
        Self::Inner: 'a;

    fn inner(&self) -> Result<Self::Guard<'_>, Self::Error<'_>> {
        self.try_borrow()
    }
}

impl<T> InnerMut for RefCell<T> {
    type GuardMut<'a>
        = RefMut<'a, Self::Inner>
    where
        Self: 'a,
        Self::Inner: 'a;
    type Error<'a>
        = BorrowMutError
    where
        Self: 'a,
        Self::Inner: 'a;

    fn inner_mut(&self) -> Result<Self::GuardMut<'_>, <Self as InnerMut>::Error<'_>> {
        self.try_borrow_mut()
    }
}

impl<T> Inner for Mutex<T> {
    type Inner = T;
    type Guard<'a>
        = MutexGuard<'a, T>
    where
        Self: 'a,
        Self::Inner: 'a;
    type Error<'a>
        = TryLockError<Self::Guard<'a>>
    where
        Self: 'a,
        Self::Inner: 'a;

    fn inner(&self) -> Result<Self::Guard<'_>, Self::Error<'_>> {
        self.try_lock()
    }
}

impl<T> InnerMut for Mutex<T> {
    type GuardMut<'a>
        = MutexGuard<'a, T>
    where
        Self: 'a,
        Self::Inner: 'a;
    type Error<'a>
        = TryLockError<Self::Guard<'a>>
    where
        Self: 'a,
        Self::Inner: 'a;

    fn inner_mut(&self) -> Result<Self::GuardMut<'_>, <Self as InnerMut>::Error<'_>> {
        self.try_lock()
    }
}

impl<T> Inner for RwLock<T> {
    type Inner = T;
    type Guard<'a>
        = RwLockReadGuard<'a, T>
    where
        Self: 'a,
        Self::Inner: 'a;
    type Error<'a>
        = TryLockError<Self::Guard<'a>>
    where
        Self: 'a,
        Self::Inner: 'a;

    fn inner(&self) -> Result<Self::Guard<'_>, Self::Error<'_>> {
        self.try_read()
    }
}

impl<T> InnerMut for RwLock<T> {
    type GuardMut<'a>
        = RwLockWriteGuard<'a, T>
    where
        Self: 'a,
        Self::Inner: 'a;
    type Error<'a>
        = TryLockError<Self::GuardMut<'a>>
    where
        Self: 'a,
        Self::Inner: 'a;

    fn inner_mut(&self) -> Result<Self::GuardMut<'_>, <Self as InnerMut>::Error<'_>> {
        self.try_write()
    }
}
