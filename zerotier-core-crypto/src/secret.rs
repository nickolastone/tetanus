// (c) 2020-2022 ZeroTier, Inc. -- currently propritery pending actual release and licensing. See LICENSE.md.

use std::ffi::c_void;

extern "C" {
    fn OPENSSL_cleanse(ptr: *mut c_void, len: usize);
}

/// Container for secrets that clears them on drop.
///
/// We can't be totally sure that things like libraries are doing this and it's
/// hard to get every use of a secret anywhere, but using this in our code at
/// least reduces the number of secrets that are left lying around in memory.
///
/// This is generally a low-risk thing since it's process memory that's protected,
/// but it's still not a bad idea due to things like swap or obscure side channel
/// attacks that allow memory to be read.
#[derive(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Secret<const L: usize>(pub [u8; L]);

impl<const L: usize> Secret<L> {
    #[inline(always)]
    pub fn new() -> Self {
        Self([0_u8; L])
    }

    /// Copy bytes into secret, will panic if size does not match.
    #[inline(always)]
    pub fn from_bytes(b: &[u8]) -> Self {
        Self(b.try_into().unwrap())
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8; L] {
        &self.0
    }

    #[inline(always)]
    pub fn first_n<const N: usize>(&self) -> &[u8; N] {
        assert!(N <= L);
        unsafe { &*self.0.as_ptr().cast() }
    }

    #[inline(always)]
    pub fn first_n_clone<const N: usize>(&self) -> Secret<N> {
        Secret::<N>(self.first_n().clone())
    }
}

impl<const L: usize> Drop for Secret<L> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { OPENSSL_cleanse(self.0.as_mut_ptr().cast(), L) };
    }
}

impl<const L: usize> Default for Secret<L> {
    #[inline(always)]
    fn default() -> Self {
        Self([0_u8; L])
    }
}

impl<const L: usize> AsRef<[u8]> for Secret<L> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const L: usize> AsRef<[u8; L]> for Secret<L> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8; L] {
        &self.0
    }
}

impl<const L: usize> AsMut<[u8]> for Secret<L> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl<const L: usize> AsMut<[u8; L]> for Secret<L> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u8; L] {
        &mut self.0
    }
}
