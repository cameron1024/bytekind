use schemars::JsonSchema;

use crate::{Format, Bytes};

impl<F: Format> JsonSchema for Bytes<F> {
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Vec::<u8>::json_schema(gen)
    }

    fn schema_name() -> String {
        Vec::<u8>::schema_name()
    }
}
