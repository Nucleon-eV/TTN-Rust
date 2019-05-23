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

pub trait Handler {
    fn activation_challenge(&self, o: ::grpc::RequestOptions, p: super::broker::ActivationChallengeRequest) -> ::grpc::SingleResponse<super::broker::ActivationChallengeResponse>;

    fn activate(&self, o: ::grpc::RequestOptions, p: super::broker::DeduplicatedDeviceActivationRequest) -> ::grpc::SingleResponse<super::handler::DeviceActivationResponse>;
}

// client

pub struct HandlerClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_ActivationChallenge: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::broker::ActivationChallengeRequest, super::broker::ActivationChallengeResponse>>,
    method_Activate: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::broker::DeduplicatedDeviceActivationRequest, super::handler::DeviceActivationResponse>>,
}

impl ::grpc::ClientStub for HandlerClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        HandlerClient {
            grpc_client: grpc_client,
            method_ActivationChallenge: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/handler.Handler/ActivationChallenge".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Activate: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/handler.Handler/Activate".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Handler for HandlerClient {
    fn activation_challenge(&self, o: ::grpc::RequestOptions, p: super::broker::ActivationChallengeRequest) -> ::grpc::SingleResponse<super::broker::ActivationChallengeResponse> {
        self.grpc_client.call_unary(o, p, self.method_ActivationChallenge.clone())
    }

    fn activate(&self, o: ::grpc::RequestOptions, p: super::broker::DeduplicatedDeviceActivationRequest) -> ::grpc::SingleResponse<super::handler::DeviceActivationResponse> {
        self.grpc_client.call_unary(o, p, self.method_Activate.clone())
    }
}

// server

pub struct HandlerServer;


impl HandlerServer {
    pub fn new_service_def<H : Handler + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/handler.Handler",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/handler.Handler/ActivationChallenge".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.activation_challenge(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/handler.Handler/Activate".to_string(),
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

pub trait ApplicationManager {
    fn register_application(&self, o: ::grpc::RequestOptions, p: super::handler::ApplicationIdentifier) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn get_application(&self, o: ::grpc::RequestOptions, p: super::handler::ApplicationIdentifier) -> ::grpc::SingleResponse<super::handler::Application>;

    fn set_application(&self, o: ::grpc::RequestOptions, p: super::handler::Application) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn delete_application(&self, o: ::grpc::RequestOptions, p: super::handler::ApplicationIdentifier) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn get_device(&self, o: ::grpc::RequestOptions, p: super::handler::DeviceIdentifier) -> ::grpc::SingleResponse<super::handler::Device>;

    fn set_device(&self, o: ::grpc::RequestOptions, p: super::handler::Device) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn delete_device(&self, o: ::grpc::RequestOptions, p: super::handler::DeviceIdentifier) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn get_devices_for_application(&self, o: ::grpc::RequestOptions, p: super::handler::ApplicationIdentifier) -> ::grpc::SingleResponse<super::handler::DeviceList>;

    fn dry_downlink(&self, o: ::grpc::RequestOptions, p: super::handler::DryDownlinkMessage) -> ::grpc::SingleResponse<super::handler::DryDownlinkResult>;

    fn dry_uplink(&self, o: ::grpc::RequestOptions, p: super::handler::DryUplinkMessage) -> ::grpc::SingleResponse<super::handler::DryUplinkResult>;

    fn simulate_uplink(&self, o: ::grpc::RequestOptions, p: super::handler::SimulatedUplinkMessage) -> ::grpc::SingleResponse<super::empty::Empty>;
}

// client

pub struct ApplicationManagerClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_RegisterApplication: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::handler::ApplicationIdentifier, super::empty::Empty>>,
    method_GetApplication: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::handler::ApplicationIdentifier, super::handler::Application>>,
    method_SetApplication: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::handler::Application, super::empty::Empty>>,
    method_DeleteApplication: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::handler::ApplicationIdentifier, super::empty::Empty>>,
    method_GetDevice: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::handler::DeviceIdentifier, super::handler::Device>>,
    method_SetDevice: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::handler::Device, super::empty::Empty>>,
    method_DeleteDevice: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::handler::DeviceIdentifier, super::empty::Empty>>,
    method_GetDevicesForApplication: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::handler::ApplicationIdentifier, super::handler::DeviceList>>,
    method_DryDownlink: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::handler::DryDownlinkMessage, super::handler::DryDownlinkResult>>,
    method_DryUplink: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::handler::DryUplinkMessage, super::handler::DryUplinkResult>>,
    method_SimulateUplink: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::handler::SimulatedUplinkMessage, super::empty::Empty>>,
}

impl ::grpc::ClientStub for ApplicationManagerClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        ApplicationManagerClient {
            grpc_client: grpc_client,
            method_RegisterApplication: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/handler.ApplicationManager/RegisterApplication".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetApplication: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/handler.ApplicationManager/GetApplication".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SetApplication: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/handler.ApplicationManager/SetApplication".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteApplication: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/handler.ApplicationManager/DeleteApplication".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetDevice: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/handler.ApplicationManager/GetDevice".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SetDevice: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/handler.ApplicationManager/SetDevice".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteDevice: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/handler.ApplicationManager/DeleteDevice".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetDevicesForApplication: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/handler.ApplicationManager/GetDevicesForApplication".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DryDownlink: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/handler.ApplicationManager/DryDownlink".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DryUplink: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/handler.ApplicationManager/DryUplink".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SimulateUplink: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/handler.ApplicationManager/SimulateUplink".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl ApplicationManager for ApplicationManagerClient {
    fn register_application(&self, o: ::grpc::RequestOptions, p: super::handler::ApplicationIdentifier) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_RegisterApplication.clone())
    }

