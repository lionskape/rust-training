#![forbid(unsafe_code)]

use std::{
    fs::{self},
    io::{self, Error, ErrorKind, Read},
    ops::Add,
    path::Path,
};

////////////////////////////////////////////////////////////////////////////////

type Callback<'a> = dyn FnMut(&mut Handle) + 'a;

#[derive(Default)]
pub struct Walker<'a> {
    callbacks: Vec<Box<Callback<'a>>>,
}

impl<'a> Walker<'a> {
    pub fn new() -> Self {
        Self { callbacks: vec![] }
    }

    pub fn add_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Handle) + 'a,
    {
        self.callbacks.push(Box::new(callback))
    }

    pub fn walk<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        if self.callbacks.is_empty() {
            return Ok(());
        }
        match Self::rec_walk(path.as_ref(), &mut self.callbacks) {
            Ok(()) => (),
            Err(err) => {
                return Err(err);
            }
        }
        Ok(())
    }

    fn rec_walk(path: &Path, callback: &mut [Box<Callback>]) -> io::Result<()> {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                if entry.is_err() {
                    return Err(Error::new(ErrorKind::NotFound, "entry not found"));
                }
                let node = entry.unwrap();
                let p = node.path();
                let mut h: Handle = match (path.is_file(), path.is_dir()) {
                    (true, false) => Handle::File(FileHandle {
                        path: &p,
                        read: false,
                    }),
                    (false, true) => Handle::Dir(DirHandle {
                        path: &p,
                        descend: false,
                    }),
                    _ => continue,
                };
                for i in 0..callback.len() {
                    (callback[i])(&mut h);

                    let mut limit: usize = 0;

                    let is_checked = if let Handle::Dir(ref mut d) = h {
                        let res = d.descend;
                        d.descend = false;
                        res
                    } else if let Handle::File(ref mut f) = h {
                        let res = f.read;
                        f.read = false;
                        res
                    } else {
                        false
                    };

                    if is_checked {
                        if limit < i {
                            callback.swap(limit, i);
                        }
                        limit = limit.add(1);
                    }
                    if limit != 0 {
                        if let Handle::Dir(ref d) = h {
                            match Self::rec_walk(d.path(), &mut callback[..limit]) {
                                Ok(()) => {}
                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        } else if let Handle::File(ref f) = h {
                            let mut file = match fs::File::open(f.path()) {
                                Ok(ok) => ok,
                                Err(err) => {
                                    return Err(err);
                                }
                            };
                            let mut buffer = Vec::new();
                            // read the whole file
                            match file.read_to_end(&mut buffer) {
                                Ok(_) => {}
                                Err(err) => {
                                    return Err(err);
                                }
                            }
                            for cb in callback[..limit].iter_mut() {
                                (cb)(&mut Handle::Content {
                                    file_path: f.path(),
                                    content: &buffer,
                                });
                            }
                        }
                    }
                }
            }
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NotFound, "dir not found"))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub enum Handle<'a> {
    Dir(DirHandle<'a>),
    File(FileHandle<'a>),
    Content {
        file_path: &'a Path,
        content: &'a [u8],
    },
}

pub struct DirHandle<'a> {
    path: &'a Path,
    descend: bool,
}

impl<'a> DirHandle<'a> {
    pub fn descend(&mut self) {
        self.descend = true;
    }

    pub fn path(&self) -> &Path {
        self.path
    }
}

pub struct FileHandle<'a> {
    path: &'a Path,
    read: bool,
}

impl<'a> FileHandle<'a> {
    pub fn read(&mut self) {
        self.read = true;
    }

    pub fn path(&self) -> &Path {
        self.path
    }
}
