use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct IndexVec<T> {
    data: Vec<T>,
}

impl<T> IndexVec<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self { data: Vec::with_capacity(cap) }
    }

    pub fn push(&mut self, value: T) -> u32 {
        let id = self.data.len() as u32;
        self.data.push(value);
        id
    }

    pub fn get(&self, id: u32) -> Option<&T> {
        self.data.get(id as usize)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut T> {
        self.data.get_mut(id as usize)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, &T)> {
        self.data.iter().enumerate().map(|(i, v)| (i as u32, v))
    }
}

impl<T> Index<u32> for IndexVec<T> {
    type Output = T;

    fn index(&self, id: u32) -> &Self::Output {
        &self.data[id as usize]
    }
}

impl<T> IndexMut<u32> for IndexVec<T> {
    fn index_mut(&mut self, id: u32) -> &mut Self::Output {
        &mut self.data[id as usize]
    }
}

impl<T> AsRef<[T]> for IndexVec<T> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<T> AsMut<[T]> for IndexVec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}
impl<T> IndexVec<T> {
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }
}
#[macro_export]
macro_rules! index_vec {
    ($value:expr; $n:expr) => {{
        let mut v = IndexVec::with_capacity($n);
        for _ in 0..$n {
            v.push($value.clone());
        }
        v
    }};

    ($($elem:expr),* $(,)?) => {{
        let mut v = IndexVec::new();
        $(
            v.push($elem);
        )*
        v
    }};
}
