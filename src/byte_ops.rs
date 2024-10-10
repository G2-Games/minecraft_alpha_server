use std::ops;

pub trait ToBytes: Sized {
    /// A byte array which can store a packed representation of this type.
    type Bytes: ByteArray;

    fn to_bytes(self) -> Self::Bytes;
}

mod private {
    pub trait ByteArray {}

    impl<const N: usize> ByteArray for [u8; N] {}
    impl ByteArray for Vec<u8> {}
}


pub trait ByteArray:
    private::ByteArray
    + ops::IndexMut<usize, Output = u8>
    + ops::IndexMut<ops::Range<usize>, Output = [u8]>
    + AsRef<[u8]>
    + AsMut<[u8]>
{
    /// Return the array with all zeros.
    /// Cannot use `Default` as it is not implemented for all array sizes.
    fn zeroed() -> Self;
}

impl<const N: usize> ByteArray for [u8; N] {
    fn zeroed() -> Self {
        [0; N]
    }
}

impl ByteArray for Vec<u8> {
    fn zeroed() -> Self {
        Vec::new()
    }
}
