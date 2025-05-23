use std::{fmt::Debug, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Default)]
pub struct Entry<T> {
    pub zobrist: u64,
    pub value: T,
    pub depth: u8,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct TranspositionTable<T> {
    table: Vec<Option<Entry<T>>>,
    max_entries: usize,
}

impl<T: Debug + Clone + Copy + PartialEq + PartialOrd + Hash + Default> TranspositionTable<T> {
    /// Creat a new [`TranspositionTable`] with capacity for the next power of two given the number of entries.
    pub fn with_capacity(num_entries: usize) -> TranspositionTable<T> {
        let size = num_entries.next_power_of_two();

        TranspositionTable::<T> {
            table: vec![None; size],
            max_entries: size,
        }
    }

    /// Create a new [`TranspositionTable`] with the given the size in megabytes.
    pub fn with_size_mb(size_mb: usize) -> TranspositionTable<T> {
        let entry_size = std::mem::size_of::<Entry<T>>();
        let num_entries = (size_mb * 1024 * 1024) / entry_size;

        Self::with_capacity(num_entries)
    }

    fn index(&self, zobrist: u64) -> usize {
        (zobrist as usize) & (self.max_entries - 1)
    }

    /// Store the given value in the [`TranspositionTable`] provided `zobrist`, `depth` and the `value`.
    ///
    /// # Example
    /// ```
    /// use chessframe::{transpositiontable::{Entry, TranspositionTable}};
    ///
    /// let mut table = TranspositionTable::<i32>::with_capacity(64);
    ///
    /// table.store(0x123456789ABCDEF, -16, 4);
    ///
    /// assert_eq!(table.get(0x123456789ABCDEF), Some(&Entry { zobrist: 0x123456789ABCDEF, value: -16, depth: 4 }));
    /// ```
    pub fn store(&mut self, zobrist: u64, value: T, depth: u8) {
        let index = self.index(zobrist);

        let entry = Entry { zobrist, value, depth };

        match &self.table[index] {
            Some(existing) if existing.zobrist == zobrist => {
                if depth >= existing.depth {
                    self.table[index] = Some(entry);
                }
            }
            _ => self.table[index] = Some(entry),
        }
    }

    /// Get the value stored in the [`TranspositionTable`] provided `zobrist`.
    ///
    /// # Example
    /// ```
    /// use chessframe::{transpositiontable::{Entry, TranspositionTable}};
    ///
    /// let mut table = TranspositionTable::<i32>::with_capacity(64);
    ///
    /// table.store(0x10101010, 64, 8);
    ///
    /// assert_eq!(table.get(0x10101010), Some(&Entry { zobrist: 0x10101010, value: 64, depth: 8 }));
    /// ```
    pub fn get(&self, zobrist: u64) -> Option<&Entry<T>> {
        let index = self.index(zobrist);
        self.table[index].as_ref().filter(|e| e.zobrist == zobrist)
    }

    /// Clear the [`TranspositionTable`].
    ///
    /// # Example
    /// ```
    /// use chessframe::{transpositiontable::{Entry, TranspositionTable}};
    ///
    /// let mut table = TranspositionTable::<i32>::with_capacity(64);
    ///
    /// table.store(0x20202020, 0, 6);
    ///
    /// assert_eq!(table.get(0x20202020), Some(&Entry { zobrist: 0x20202020, value: 0, depth: 6 }));
    ///
    /// table.clear();
    ///
    /// assert_eq!(table.get(0x20202020), None);
    /// ```
    pub fn clear(&mut self) {
        self.table.fill(None);
    }
}
