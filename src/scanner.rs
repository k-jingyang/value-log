// Copyright (c) 2024-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

use crate::{id::SegmentId, Compressor, ValueHandle, ValueLog};
use std::{collections::BTreeMap, sync::MutexGuard};

#[derive(Debug, Default)]
pub struct SegmentCounter {
    pub size: u64,
    pub item_count: u64,
}

pub type SizeMap = BTreeMap<SegmentId, SegmentCounter>;

pub struct Scanner<'a, I: Iterator<Item = std::io::Result<(ValueHandle, u32)>>> {
    iter: I,

    #[allow(unused)]
    lock_guard: MutexGuard<'a, ()>,

    size_map: SizeMap,
}

impl<'a, I: Iterator<Item = std::io::Result<(ValueHandle, u32)>>> Scanner<'a, I> {
    pub fn new<C: Compressor + Clone>(vlog: &'a ValueLog<C>, iter: I) -> Self {
        Self {
            iter,
            lock_guard: vlog.rollover_guard.lock().expect("lock is poisoned"),
            size_map: BTreeMap::default(),
        }
    }

    pub fn finish(self) -> SizeMap {
        self.size_map
    }

    pub fn scan(&mut self) -> crate::Result<()> {
        for vhandle in self.iter.by_ref() {
            let (vhandle, size) = vhandle.map_err(|_| {
                crate::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Index returned error",
                ))
            })?;
            let size = u64::from(size);

            self.size_map
                .entry(vhandle.segment_id)
                .and_modify(|x| {
                    x.item_count += 1;
                    x.size += size;
                })
                .or_insert_with(|| SegmentCounter {
                    size,
                    item_count: 1,
                });
        }

        Ok(())
    }
}
