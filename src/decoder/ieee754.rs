// IEEE Std. 754 floating-point numbers

use std::collections::HashMap;

use crate::{
    bits::{Bit, Bits},
    Fields, IdxRange,
};

pub struct F32Decoder;
impl<'a> Fields<'a> for F32Decoder {
    fn fields(&self, bits: &'a Bits) -> HashMap<&str, Vec<(&'a [Bit], IdxRange)>> {
        assert_eq!(bits.len(), 32);

        let mut f32_fields = HashMap::new();
        f32_fields.insert("sign", vec![(&bits[31..=31], IdxRange::new(0, 0))]);
        f32_fields.insert("exponent", vec![(&bits[23..=31], IdxRange::new(7, 0))]);
        f32_fields.insert("mantissa", vec![(&bits[0..=22], IdxRange::new(22, 0))]);

        let x = [&bits[0..=10], &bits[11..=22]].concat();

        f32_fields
    }
}
