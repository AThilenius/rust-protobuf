// This file is generated by rust-protobuf 3.0.0-pre. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `google/protobuf/duration.proto`

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(serde, derive(Serialize, Deserialize))]
pub struct Duration {
    // message fields
    ///  Signed seconds of the span of time. Must be from -315,576,000,000
    ///  to +315,576,000,000 inclusive.
    pub seconds: i64,
    ///  Signed fractions of a second at nanosecond resolution of the span
    ///  of time. Durations less than one second are represented with a 0
    ///  `seconds` field and a positive or negative `nanos` field. For durations
    ///  of one second or more, a non-zero value for the `nanos` field must be
    ///  of the same sign as the `seconds` field. Must be from -999,999,999
    ///  to +999,999,999 inclusive.
    pub nanos: i32,
    // special fields
    #[cfg_attr(serde, serde(skip))]
    pub unknown_fields: crate::UnknownFields,
    #[cfg_attr(serde, serde(skip))]
    pub cached_size: crate::rt::CachedSize,
}

impl<'a> ::std::default::Default for &'a Duration {
    fn default() -> &'a Duration {
        <Duration as crate::Message>::default_instance()
    }
}

impl Duration {
    pub fn new() -> Duration {
        ::std::default::Default::default()
    }
}

impl crate::Message for Duration {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut crate::CodedInputStream<'_>) -> crate::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != crate::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(crate::rt::unexpected_wire_type(wire_type));
                    }
                    self.seconds = is.read_int64()?;
                },
                2 => {
                    if wire_type != crate::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(crate::rt::unexpected_wire_type(wire_type));
                    }
                    self.nanos = is.read_int32()?;
                },
                _ => {
                    crate::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.seconds != 0 {
            my_size += crate::rt::value_size(1, self.seconds, crate::wire_format::WireTypeVarint);
        }
        if self.nanos != 0 {
            my_size += crate::rt::value_size(2, self.nanos, crate::wire_format::WireTypeVarint);
        }
        my_size += crate::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut crate::CodedOutputStream<'_>) -> crate::ProtobufResult<()> {
        if self.seconds != 0 {
            os.write_int64(1, self.seconds)?;
        }
        if self.nanos != 0 {
            os.write_int32(2, self.nanos)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &crate::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut crate::UnknownFields {
        &mut self.unknown_fields
    }

    fn descriptor(&self) -> &'static crate::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Duration {
        Duration::new()
    }

    fn descriptor_static() -> &'static crate::reflect::MessageDescriptor {
        static descriptor: crate::rt::LazyV2<crate::reflect::MessageDescriptor> = crate::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(crate::reflect::rt::v2::make_simple_field_accessor::<_, crate::reflect::types::ProtobufTypeInt64>(
                "seconds",
                |m: &Duration| { &m.seconds },
                |m: &mut Duration| { &mut m.seconds },
            ));
            fields.push(crate::reflect::rt::v2::make_simple_field_accessor::<_, crate::reflect::types::ProtobufTypeInt32>(
                "nanos",
                |m: &Duration| { &m.nanos },
                |m: &mut Duration| { &mut m.nanos },
            ));
            crate::reflect::MessageDescriptor::new::<Duration>(
                "Duration",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Duration {
        static instance: Duration = Duration {
            seconds: 0,
            nanos: 0,
            unknown_fields: crate::UnknownFields::new(),
            cached_size: crate::rt::CachedSize::new(),
        };
        &instance
    }
}

impl crate::Clear for Duration {
    fn clear(&mut self) {
        self.seconds = 0;
        self.nanos = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Duration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        crate::text_format::fmt(self, f)
    }
}

impl crate::reflect::ProtobufValue for Duration {
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1egoogle/protobuf/duration.proto\x12\x0fgoogle.protobuf\":\n\x08Dura\
    tion\x12\x18\n\x07seconds\x18\x01\x20\x01(\x03R\x07seconds\x12\x14\n\x05\
    nanos\x18\x02\x20\x01(\x05R\x05nanosB|\n\x13com.google.protobufB\rDurati\
    onProtoP\x01Z*github.com/golang/protobuf/ptypes/duration\xf8\x01\x01\xa2\
    \x02\x03GPB\xaa\x02\x1eGoogle.Protobuf.WellKnownTypesJ\xe1\x1e\n\x06\x12\
    \x04\x1e\0g\x01\n\xcc\x0c\n\x01\x0c\x12\x03\x1e\0\x122\xc1\x0c\x20Protoc\
    ol\x20Buffers\x20-\x20Google's\x20data\x20interchange\x20format\n\x20Cop\
    yright\x202008\x20Google\x20Inc.\x20\x20All\x20rights\x20reserved.\n\x20\
    https://developers.google.com/protocol-buffers/\n\n\x20Redistribution\
    \x20and\x20use\x20in\x20source\x20and\x20binary\x20forms,\x20with\x20or\
    \x20without\n\x20modification,\x20are\x20permitted\x20provided\x20that\
    \x20the\x20following\x20conditions\x20are\n\x20met:\n\n\x20\x20\x20\x20\
    \x20*\x20Redistributions\x20of\x20source\x20code\x20must\x20retain\x20th\
    e\x20above\x20copyright\n\x20notice,\x20this\x20list\x20of\x20conditions\
    \x20and\x20the\x20following\x20disclaimer.\n\x20\x20\x20\x20\x20*\x20Red\
    istributions\x20in\x20binary\x20form\x20must\x20reproduce\x20the\x20abov\
    e\n\x20copyright\x20notice,\x20this\x20list\x20of\x20conditions\x20and\
    \x20the\x20following\x20disclaimer\n\x20in\x20the\x20documentation\x20an\
    d/or\x20other\x20materials\x20provided\x20with\x20the\n\x20distribution.\
    \n\x20\x20\x20\x20\x20*\x20Neither\x20the\x20name\x20of\x20Google\x20Inc\
    .\x20nor\x20the\x20names\x20of\x20its\n\x20contributors\x20may\x20be\x20\
    used\x20to\x20endorse\x20or\x20promote\x20products\x20derived\x20from\n\
    \x20this\x20software\x20without\x20specific\x20prior\x20written\x20permi\
    ssion.\n\n\x20THIS\x20SOFTWARE\x20IS\x20PROVIDED\x20BY\x20THE\x20COPYRIG\
    HT\x20HOLDERS\x20AND\x20CONTRIBUTORS\n\x20\"AS\x20IS\"\x20AND\x20ANY\x20\
    EXPRESS\x20OR\x20IMPLIED\x20WARRANTIES,\x20INCLUDING,\x20BUT\x20NOT\n\
    \x20LIMITED\x20TO,\x20THE\x20IMPLIED\x20WARRANTIES\x20OF\x20MERCHANTABIL\
    ITY\x20AND\x20FITNESS\x20FOR\n\x20A\x20PARTICULAR\x20PURPOSE\x20ARE\x20D\
    ISCLAIMED.\x20IN\x20NO\x20EVENT\x20SHALL\x20THE\x20COPYRIGHT\n\x20OWNER\
    \x20OR\x20CONTRIBUTORS\x20BE\x20LIABLE\x20FOR\x20ANY\x20DIRECT,\x20INDIR\
    ECT,\x20INCIDENTAL,\n\x20SPECIAL,\x20EXEMPLARY,\x20OR\x20CONSEQUENTIAL\
    \x20DAMAGES\x20(INCLUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\x20PROCUREM\
    ENT\x20OF\x20SUBSTITUTE\x20GOODS\x20OR\x20SERVICES;\x20LOSS\x20OF\x20USE\
    ,\n\x20DATA,\x20OR\x20PROFITS;\x20OR\x20BUSINESS\x20INTERRUPTION)\x20HOW\
    EVER\x20CAUSED\x20AND\x20ON\x20ANY\n\x20THEORY\x20OF\x20LIABILITY,\x20WH\
    ETHER\x20IN\x20CONTRACT,\x20STRICT\x20LIABILITY,\x20OR\x20TORT\n\x20(INC\
    LUDING\x20NEGLIGENCE\x20OR\x20OTHERWISE)\x20ARISING\x20IN\x20ANY\x20WAY\
    \x20OUT\x20OF\x20THE\x20USE\n\x20OF\x20THIS\x20SOFTWARE,\x20EVEN\x20IF\
    \x20ADVISED\x20OF\x20THE\x20POSSIBILITY\x20OF\x20SUCH\x20DAMAGE.\n\n\x08\
    \n\x01\x02\x12\x03\x20\0\x18\n\x08\n\x01\x08\x12\x03\"\0;\n\t\n\x02\x08%\
    \x12\x03\"\0;\n\x08\n\x01\x08\x12\x03#\0\x1f\n\t\n\x02\x08\x1f\x12\x03#\
    \0\x1f\n\x08\n\x01\x08\x12\x03$\0A\n\t\n\x02\x08\x0b\x12\x03$\0A\n\x08\n\
    \x01\x08\x12\x03%\0,\n\t\n\x02\x08\x01\x12\x03%\0,\n\x08\n\x01\x08\x12\
    \x03&\0.\n\t\n\x02\x08\x08\x12\x03&\0.\n\x08\n\x01\x08\x12\x03'\0\"\n\t\
    \n\x02\x08\n\x12\x03'\0\"\n\x08\n\x01\x08\x12\x03(\0!\n\t\n\x02\x08$\x12\
    \x03(\0!\n\x92\x0c\n\x02\x04\0\x12\x04Z\0g\x01\x1a\x85\x0c\x20A\x20Durat\
    ion\x20represents\x20a\x20signed,\x20fixed-length\x20span\x20of\x20time\
    \x20represented\n\x20as\x20a\x20count\x20of\x20seconds\x20and\x20fractio\
    ns\x20of\x20seconds\x20at\x20nanosecond\n\x20resolution.\x20It\x20is\x20\
    independent\x20of\x20any\x20calendar\x20and\x20concepts\x20like\x20\"day\
    \"\n\x20or\x20\"month\".\x20It\x20is\x20related\x20to\x20Timestamp\x20in\
    \x20that\x20the\x20difference\x20between\n\x20two\x20Timestamp\x20values\
    \x20is\x20a\x20Duration\x20and\x20it\x20can\x20be\x20added\x20or\x20subt\
    racted\n\x20from\x20a\x20Timestamp.\x20Range\x20is\x20approximately\x20+\
    -10,000\x20years.\n\n\x20Example\x201:\x20Compute\x20Duration\x20from\
    \x20two\x20Timestamps\x20in\x20pseudo\x20code.\n\n\x20\x20\x20\x20\x20Ti\
    mestamp\x20start\x20=\x20...;\n\x20\x20\x20\x20\x20Timestamp\x20end\x20=\
    \x20...;\n\x20\x20\x20\x20\x20Duration\x20duration\x20=\x20...;\n\n\x20\
    \x20\x20\x20\x20duration.seconds\x20=\x20end.seconds\x20-\x20start.secon\
    ds;\n\x20\x20\x20\x20\x20duration.nanos\x20=\x20end.nanos\x20-\x20start.\
    nanos;\n\n\x20\x20\x20\x20\x20if\x20(duration.seconds\x20<\x200\x20&&\
    \x20duration.nanos\x20>\x200)\x20{\n\x20\x20\x20\x20\x20\x20\x20duration\
    .seconds\x20+=\x201;\n\x20\x20\x20\x20\x20\x20\x20duration.nanos\x20-=\
    \x201000000000;\n\x20\x20\x20\x20\x20}\x20else\x20if\x20(durations.secon\
    ds\x20>\x200\x20&&\x20duration.nanos\x20<\x200)\x20{\n\x20\x20\x20\x20\
    \x20\x20\x20duration.seconds\x20-=\x201;\n\x20\x20\x20\x20\x20\x20\x20du\
    ration.nanos\x20+=\x201000000000;\n\x20\x20\x20\x20\x20}\n\n\x20Example\
    \x202:\x20Compute\x20Timestamp\x20from\x20Timestamp\x20+\x20Duration\x20\
    in\x20pseudo\x20code.\n\n\x20\x20\x20\x20\x20Timestamp\x20start\x20=\x20\
    ...;\n\x20\x20\x20\x20\x20Duration\x20duration\x20=\x20...;\n\x20\x20\
    \x20\x20\x20Timestamp\x20end\x20=\x20...;\n\n\x20\x20\x20\x20\x20end.sec\
    onds\x20=\x20start.seconds\x20+\x20duration.seconds;\n\x20\x20\x20\x20\
    \x20end.nanos\x20=\x20start.nanos\x20+\x20duration.nanos;\n\n\x20\x20\
    \x20\x20\x20if\x20(end.nanos\x20<\x200)\x20{\n\x20\x20\x20\x20\x20\x20\
    \x20end.seconds\x20-=\x201;\n\x20\x20\x20\x20\x20\x20\x20end.nanos\x20+=\
    \x201000000000;\n\x20\x20\x20\x20\x20}\x20else\x20if\x20(end.nanos\x20>=\
    \x201000000000)\x20{\n\x20\x20\x20\x20\x20\x20\x20end.seconds\x20+=\x201\
    ;\n\x20\x20\x20\x20\x20\x20\x20end.nanos\x20-=\x201000000000;\n\x20\x20\
    \x20\x20\x20}\n\n\x20Example\x203:\x20Compute\x20Duration\x20from\x20dat\
    etime.timedelta\x20in\x20Python.\n\n\x20\x20\x20\x20\x20td\x20=\x20datet\
    ime.timedelta(days=3,\x20minutes=10)\n\x20\x20\x20\x20\x20duration\x20=\
    \x20Duration()\n\x20\x20\x20\x20\x20duration.FromTimedelta(td)\n\n\n\n\n\
    \n\x03\x04\0\x01\x12\x03Z\x08\x10\np\n\x04\x04\0\x02\0\x12\x03^\x02\x14\
    \x1ac\x20Signed\x20seconds\x20of\x20the\x20span\x20of\x20time.\x20Must\
    \x20be\x20from\x20-315,576,000,000\n\x20to\x20+315,576,000,000\x20inclus\
    ive.\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03^\x02\x07\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03^\x08\x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03^\x12\x13\
    \n\x83\x03\n\x04\x04\0\x02\x01\x12\x03f\x02\x12\x1a\xf5\x02\x20Signed\
    \x20fractions\x20of\x20a\x20second\x20at\x20nanosecond\x20resolution\x20\
    of\x20the\x20span\n\x20of\x20time.\x20Durations\x20less\x20than\x20one\
    \x20second\x20are\x20represented\x20with\x20a\x200\n\x20`seconds`\x20fie\
    ld\x20and\x20a\x20positive\x20or\x20negative\x20`nanos`\x20field.\x20For\
    \x20durations\n\x20of\x20one\x20second\x20or\x20more,\x20a\x20non-zero\
    \x20value\x20for\x20the\x20`nanos`\x20field\x20must\x20be\n\x20of\x20the\
    \x20same\x20sign\x20as\x20the\x20`seconds`\x20field.\x20Must\x20be\x20fr\
    om\x20-999,999,999\n\x20to\x20+999,999,999\x20inclusive.\n\n\x0c\n\x05\
    \x04\0\x02\x01\x05\x12\x03f\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03f\x08\r\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03f\x10\x11b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
pub fn file_descriptor_proto() -> &'static crate::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: crate::rt::LazyV2<crate::descriptor::FileDescriptorProto> = crate::rt::LazyV2::INIT;
    file_descriptor_proto_lazy.get(|| {
        crate::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static crate::reflect::FileDescriptor {
    static file_descriptor_lazy: crate::rt::LazyV2<crate::reflect::FileDescriptor> = crate::rt::LazyV2::INIT;
    file_descriptor_lazy.get(|| {
        let mut deps = ::std::vec::Vec::new();
        let mut messages = ::std::vec::Vec::new();
        messages.push(<Duration as crate::Message>::descriptor_static());
        let mut enums = ::std::vec::Vec::new();
        crate::reflect::FileDescriptor::new(
            file_descriptor_proto(),
            deps,
            messages,
            enums,
        )
    })
}
