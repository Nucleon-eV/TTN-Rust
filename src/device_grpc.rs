// This file is generated. Do not edit
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


// interface

pub trait DeviceManager {
    fn get_device(&self, o: ::grpc::RequestOptions, p: super::device::DeviceIdentifier) -> ::grpc::SingleResponse<super::device::Device>;

    fn set_device(&self, o: ::grpc::RequestOptions, p: super::device::Device) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn delete_device(&self, o: ::grpc::RequestOptions, p: super::device::DeviceIdentifier) -> ::grpc::SingleResponse<super::empty::Empty>;
}

// client

pub struct DeviceManagerClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_GetDevice: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::device::DeviceIdentifier, super::device::Device>>,
    method_SetDevice: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::device::Device, super::empty::Empty>>,
    method_DeleteDevice: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::device::DeviceIdentifier, super::empty::Empty>>,
}

impl ::grpc::ClientStub for DeviceManagerClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        DeviceManagerClient {
            grpc_client: grpc_client,
            method_GetDevice: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lorawan.DeviceManager/GetDevice".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SetDevice: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lorawan.DeviceManager/SetDevice".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteDevice: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lorawan.DeviceManager/DeleteDevice".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl DeviceManager for DeviceManagerClient {
    fn get_device(&self, o: ::grpc::RequestOptions, p: super::device::DeviceIdentifier) -> ::grpc::SingleResponse<super::device::Device> {
        self.grpc_client.call_unary(o, p, self.method_GetDevice.clone())
    }

    fn set_device(&self, o: ::grpc::RequestOptions, p: super::device::Device) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_SetDevice.clone())
    }

    fn delete_device(&self, o: ::grpc::RequestOptions, p: super::device::DeviceIdentifier) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_DeleteDevice.clone())
    }
}

// server

pub struct DeviceManagerServer;


impl DeviceManagerServer {
    pub fn new_service_def<H : DeviceManager + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/lorawan.DeviceManager",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lorawan.DeviceManager/GetDevice".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_device(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lorawan.DeviceManager/SetDevice".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.set_device(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lorawan.DeviceManager/DeleteDevice".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_device(o, p))
                    },
                ),
            ],
        )
    }
}
