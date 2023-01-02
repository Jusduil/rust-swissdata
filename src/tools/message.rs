//! Some struct for manage translated message to user

use std::collections::BTreeMap;
use std::fmt;

/// Represent a T translated
///
/// Warning: FromIterator is implemented, but if iterator is empty, return a
/// Translated with T::default(), usually, this is not you want)
pub struct Translated<T> {
    contents: BTreeMap<String, T>,
    default: String,
}
impl<T> Translated<T> {
    /// Get the T for default language
    pub fn default(&self) -> &T {
        self.contents.get(&self.default).unwrap()
    }

    /// Get the T for the selected language if exists
    pub fn get<S: AsRef<str>>(&self, key: S) -> Option<&T> {
        self.contents.get(key.as_ref())
    }

    /// Get the T for the selected language or default language if not
    pub fn get_or_default<S: AsRef<str>>(&self, key: S) -> &T {
        self.get(key).unwrap_or_else(|| self.default())
    }
}
impl<T> Default for Translated<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            default: Default::default(),
            contents: [(Default::default(), T::default())].into_iter().collect(),
        }
    }
}
impl<T> fmt::Debug for Translated<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if fmt.alternate() {
            let mut dbg = fmt.debug_struct("Translated");
            for (k, v) in &self.contents {
                let k = if k == &self.default {
                    format!("{k}*")
                } else {
                    k.to_string()
                };
                dbg.field(k.as_str(), v);
            }
            dbg.finish()
        } else {
            fmt.debug_tuple("Translated").field(self.default()).finish()
        }
    }
}
impl<T> fmt::Display for Translated<T>
where
    T: fmt::Display,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.default().fmt(fmt)
    }
}

impl<T, U, S> FromIterator<(S, T)> for Translated<U>
where
    S: AsRef<str>,
    U: Default,
    T: Into<U>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (S, T)>,
    {
        let mut default = None;
        let mut contents: BTreeMap<String, U> = iter
            .into_iter()
            .map(|(lang, content)| {
                default.get_or_insert(lang.as_ref().to_string());
                (lang.as_ref().to_string(), content.into())
            })
            .collect();
        let default = match default {
            Some(default) => default,
            None => {
                contents.insert("".to_string(), U::default());
                "".into()
            }
        };
        Self { default, contents }
    }
}
impl<S> FromIterator<(S, S, S)> for Translated<Link<S>>
where
    S: AsRef<str> + Default,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (S, S, S)>,
    {
        iter.into_iter().map(|(a, b, c)| (a, (b, c))).collect()
    }
}

/// A link with description
#[derive(Default)]
pub struct Link<S> {
    name: S,
    url: S,
}
impl<S> Link<S> {
    /// Get the name to display
    pub fn name(&self) -> &S {
        &self.name
    }

    /// Url of link
    pub fn url(&self) -> &S {
        &self.url
    }
}
impl<S> fmt::Debug for Link<S>
where
    S: AsRef<str>,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = self.name.as_ref();
        let url = self.url.as_ref();
        write!(fmt, "[{name}]({url})")
    }
}
impl<S> From<(S, S)> for Link<S> {
    fn from((name, url): (S, S)) -> Self {
        Self { name, url }
    }
}
