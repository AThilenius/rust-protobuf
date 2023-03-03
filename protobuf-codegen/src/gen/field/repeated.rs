use crate::gen::field::elem::FieldElem;
use crate::gen::file_and_mod::FileAndMod;
use crate::gen::rust::snippets::EXPR_VEC_NEW;
use crate::gen::rust_types_values::RustType;

/// Repeated field can be `Vec<T>` or `RepeatedField<T>`.
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum RepeatedFieldKind {
    Vec,
}

impl RepeatedFieldKind {
    fn wrap_element(&self, element_type: RustType) -> RustType {
        let element_type = Box::new(element_type);
        match self {
            RepeatedFieldKind::Vec => RustType::Vec(element_type),
        }
    }

    fn default(&self) -> String {
        match self {
            RepeatedFieldKind::Vec => EXPR_VEC_NEW.to_owned(),
        }
    }
}

#[derive(Clone)]
pub struct RepeatedField<'a> {
    pub elem: FieldElem<'a>,
    pub packed: bool,
}

impl<'a> RepeatedField<'a> {
    pub fn kind(&self) -> RepeatedFieldKind {
        RepeatedFieldKind::Vec
    }

    pub fn rust_type(&self, reference: &FileAndMod) -> RustType {
        self.kind()
            .wrap_element(self.elem.rust_storage_elem_type(reference))
    }

    pub fn default(&self) -> String {
        self.kind().default()
    }
}
