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

pub type Result<Succ> = RResult<Succ, IOError>;

impl<FE: FileExt> FSLock<FE> {

    pub fn new(f: FE) -> FSLock<FE> {
        FSLock {
            locked_obj: RwLock::new(f),
        }
    }

    pub fn read(&self) -> Result<LockResult<RwLockReadGuard<FE>>> {
        let read = self.locked_obj.read();
        if let Ok(r) = read {
            let e = r.deref().lock_shared();
            if e.is_err() {
                return Err(e.err().unwrap());
            } else {
                Ok(Ok(r))
            }
        } else {
            Ok(read)
        }
    }

    pub fn try_read(&self) -> Result<TryLockResult<RwLockReadGuard<FE>>> {
        let read = self.locked_obj.try_read();
        if let Ok(r) = read {
            let e = r.deref().try_lock_shared();
            if e.is_err() {
                return Err(e.err().unwrap());
            } else {
                Ok(Ok(r))
            }
        } else {
            Ok(read)
        }
    }

    pub fn write(&self) -> Result<LockResult<RwLockWriteGuard<FE>>> {
        let read = self.locked_obj.write();
        if let Ok(r) = read {
            let e = r.deref().lock_exclusive();
            if e.is_err() {
                return Err(e.err().unwrap());
            } else {
                Ok(Ok(r))
            }
        } else {
            Ok(read)
        }
    }

    pub fn try_write(&self) -> Result<TryLockResult<RwLockWriteGuard<FE>>> {
        let read = self.locked_obj.try_write();
        if let Ok(r) = read {
            let e = r.deref().try_lock_exclusive();
            if e.is_err() {
                return Err(e.err().unwrap());
            } else {
                Ok(Ok(r))
            }
        } else {
            Ok(read)
        }
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

#[cfg(test)]
mod test {

    extern crate tempdir;

    use std::sync::Arc;
    use std::fs::File;

    use super::FSLock;

    fn get_new_testing_file() -> Arc<FSLock<File>> {
        use std::fs::OpenOptions;
        let tempdir = tempdir::TempDir::new("flocking").unwrap();
        let path    = tempdir.path().join("flocking");
        let file    = OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .create(true)
                                    .open(&path)
                                    .unwrap();
        Arc::new(FSLock::new(file))
    }

    #[test]
    fn test_threaded_rw() {
        use std::io::Read;
        use std::io::Write;
        use std::ops::Deref;
        use std::thread;

        let lock = get_new_testing_file();

        let t1_lock = lock.clone();
        let t1 = thread::spawn(move || {
            for i in 0..25 {
                t1_lock.write()
                    .map(|o| {
                        o.map(|file| file.deref().write_fmt(format_args!("{}", i)))
                    }).ok();
            };
        });

        let t2_lock = lock.clone();
        let t2 = thread::spawn(move || {
            for _ in 0..50 {
                let mut s = String::new();
                t2_lock.read()
                    .map(|o| {
                        o.map(|file| file.deref().read_to_string(&mut s))
                    }).ok();
            };
        });

        assert!(t1.join().is_ok());
        assert!(t2.join().is_ok());
    }

}

