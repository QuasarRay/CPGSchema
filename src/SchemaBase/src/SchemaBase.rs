pub trait SchemaBase {
    /// Documentation ordering key. Smaller values appear first.
    fn doc_index(&self) -> i32;

    /// Human-readable description of this schema module.
    fn description(&self) -> &str;

    /// Whether this module is supplied by a language frontend.
    ///
    /// The language-neutral default is `false`.
    fn provided_by_frontend(&self) -> bool {
        false
    }

    /// Borrow the three common fields as a binding-neutral Rust view.
    fn schema_base(&self) -> SchemaBaseRef<'_> {
        SchemaBaseRef {
            doc_index: self.doc_index(),
            description: self.description(),
            provided_by_frontend: self.provided_by_frontend(),
        }
    }
}

/// Borrowed Rust value representation of the common contract.
///
/// This is useful at Rust API boundaries without requiring an implementation
/// type to expose its internal representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchemaBaseRef<'a> {
    pub doc_index: i32,
    pub description: &'a str,
    pub provided_by_frontend: bool,
}

impl SchemaBase for SchemaBaseRef<'_> {
    #[inline]
    fn doc_index(&self) -> i32 {
        self.doc_index
    }

    #[inline]
    fn description(&self) -> &str {
        self.description
    }

    #[inline]
    fn provided_by_frontend(&self) -> bool {
        self.provided_by_frontend
    }
}

/// Convenience implementation for references to SchemaBase implementations.
impl<T: SchemaBase + ?Sized> SchemaBase for &T {
    #[inline]
    fn doc_index(&self) -> i32 {
        (**self).doc_index()
    }

    #[inline]
    fn description(&self) -> &str {
        (**self).description()
    }

    #[inline]
    fn provided_by_frontend(&self) -> bool {
        (**self).provided_by_frontend()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Example;

    impl SchemaBase for Example {
        fn doc_index(&self) -> i32 {
            7
        }

        fn description(&self) -> &str {
            "example"
        }
    }

    #[test]
    fn semantic_default_is_false() {
        let value = Example;
        assert!(!value.provided_by_frontend());
    }

    #[test]
    fn borrowed_view_preserves_contract() {
        let value = Example;
        assert_eq!(
            value.schema_base(),
            SchemaBaseRef {
                doc_index: 7,
                description: "example",
                provided_by_frontend: false,
            }
        );
    }
}
