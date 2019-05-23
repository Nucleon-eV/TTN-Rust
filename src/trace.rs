// This file is generated by rust-protobuf 2.6.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Trace {
    // message fields
    pub id: ::std::string::String,
    pub time: i64,
    pub service_id: ::std::string::String,
    pub service_name: ::std::string::String,
    pub event: ::std::string::String,
    pub metadata: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub parents: ::protobuf::RepeatedField<Trace>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Trace {
    fn default() -> &'a Trace {
        <Trace as ::protobuf::Message>::default_instance()
    }
}

impl Trace {
    pub fn new() -> Trace {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // int64 time = 2;


    pub fn get_time(&self) -> i64 {
        self.time
    }
    pub fn clear_time(&mut self) {
        self.time = 0;
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: i64) {
        self.time = v;
    }

    // string service_id = 3;


    pub fn get_service_id(&self) -> &str {
        &self.service_id
    }
    pub fn clear_service_id(&mut self) {
        self.service_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_service_id(&mut self, v: ::std::string::String) {
        self.service_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service_id(&mut self) -> &mut ::std::string::String {
        &mut self.service_id
    }

    // Take field
    pub fn take_service_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.service_id, ::std::string::String::new())
    }

    // string service_name = 4;


    pub fn get_service_name(&self) -> &str {
        &self.service_name
    }
    pub fn clear_service_name(&mut self) {
        self.service_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_service_name(&mut self, v: ::std::string::String) {
        self.service_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service_name(&mut self) -> &mut ::std::string::String {
        &mut self.service_name
    }

    // Take field
    pub fn take_service_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.service_name, ::std::string::String::new())
    }

    // string event = 5;


    pub fn get_event(&self) -> &str {
        &self.event
    }
    pub fn clear_event(&mut self) {
        self.event.clear();
    }

    // Param is passed by value, moved
    pub fn set_event(&mut self, v: ::std::string::String) {
        self.event = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event(&mut self) -> &mut ::std::string::String {
        &mut self.event
    }

    // Take field
    pub fn take_event(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.event, ::std::string::String::new())
    }

    // repeated .trace.Trace.MetadataEntry metadata = 6;


    pub fn get_metadata(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.metadata
    }
    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.metadata = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.metadata
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.metadata, ::std::collections::HashMap::new())
    }

    // repeated .trace.Trace parents = 11;


    pub fn get_parents(&self) -> &[Trace] {
        &self.parents
    }
    pub fn clear_parents(&mut self) {
        self.parents.clear();
    }

    // Param is passed by value, moved
    pub fn set_parents(&mut self, v: ::protobuf::RepeatedField<Trace>) {
        self.parents = v;
    }

    // Mutable pointer to the field.
    pub fn mut_parents(&mut self) -> &mut ::protobuf::RepeatedField<Trace> {
        &mut self.parents
    }

    // Take field
    pub fn take_parents(&mut self) -> ::protobuf::RepeatedField<Trace> {
        ::std::mem::replace(&mut self.parents, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Trace {
    fn is_initialized(&self) -> bool {
        for v in &self.parents {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.time = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.service_id)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.service_name)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.event)?;
                },
                6 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.metadata)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.parents)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if self.time != 0 {
            my_size += ::protobuf::rt::value_size(2, self.time, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.service_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.service_id);
        }
        if !self.service_name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.service_name);
        }
        if !self.event.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.event);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(6, &self.metadata);
        for value in &self.parents {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if self.time != 0 {
            os.write_int64(2, self.time)?;
        }
        if !self.service_id.is_empty() {
            os.write_string(3, &self.service_id)?;
        }
        if !self.service_name.is_empty() {
            os.write_string(4, &self.service_name)?;
        }
        if !self.event.is_empty() {
            os.write_string(5, &self.event)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(6, &self.metadata, os)?;
        for v in &self.parents {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Trace {
        Trace::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    |m: &Trace| { &m.id },
                    |m: &mut Trace| { &mut m.id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "time",
                    |m: &Trace| { &m.time },
                    |m: &mut Trace| { &mut m.time },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service_id",
                    |m: &Trace| { &m.service_id },
                    |m: &mut Trace| { &mut m.service_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service_name",
                    |m: &Trace| { &m.service_name },
                    |m: &mut Trace| { &mut m.service_name },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "event",
                    |m: &Trace| { &m.event },
                    |m: &mut Trace| { &mut m.event },
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                    "metadata",
                    |m: &Trace| { &m.metadata },
                    |m: &mut Trace| { &mut m.metadata },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Trace>>(
                    "parents",
                    |m: &Trace| { &m.parents },
                    |m: &mut Trace| { &mut m.parents },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Trace>(
                    "Trace",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Trace {
        static mut instance: ::protobuf::lazy::Lazy<Trace> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Trace,
        };
        unsafe {
            instance.get(Trace::new)
        }
    }
}

impl ::protobuf::Clear for Trace {
    fn clear(&mut self) {
        self.id.clear();
        self.time = 0;
        self.service_id.clear();
        self.service_name.clear();
        self.event.clear();
        self.metadata.clear();
        self.parents.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Trace {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Trace {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n1github.com/TheThingsNetwork/api/trace/trace.proto\x12\x05trace\x1a-gi\
    thub.com/gogo/protobuf/gogoproto/gogo.proto\"\xb7\x02\n\x05Trace\x12\x16\
    \n\x02id\x18\x01\x20\x01(\tR\x02idB\x06\xe2\xde\x1f\x02ID\x12\x12\n\x04t\
    ime\x18\x02\x20\x01(\x03R\x04time\x12,\n\nservice_id\x18\x03\x20\x01(\tR\
    \tserviceIdB\r\xe2\xde\x1f\tServiceID\x12!\n\x0cservice_name\x18\x04\x20\
    \x01(\tR\x0bserviceName\x12\x14\n\x05event\x18\x05\x20\x01(\tR\x05event\
    \x126\n\x08metadata\x18\x06\x20\x03(\x0b2\x1a.trace.Trace.MetadataEntryR\
    \x08metadata\x12&\n\x07parents\x18\x0b\x20\x03(\x0b2\x0c.trace.TraceR\
    \x07parents\x1a;\n\rMetadataEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\
    \x03key\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\x028\x01Br\n\
    \x1eorg.thethingsnetwork.api.traceB\nTraceProtoP\x01Z%github.com/TheThin\
    gsNetwork/api/trace\xaa\x02\x1aTheThingsNetwork.API.Traceb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}