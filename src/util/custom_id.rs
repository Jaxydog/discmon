use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomId<'c> {
    pub base: &'c str,
    pub name: &'c str,
    pub data: Vec<&'c str>,
}

impl<'c> CustomId<'c> {
    pub const fn new(base: &'c str, name: &'c str) -> Self {
        let data = vec![];

        Self { base, name, data }
    }
    pub fn try_resolve(head: &'c str) -> Result<Self> {
        let Some([base, name]) = head.split('_').array_chunks().next() else {
            return Err(anyhow!("invalid custom identifier header"));
        };

        Ok(Self::new(base, name))
    }

    pub fn push_data(&mut self, arg: &'c str) -> Result<()> {
        let length = self.to_string().len() + arg.len() + 1;

        if length > 64 {
            return Err(anyhow!("invalid custom identifier length ({length} > 64)"));
        }

        self.data.push(arg);
        Ok(())
    }
}

impl From<&CustomId<'_>> for String {
    fn from(value: &CustomId<'_>) -> Self {
        value.to_string()
    }
}

impl Display for CustomId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let CustomId { base, name, data } = self;

        if data.is_empty() {
            write!(f, "{base}_{name}")
        } else {
            let data = data.join(";");

            write!(f, "{base}_{name};{data}")
        }
    }
}
