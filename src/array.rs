use core::{marker::PhantomData, mem::size_of};

use asr::{Address, Address64, Process};
use bytemuck::AnyBitPattern;

#[repr(C)]
#[derive(Copy, Clone, Debug, AnyBitPattern)]
pub struct CSharpArray<T> {
    address: Address64,
    phantom_data: PhantomData<T>,
}

impl<T: AnyBitPattern> CSharpArray<T> {
    pub fn new(address: Address64) -> Self {
        Self {
            address,
            phantom_data: PhantomData,
        }
    }
    /// Returns the number of elements in the current array
    pub fn count(&self, process: &Process) -> Result<usize, ()> {
        match process.read::<u32>(self.address + 0x18) {
            Ok(x) => Ok(x as usize),
            _ => Err(()),
        }
    }

    /// Reads the entire array
    pub fn read(&self, process: &Process) -> Result<Vec<T>, ()> {
        let count = self.count(process)?.min(2048);

        let mut buf = Vec::with_capacity(count);
        let uninit = buf.spare_capacity_mut();
        process
            .read_into_uninit_slice(self.address + 0x20, uninit)
            .map_err(|_| ())?;

        // SAFETY:
        // - len() is equal to the capacity of the Vec
        // - The elements of the buffer are initialized by the previous read_into_uninit_slice function
        unsafe {
            buf.set_len(count);
        }

        Ok(buf)
    }

    pub fn read_class<U>(
        &self,
        process: &Process,
        read_fn: impl Fn(&Process, Address) -> Result<U, ()>,
    ) -> Result<Vec<U>, ()>
    where
        T: Into<Address>,
    {
        self.read(process)?
            .into_iter()
            .map(|addr| read_fn(process, addr.into()).map_err(|_| ()))
            .collect()
    }

    pub fn iter<'a>(&'a self, process: &'a Process) -> impl DoubleEndedIterator<Item = T> + 'a {
        let count = self.count(process).unwrap_or_default();
        (0..count).filter_map(move |val| {
            process
                .read(self.address + 0x20 + val.wrapping_mul(size_of::<T>()) as u64)
                .ok()
        })
    }
}
