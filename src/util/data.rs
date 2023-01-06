use std::{
    fs::{create_dir_all, remove_file, File},
    io::Write,
    path::PathBuf,
    str::FromStr,
};

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Data<'res, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    dir: &'res str,
    key: &'res str,
    value: T,
}

impl<'res, T> Data<'res, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub const DIR: &str = "res";
    pub const EXT: &str = "rmp";

    pub fn root() -> PathBuf {
        PathBuf::from(Self::DIR)
    }
    pub fn dir_from(dir: &str) -> Result<PathBuf> {
        let dir = PathBuf::from_str(dir)?;

        Ok(Self::root().join(dir))
    }
    pub fn path_from(dir: &str, key: &str) -> Result<PathBuf> {
        let dir = Self::dir_from(dir)?;
        let key = PathBuf::from_str(key)?;

        Ok(dir.join(key).with_extension(Self::EXT))
    }

    pub const fn new(dir: &'res str, key: &'res str, value: T) -> Self {
        Self { dir, key, value }
    }
    pub fn load(dir: &'res str, key: &'res str) -> Result<Self> {
        let path = Self::path_from(dir, key)?;
        let file = File::open(path)?;
        let value = rmp_serde::from_read(file)?;

        Ok(Self::new(dir, key, value))
    }

    pub fn dir(&self) -> Result<PathBuf> {
        Self::dir_from(self.dir)
    }
    pub fn path(&self) -> Result<PathBuf> {
        Self::path_from(self.dir, self.key)
    }

    pub fn res_save(&self) -> Result<()> {
        let data = rmp_serde::to_vec(&self.value)?;

        create_dir_all(self.dir()?)?;

        let mut file = File::create(self.path()?)?;
        file.write_all(&data)?;
        file.flush().map_err(Into::into)
    }
    #[allow(clippy::missing_const_for_fn)]
    pub fn res_unwrap(self) -> T {
        self.value
    }
    pub fn res_delete(self) -> Result<T> {
        remove_file(self.path()?)?;

        Ok(self.res_unwrap())
    }
    pub fn res_rename(&mut self, dir: &'res str, key: &'res str) -> Result<()> {
        remove_file(self.path()?)?;

        self.dir = dir;
        self.key = key;
        self.res_save()
    }
    pub fn res_clone(&self, dir: &'res str, key: &'res str) -> Result<Self>
    where
        T: Clone,
    {
        let copied = Self::new(dir, key, self.value.clone());
        copied.res_save()?;

        Ok(copied)
    }
}

impl<T> Deref for Data<'_, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Data<'_, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
