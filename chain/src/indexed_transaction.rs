use crate::hash::H256;
use crate::read_and_hash::ReadAndHash;
use crate::transaction::Transaction;
use heapsize::HeapSizeOf;
use ser::{Deserializable, Error as ReaderError, Reader};
// use serde::{Deserialize, Serialize};
use std::{cmp, fmt, io};

#[derive(Default, Clone)]
pub struct IndexedTransaction {
    pub hash: H256,
    pub raw: Transaction,
}

impl fmt::Debug for IndexedTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IndexedTransaction")
            .field("hash", &self.hash.reversed())
            .field("raw", &self.raw)
            .finish()
    }
}

#[cfg(feature = "test-helpers")]
impl<T> From<T> for IndexedTransaction
where
    Transaction: From<T>,
{
    fn from(other: T) -> Self {
        Self::from_raw(other)
    }
}

impl HeapSizeOf for IndexedTransaction {
    fn heap_size_of_children(&self) -> usize {
        self.raw.heap_size_of_children()
    }
}

impl IndexedTransaction {
    pub fn new(hash: H256, transaction: Transaction) -> Self {
        IndexedTransaction {
            hash: hash,
            raw: transaction,
        }
    }

    // Explicit conversion of the raw Transaction into IndexedTransaction.
    // Hashes transaction contents.
    // pub fn from_raw<T>(transaction: T) -> Self
    // where
    //     Transaction: From<T>,
    // {
    //     let transaction = Transaction::from(transaction);
    //     Self::new(transaction_hash(&transaction), transaction)
    // }
}

impl cmp::PartialEq for IndexedTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

// impl Deserializable for IndexedTransaction {
//     fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, ReaderError>
//     where
//         T: io::Read,
//     {
//         let data = r#try!(reader.read_and_hash::<Transaction>());
//         // TODO: use len
//         let tx = IndexedTransaction {
//             raw: data.data,
//             hash: data.hash,
//         };

//         Ok(tx)
//     }
// }
