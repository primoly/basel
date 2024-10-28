use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::marker::PhantomData;
use time::macros::format_description;
use time::Date;

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct GeoPoint2d {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct File {
    pub url: String,
    pub width: u16,
    pub height: u16,
}

pub(crate) fn deserialize_date<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let format = format_description!("[year]-[month]-[day]");
    if let Ok(date) = time::Date::parse(&s, format) {
        Ok(Some(date))
    } else {
        Ok(None)
    }
}

pub(crate) fn serialize_date<S: Serializer>(
    date: &Option<Date>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(date) = date {
        serializer.serialize_some(&format!(
            "{:04}-{:02}-{:02}",
            date.year(),
            date.month(),
            date.day()
        ))
    } else {
        serializer.serialize_none()
    }
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Data<T> {
    pub total_count: u64,
    pub results: Vec<T>,
}

pub(crate) trait Field {
    fn name(self) -> &'static str;
}

#[derive(Debug, Clone, Default)]
pub struct Order<T: Field> {
    pub(crate) inner: String,
    phantom: PhantomData<T>,
}

impl<T: Field> Order<T> {
    pub fn new() -> Self {
        Order {
            inner: String::new(),
            phantom: PhantomData,
        }
    }

    pub fn ascending(mut self, field: T) -> Self {
        if !self.inner.is_empty() {
            self.inner.push_str(", ");
        }
        self.inner.push_str(&format!("`{}`", field.name()));
        self
    }

    pub fn descending(mut self, field: T) -> Self {
        if !self.inner.is_empty() {
            self.inner.push_str(", ");
        }
        self.inner.push_str(&format!("`{}` desc", field.name()));
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct Filter<T: Field> {
    pub(crate) inner: String,
    phantom: PhantomData<T>,
}

impl<T: Field> Filter<T> {
    pub fn is_null(field: T) -> Self {
        Filter {
            inner: format!("`{}` is null", field.name()),
            phantom: PhantomData,
        }
    }

    pub fn is_not_null(field: T) -> Self {
        Filter {
            inner: format!("`{}` is not null", field.name()),
            phantom: PhantomData,
        }
    }

    pub fn equal_str(field: T, value: &str) -> Self {
        Filter {
            inner: format!("`{}` = \"{}\"", field.name(), escape(value)),
            phantom: PhantomData,
        }
    }

    pub fn equal_num(field: T, value: f64) -> Self {
        Filter {
            inner: format!("`{}` = {value}", field.name()),
            phantom: PhantomData,
        }
    }

    pub fn not_equal_str(field: T, value: &str) -> Self {
        Filter {
            inner: format!("`{}` != \"{}\"", field.name(), escape(value)),
            phantom: PhantomData,
        }
    }

    pub fn not_equal_num(field: T, value: f64) -> Self {
        Filter {
            inner: format!("`{}` != {value}", field.name()),
            phantom: PhantomData,
        }
    }

    pub fn greater(field: T, value: f64) -> Self {
        Filter {
            inner: format!("{} > {value}", field.name()),
            phantom: PhantomData,
        }
    }

    pub fn greater_or_equal(field: T, value: f64) -> Self {
        Filter {
            inner: format!("{} >= {value}", field.name()),
            phantom: PhantomData,
        }
    }

    pub fn less(field: T, value: f64) -> Self {
        Filter {
            inner: format!("{} < {value}", field.name()),
            phantom: PhantomData,
        }
    }

    pub fn less_or_equal(field: T, value: f64) -> Self {
        Filter {
            inner: format!("{} <= {value}", field.name()),
            phantom: PhantomData,
        }
    }

    pub fn starts_with(field: T, value: &str) -> Self {
        Filter {
            inner: format!("startswith(`{}`, \"{}\")", field.name(), escape(value)),
            phantom: PhantomData,
        }
    }

    pub fn search(field: T, value: &str) -> Self {
        Filter {
            inner: format!("search(`{}`, \"{}\")", field.name(), escape(value)),
            phantom: PhantomData,
        }
    }

    pub fn and(self, other: Self) -> Self {
        Filter {
            inner: format!("({}) and ({})", self.inner, other.inner),
            phantom: PhantomData,
        }
    }

    pub fn or(self, other: Self) -> Self {
        Filter {
            inner: format!("({}) or ({})", self.inner, other.inner),
            phantom: PhantomData,
        }
    }

    pub fn not(self) -> Self {
        Filter {
            inner: format!("not ({})", self.inner),
            phantom: PhantomData,
        }
    }
}
