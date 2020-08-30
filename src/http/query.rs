use std::collections::HashMap;

pub struct Query<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> Query<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for Query<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub in s.split('&') {
            let mut key = sub;
            let mut val = "";

            if let Some(i) = sub.find('=') {
                key = &sub[..i];
                val = &sub[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                    Value::Multiple(vec) => vec.push(val)
                })
                .or_insert(Value::Single(val));
        }


        Query { data }

        unimplemented!()
    }
}