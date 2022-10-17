pub use bumpalo::Bump;
use std::{fmt, ptr::{self, NonNull}};

pub struct LinkedList<'bmp, T: 'bmp> {
    bump: &'bmp Bump,
    next: Option<NonNull<LinkedList<'bmp, T>>>,
    before: Option<NonNull<LinkedList<'bmp, T>>>,

    pub value: NonNull<T>,
}

impl<'bmp, T> LinkedList<'bmp, T> {
    pub fn new_leaked(value: T) -> &'bmp mut Self {
        let bump = Box::leak(Box::new(Bump::new()));

        Self::new_with_bump(value, bump)
    }

    #[allow(clippy::mut_from_ref)]
    pub fn new_with_bump(value: T, bump: &'bmp Bump) -> &mut Self {
        let bump_value = bump.alloc_with(|| value);

        bump.alloc(Self {
            bump,
            value: unsafe { NonNull::new_unchecked(bump_value as *mut _) },
            next: None,
            before: None
        })
    }

    pub fn get_last_mut(&mut self) -> &mut LinkedList<'bmp, T> {
        let mut current = self;

        while let Some(mut next) = current.next {
            current = unsafe { next.as_mut() };
        };

        current
    }

    pub fn get_last(&self) -> &LinkedList<'bmp, T> {
        let mut current = self;

        while let Some(next) = current.next {
            current = unsafe { next.as_ref() };
        };

        current
    }

    pub fn get_first_mut(&mut self) -> &mut LinkedList<'bmp, T> {
        let mut current = self;

        while let Some(mut before) = current.before {
            current = unsafe { before.as_mut() };
        };

        current
    }

    pub fn get_first(&self) -> &LinkedList<'bmp, T> {
        let mut current = self;

        while let Some(before) = current.before {
            current = unsafe { before.as_ref() };
        };

        current
    }

    pub fn push_to_end(&mut self, value: T) -> &mut LinkedList<'bmp, T> {
        let mut next = unsafe {
            NonNull::new_unchecked(Self::new_with_bump(value, self.bump) as *mut LinkedList<'bmp, T>)
        };

        unsafe {
            next.as_mut().before = Some(NonNull::new_unchecked(self as *mut _));
        }

        self.get_last_mut().next = Some(next);

        unsafe { next.as_mut() }
    }

    pub fn push_to_front(&mut self, value: T) -> &mut LinkedList<'bmp, T> {
        let mut before = unsafe {
            NonNull::new_unchecked(Self::new_with_bump(value, self.bump) as *mut LinkedList<'bmp, T>)
        };

        self.before = Some(before);
        unsafe {
            before.as_mut().next = Some(NonNull::new_unchecked(self as *mut _));
        }

        unsafe { before.as_mut() }
    }

    pub fn iter(&self) -> Iter<'_, 'bmp, T> {
        Iter { next: Some(self) }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, 'bmp, T> {
        IterMut { next: Some(self) }
    }

    pub fn get(&self) -> &T {
        unsafe {
            self.value.as_ref()
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe {
            self.value.as_mut()
        }
    }

    pub fn get_n(&self, n: usize) -> Option<&T> {
        self.iter().nth(n)
    }

    pub fn get_n_mut(&mut self, n: usize) -> Option<&mut T> {
        self.iter_mut().nth(n)
    }

    pub fn pop_end(&mut self) -> T {
        let last = self.get_last_mut();

        unsafe {
            if let Some(mut before) = last.before {
                before.as_mut().next = None
            };
        };

        last.before = None;

        unsafe {
            ptr::read(last.value.as_ref())
        }
    }
}

pub struct Iter<'a, 'bmp, T> {
    next: Option<&'a LinkedList<'bmp, T>>
}

impl<'a, 'bmp, T> Iterator for Iter<'a, 'bmp, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|ll| {
            self.next = ll
                .next
                .map(|ll| unsafe { ll.as_ref() });

            unsafe { ll.value.as_ref() }
        })
    }
}

pub struct IterMut<'a, 'bmp, T> {
    next: Option<&'a mut LinkedList<'bmp, T>>
}

impl<'a, 'bmp, T> Iterator for IterMut<'a, 'bmp, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let ll = self.next.take()?;

        self.next = ll.next.map(|mut ll| unsafe { ll.as_mut() });

        unsafe { Some(ll.value.as_mut()) }
    }
}

impl<'bmp, T: fmt::Debug> fmt::Debug for LinkedList<'bmp, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // skip bump field

        f.debug_struct("LinkedList")
            .field("value", unsafe { self.value.as_ref() })
            .field("next", &self.next.map(|next| unsafe { next.as_ref() }))
            .finish()
    }
}