    fn get_application(&self, o: ::grpc::RequestOptions, p: super::handler::ApplicationIdentifier) -> ::grpc::SingleResponse<super::handler::Application> {
        self.grpc_client.call_unary(o, p, self.method_GetApplication.clone())
    }

    fn set_application(&self, o: ::grpc::RequestOptions, p: super::handler::Application) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_SetApplication.clone())
    }

    fn delete_application(&self, o: ::grpc::RequestOptions, p: super::handler::ApplicationIdentifier) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_DeleteApplication.clone())
    }

    fn get_device(&self, o: ::grpc::RequestOptions, p: super::handler::DeviceIdentifier) -> ::grpc::SingleResponse<super::handler::Device> {
        self.grpc_client.call_unary(o, p, self.method_GetDevice.clone())
    }

    fn set_device(&self, o: ::grpc::RequestOptions, p: super::handler::Device) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_SetDevice.clone())
    }

    fn delete_device(&self, o: ::grpc::RequestOptions, p: super::handler::DeviceIdentifier) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_DeleteDevice.clone())
    }

    fn get_devices_for_application(&self, o: ::grpc::RequestOptions, p: super::handler::ApplicationIdentifier) -> ::grpc::SingleResponse<super::handler::DeviceList> {
        self.grpc_client.call_unary(o, p, self.method_GetDevicesForApplication.clone())
    }

    fn dry_downlink(&self, o: ::grpc::RequestOptions, p: super::handler::DryDownlinkMessage) -> ::grpc::SingleResponse<super::handler::DryDownlinkResult> {
        self.grpc_client.call_unary(o, p, self.method_DryDownlink.clone())
    }

    fn dry_uplink(&self, o: ::grpc::RequestOptions, p: super::handler::DryUplinkMessage) -> ::grpc::SingleResponse<super::handler::DryUplinkResult> {
        self.grpc_client.call_unary(o, p, self.method_DryUplink.clone())
    }

    fn simulate_uplink(&self, o: ::grpc::RequestOptions, p: super::handler::SimulatedUplinkMessage) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_SimulateUplink.clone())
    }
}

// server

pub struct ApplicationManagerServer;


impl ApplicationManagerServer {
    pub fn new_service_def<H : ApplicationManager + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/handler.ApplicationManager",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/handler.ApplicationManager/RegisterApplication".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.register_application(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/handler.ApplicationManager/GetApplication".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_application(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/handler.ApplicationManager/SetApplication".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.set_application(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/handler.ApplicationManager/DeleteApplication".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_application(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/handler.ApplicationManager/GetDevice".to_string(),
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
                        name: "/handler.ApplicationManager/SetDevice".to_string(),
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
                        name: "/handler.ApplicationManager/DeleteDevice".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_device(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/handler.ApplicationManager/GetDevicesForApplication".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_devices_for_application(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/handler.ApplicationManager/DryDownlink".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.dry_downlink(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/handler.ApplicationManager/DryUplink".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.dry_uplink(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/handler.ApplicationManager/SimulateUplink".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.simulate_uplink(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait HandlerManager {
    fn get_status(&self, o: ::grpc::RequestOptions, p: super::handler::StatusRequest) -> ::grpc::SingleResponse<super::handler::Status>;
}

// client

pub struct HandlerManagerClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_GetStatus: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::handler::StatusRequest, super::handler::Status>>,
}

impl ::grpc::ClientStub for HandlerManagerClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        HandlerManagerClient {
            grpc_client: grpc_client,
            method_GetStatus: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/handler.HandlerManager/GetStatus".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl HandlerManager for HandlerManagerClient {
    fn get_status(&self, o: ::grpc::RequestOptions, p: super::handler::StatusRequest) -> ::grpc::SingleResponse<super::handler::Status> {
        self.grpc_client.call_unary(o, p, self.method_GetStatus.clone())
    }
}

// server

pub struct HandlerManagerServer;


impl HandlerManagerServer {
    pub fn new_service_def<H : HandlerManager + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/handler.HandlerManager",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/handler.HandlerManager/GetStatus".to_string(),
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
