use types_reader::{attribute_params::ParamValue, StructProperty};

pub struct OutputJson<'s> {
    pub fields: Vec<JsonField<'s>>,
}

impl<'s> OutputJson<'s> {
    pub fn new(properties: Vec<StructProperty<'s>>) -> Self {
        let mut fields = Vec::new();

        for property in properties {
            fields.push(JsonField::new(property))
        }

        Self { fields }
    }
}

pub struct JsonField<'s> {
    pub property: StructProperty<'s>,
}

impl<'s> JsonField<'s> {
    pub fn new(property: StructProperty<'s>) -> Self {
        Self { property }
    }

    pub fn name(&self) -> ParamValue {
        if let Ok(value) = self.property.attrs.get_named_param("serde", "rename") {
            return value;
        }

        ParamValue {
            value: self.property.name.as_bytes(),
        }
    }
}
