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

pub trait Broker {
    fn associate(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::broker::UplinkMessage>) -> ::grpc::StreamingResponse<super::broker::DownlinkMessage>;

    fn subscribe(&self, o: ::grpc::RequestOptions, p: super::broker::SubscribeRequest) -> ::grpc::StreamingResponse<super::broker::DeduplicatedUplinkMessage>;

    fn publish(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::broker::DownlinkMessage>) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn activate(&self, o: ::grpc::RequestOptions, p: super::broker::DeviceActivationRequest) -> ::grpc::SingleResponse<super::broker::DeviceActivationResponse>;
}

// client

pub struct BrokerClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Associate: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::broker::UplinkMessage, super::broker::DownlinkMessage>>,
    method_Subscribe: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::broker::SubscribeRequest, super::broker::DeduplicatedUplinkMessage>>,
    method_Publish: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::broker::DownlinkMessage, super::empty::Empty>>,
    method_Activate: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::broker::DeviceActivationRequest, super::broker::DeviceActivationResponse>>,
}

impl ::grpc::ClientStub for BrokerClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        BrokerClient {
            grpc_client: grpc_client,
            method_Associate: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/broker.Broker/Associate".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Subscribe: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/broker.Broker/Subscribe".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Publish: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/broker.Broker/Publish".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ClientStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Activate: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/broker.Broker/Activate".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Broker for BrokerClient {
    fn associate(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::broker::UplinkMessage>) -> ::grpc::StreamingResponse<super::broker::DownlinkMessage> {
        self.grpc_client.call_bidi(o, p, self.method_Associate.clone())
    }

    fn subscribe(&self, o: ::grpc::RequestOptions, p: super::broker::SubscribeRequest) -> ::grpc::StreamingResponse<super::broker::DeduplicatedUplinkMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_Subscribe.clone())
    }

    fn publish(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::broker::DownlinkMessage>) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_client_streaming(o, p, self.method_Publish.clone())
    }

    fn activate(&self, o: ::grpc::RequestOptions, p: super::broker::DeviceActivationRequest) -> ::grpc::SingleResponse<super::broker::DeviceActivationResponse> {
        self.grpc_client.call_unary(o, p, self.method_Activate.clone())
    }
}

// server

pub struct BrokerServer;


impl BrokerServer {
    pub fn new_service_def<H : Broker + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/broker.Broker",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/broker.Broker/Associate".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerBidi::new(move |o, p| handler_copy.associate(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/broker.Broker/Subscribe".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.subscribe(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/broker.Broker/Publish".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ClientStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerClientStreaming::new(move |o, p| handler_copy.publish(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/broker.Broker/Activate".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.activate(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait BrokerManager {
    fn register_application_handler(&self, o: ::grpc::RequestOptions, p: super::broker::ApplicationHandlerRegistration) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn get_status(&self, o: ::grpc::RequestOptions, p: super::broker::StatusRequest) -> ::grpc::SingleResponse<super::broker::Status>;
}

// client

pub struct BrokerManagerClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_RegisterApplicationHandler: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::broker::ApplicationHandlerRegistration, super::empty::Empty>>,
    method_GetStatus: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::broker::StatusRequest, super::broker::Status>>,
}

impl ::grpc::ClientStub for BrokerManagerClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        BrokerManagerClient {
            grpc_client: grpc_client,
            method_RegisterApplicationHandler: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/broker.BrokerManager/RegisterApplicationHandler".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetStatus: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/broker.BrokerManager/GetStatus".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl BrokerManager for BrokerManagerClient {
    fn register_application_handler(&self, o: ::grpc::RequestOptions, p: super::broker::ApplicationHandlerRegistration) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_RegisterApplicationHandler.clone())
    }

    fn get_status(&self, o: ::grpc::RequestOptions, p: super::broker::StatusRequest) -> ::grpc::SingleResponse<super::broker::Status> {
        self.grpc_client.call_unary(o, p, self.method_GetStatus.clone())
    }
}

// server

pub struct BrokerManagerServer;


impl BrokerManagerServer {
    pub fn new_service_def<H : BrokerManager + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/broker.BrokerManager",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/broker.BrokerManager/RegisterApplicationHandler".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.register_application_handler(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/broker.BrokerManager/GetStatus".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_status(o, p))
                    },
                ),
            ],
        )
    }
}
