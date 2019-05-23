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

pub mod exts {
    use protobuf::Message as Message_imported_for_functions;

    pub const http: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MethodOptions, ::protobuf::types::ProtobufTypeMessage<super::super::http::HttpRule>> = ::protobuf::ext::ExtFieldOptional { field_number: 72295728, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cgoogle/api/annotations.proto\x12\ngoogle.api\x1a\x15google/api/htt\
    p.proto\x1a\x20google/protobuf/descriptor.proto:K\n\x04http\x18\xb0\xca\
    \xbc\"\x20\x01(\x0b2\x14.google.api.HttpRule\x12\x1e.google.protobuf.Met\
    hodOptionsR\x04httpBn\n\x0ecom.google.apiB\x10AnnotationsProtoP\x01ZAgoo\
    gle.golang.org/genproto/googleapis/api/annotations;annotations\xa2\x02\
    \x04GAPIb\x06proto3\
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