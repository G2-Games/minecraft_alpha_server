use std::ops;

pub trait ToBytes: Sized {
    /// A byte array which can store a packed representation of this type.
    type Bytes: ByteArray;

    fn to_bytes(self) -> Self::Bytes;
}

mod private {
    pub trait ByteArray {}

    impl<const N: usize> ByteArray for [u8; N] {}
}


pub trait ByteArray:
    private::ByteArray
    + ops::IndexMut<usize, Output = u8>
    + ops::IndexMut<ops::Range<usize>, Output = [u8]>
    + AsRef<[u8]>
    + AsMut<[u8]>
{
    /// The length of this byte array.
    const SIZE: usize;

    /// Return the array with all zeros.
    /// Cannot use `Default` as it is not implemented for all array sizes.
    fn zeroed() -> Self;
}

impl<const N: usize> ByteArray for [u8; N] {
    const SIZE: usize = N;

    fn zeroed() -> Self {
        [0; N]
    }
}
