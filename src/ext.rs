/// Checks whether bytes or a string are nul terminated.
pub trait IsNulTerminated {
    fn is_nul_terminated(&self) -> bool;
}

impl IsNulTerminated for str {
    #[inline]
    fn is_nul_terminated(&self) -> bool {
        self.as_bytes().is_nul_terminated()
    }
}

impl IsNulTerminated for [u8] {
    #[inline]
    fn is_nul_terminated(&self) -> bool {
        self.last().cloned() == Some(0)
    }
}

impl<const N: usize> IsNulTerminated for [u8; N] {
    #[inline]
    fn is_nul_terminated(&self) -> bool {
        self.last().cloned() == Some(0)
    }
}
