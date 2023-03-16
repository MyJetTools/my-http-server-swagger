use types_reader::{attribute_params::ParamValue, StructProperty};

pub trait StructPropertyExt {
    fn get_name(&self) -> ParamValue;
}

impl<'s> StructPropertyExt for StructProperty<'s> {
    fn get_name(&self) -> ParamValue {
        if let Ok(value) = self.attrs.get_named_param("serde", "rename") {
            return value;
        }

        ParamValue {
            value: self.name.as_bytes(),
        }
    }
}
