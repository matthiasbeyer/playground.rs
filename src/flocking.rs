use std::io::Error as IOError;
use std::ops::Deref;
use std::sync::LockResult;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use std::sync::TryLockResult;
use std::result::Result as RResult;

use fs2::FileExt;

pub struct FSLock<FE: FileExt> {
    locked_obj: RwLock<FE>,
}

pub type Result<E> = RResult<RResult<(), IOError>, E>;

impl<FE: FileExt> FSLock<FE> {

    pub fn new(f: FE) -> FSLock<FE> {
        FSLock {
            locked_obj: RwLock::new(f),
        }
    }

    pub fn read(&self) -> Result<LockResult<RwLockReadGuard<FE>>> {
        self.locked_obj
            .read()
            .map(|obj| obj.deref().lock_shared())
            .map_err(|e| Err(e))
    }

    pub fn try_read(&self) -> Result<TryLockResult<RwLockReadGuard<FE>>> {
        self.locked_obj
            .try_read()
            .map(|obj| obj.deref().try_lock_shared())
            .map_err(|e| Err(e))
    }

    pub fn write(&self) -> Result<LockResult<RwLockWriteGuard<FE>>> {
        self.locked_obj
            .write()
            .map(|obj| obj.deref().lock_exclusive())
            .map_err(|e| Err(e))
    }

    pub fn try_write(&self) -> Result<TryLockResult<RwLockWriteGuard<FE>>> {
        self.locked_obj
            .try_write()
            .map(|obj| obj.deref().try_lock_exclusive())
            .map_err(|e| Err(e))
    }

    pub fn is_poisoned(&self) -> bool {
        self.locked_obj.is_poisoned()
    }

    // pub fn get_mut(&mut self) -> Result<LockResult<&mut FE>> {
    //     self.locked_obj
    //         .get_mut()
    //         .map(|obj| obj.deref().lock_exclusive())
    //         .map_err(|e| Err(e))
    // }

}

impl<FE: FileExt> Drop for FSLock<FE> {

    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.locked_obj.write().map(|obj| obj.deref().unlock());
    }

}

