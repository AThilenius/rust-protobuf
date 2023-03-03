use protobuf::rt::WireType;

pub fn make_tag(field_number: u32, wire_type: WireType) -> u32 {
    (field_number << 3) | (wire_type as u32)
}
