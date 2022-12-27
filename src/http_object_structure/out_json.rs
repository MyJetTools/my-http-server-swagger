use macros_utils::ParamValue;
use types_reader::StructProperty;

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
        if let Some(attr) = self.property.attrs.get("serde") {
            if let Some(attr) = attr {
                if let Some(value) = attr.get_named_param("rename") {
                    return value;
                }
            }
        }

        ParamValue {
            value: self.property.name.as_bytes(),
        }
    }
}
