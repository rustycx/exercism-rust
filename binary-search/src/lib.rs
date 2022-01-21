use std::cmp::Ordering;

pub fn find<R, T>(array: R, key: T) -> Option<usize>
where
    T: Ord,
    R: AsRef<[T]>,
{
    let array = array.as_ref();
    let (mut i, mut j) = (0, array.len());
    while i < j {
        let mid = i + (j - i) / 2;
        if key < array[mid] {
            j = mid;
        } else if key > array[mid] {
            i = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}

pub fn find1<R: AsRef<[T]>, T: Ord>(space: R, key: T) -> Option<usize> {
    let space = space.as_ref();
    let mid = space.len() / 2;
    match key.cmp(space.get(mid)?) {
        Ordering::Equal => Some(mid),
        Ordering::Less => find(&space[..mid], key),
        Ordering::Greater => find(&space[mid + 1..], key).map(|i| i + mid + 1),
    }
}
