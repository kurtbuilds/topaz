use multimap::MultiMap;


#[derive(Debug, Clone)]
pub struct Query(MultiMap<String, String>);

impl Query{
    pub fn from_string(s: &str) -> Self {
        let mut query = MultiMap::new();
        if s.len() > 0 {
            for pair in s[1..].split('&').filter(|s| !s.is_empty()) {
                let mut pair = pair.splitn(2, '=');
                let key = pair.next().unwrap();
                let value = pair.next().unwrap_or("");
                query.insert(key.to_string(), value.to_string());
            }
        }
        Self(query)
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        if self.0.is_empty() {
            return s;
        }
        s.push('?');
        let mut first = true;
        for (key, value) in self.0.iter() {
            if first {
                first = false;
            } else {
                s.push('&');
            }
            s.push_str(key);
            s.push('=');
            s.push_str(value);
        }
        s
    }

}

impl std::fmt::Display for Query{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::ops::Index<String> for Query {
    type Output = String;

    fn index(&self, key: String) -> &Self::Output {
        &self.0[&key]
    }
}

pub fn location() -> Location {
    Location::global()
}

#[derive(Clone)]
pub struct LocationData {
    /// Writeable field. Setting a new value will navigate to a new page, even on a separate domain.
    /// Javascript: `window.location.href`.
    pub href: String,

    /// Note that setting this does nothing. Use global::history::push_state to change the URL on the same domain.
    /// Javascript: `window.location.pathname`.
    pub pathname: String,

    /// Alias for: `pathname`
    /// Javascript: `window.location.pathname`.
    pub path: String,

    /// Javascript: `window.location.protocol`.
    pub protocol: String,

    /// Alias for: `protocol`
    pub scheme: String,

    /// Javascript: `window.location.search`.
    pub search: String,

    /// Writeable field. Setting a new value will set a new hash.
    /// Staying consistent with Javascript, this has a leading `#` if non-empty. Use `anchor` to get the value without the `#`.
    /// Javascript: `window.location.hash`.
    pub hash: String,

    /// Writeable field. Setting a new value will set a new hash.
    /// Prefer this to `hash`. Similar to `hash`, but without the leading `#`.
    pub anchor: String,

    pub query: Query,

}

impl LocationData {
    /// Instantiate document data using values from web_sys.
    pub(crate) fn from_web(location: &web_sys::Location) -> Self {
        let protocol = location.protocol().unwrap();
        let path = location.pathname().unwrap();
        let search = location.search().unwrap();
        let hash = location.hash().unwrap();
        let anchor = {
            let mut hash = hash.clone();
            if hash.len() > 0 {
                hash.remove(0);
            }
            hash
        };


        Self {
            href: location.href().unwrap(),
            scheme: protocol.clone(),
            pathname: path.clone(),
            query: Query::from_string(&search),
            anchor,
            path,
            protocol,
            search,
            hash,
        }
    }
}

pub struct Location {
    original: LocationData,
    modifiable: LocationData,

    inner: web_sys::Location,
}

impl Location {
    pub(crate) fn global() -> Self {
        let inner = web_sys::window().unwrap().location();
        let original = LocationData::from_web(&inner);
        Location {
            modifiable: original.clone(),
            original,
            inner,
        }
    }
}


impl std::ops::Deref for Location {
    type Target = LocationData;

    fn deref(&self) -> &Self::Target {
        &self.original
    }
}

impl std::ops::DerefMut for Location {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.modifiable
    }
}

impl Drop for Location {
    fn drop(&mut self) {
        if self.modifiable.href != self.original.href {
            println!("Href changed from {} to {}", self.original.href, self.modifiable.href);
            self.inner.set_href(&self.modifiable.href).unwrap();
        }
        if self.modifiable.hash != self.original.hash {
            println!("Hash changed from {} to {}", self.original.hash, self.modifiable.hash);
            self.inner.set_hash(&self.modifiable.hash).unwrap();
        }
        if self.modifiable.anchor != self.original.anchor {
            println!("Hash changed from {} to {}", self.original.anchor, self.modifiable.anchor);
            self.inner.set_hash(&self.modifiable.anchor).unwrap();
        }
    }
}