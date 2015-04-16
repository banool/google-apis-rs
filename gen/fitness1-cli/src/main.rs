// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin, exit_status)]
#![plugin(docopt_macros)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

extern crate docopt;
extern crate yup_oauth2 as oauth2;
extern crate rustc_serialize;
extern crate serde;
extern crate hyper;
extern crate mime;
extern crate google_fitness1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  fitness1 [options] users data-sources-create <user-id> -r <kv>... [-p <v>]... [-o <out>]
  fitness1 [options] users data-sources-datasets-delete <user-id> <data-source-id> <dataset-id> [-p <v>]...
  fitness1 [options] users data-sources-datasets-get <user-id> <data-source-id> <dataset-id> [-p <v>]... [-o <out>]
  fitness1 [options] users data-sources-datasets-patch <user-id> <data-source-id> <dataset-id> -r <kv>... [-p <v>]... [-o <out>]
  fitness1 [options] users data-sources-get <user-id> <data-source-id> [-p <v>]... [-o <out>]
  fitness1 [options] users data-sources-list <user-id> [-p <v>]... [-o <out>]
  fitness1 [options] users data-sources-patch <user-id> <data-source-id> -r <kv>... [-p <v>]... [-o <out>]
  fitness1 [options] users data-sources-update <user-id> <data-source-id> -r <kv>... [-p <v>]... [-o <out>]
  fitness1 [options] users sessions-delete <user-id> <session-id> [-p <v>]...
  fitness1 [options] users sessions-list <user-id> [-p <v>]... [-o <out>]
  fitness1 [options] users sessions-update <user-id> <session-id> -r <kv>... [-p <v>]... [-o <out>]
  fitness1 --help

All documentation details can be found TODO: <URL to github.io docs here, see #51>

Configuration:
  --scope <url>  
            Specify the authentication a method should be executed in. Each scope requires
            the user to grant this application permission to use it.
            If unset, it defaults to the shortest scope url for a particular method.
  --config-dir <folder>
            A directory into which we will store our persistent data. Defaults to a user-writable
            directory that we will create during the first invocation.
            [default: ~/.google-service-cli]
");

mod cmn;
use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg, 
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use rustc_serialize::json;

struct Engine {
    opt: Options,
    hub: api::Fitness<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _users_data_sources_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::DataSource = Default::default();
        let mut call = self.hub.users().data_sources_create(&request, &self.opt.arg_user_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_application_init(request: &mut api::DataSource) {
                if request.application.is_none() {
                    request.application = Some(Default::default());
                }
            }
            
            fn request_data_type_init(request: &mut api::DataSource) {
                if request.data_type.is_none() {
                    request.data_type = Some(Default::default());
                }
            }
            
            fn request_device_init(request: &mut api::DataSource) {
                if request.device.is_none() {
                    request.device = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "data-stream-name" => {
                        request.data_stream_name = Some(value.unwrap_or("").to_string());
                    },
                "data-type.name" => {
                        request_data_type_init(&mut request);
                        request.data_type.as_mut().unwrap().name = value.unwrap_or("").to_string();
                    },
                "application.package-name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().package_name = value.unwrap_or("").to_string();
                    },
                "application.version" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().version = value.unwrap_or("").to_string();
                    },
                "application.name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().name = value.unwrap_or("").to_string();
                    },
                "application.details-url" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().details_url = value.unwrap_or("").to_string();
                    },
                "device.model" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().model = value.unwrap_or("").to_string();
                    },
                "device.version" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().version = value.unwrap_or("").to_string();
                    },
                "device.type" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "device.uid" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().uid = value.unwrap_or("").to_string();
                    },
                "device.manufacturer" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().manufacturer = value.unwrap_or("").to_string();
                    },
                "data-stream-id" => {
                        request_device_init(&mut request);
                        request.data_stream_id = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_device_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _users_data_sources_datasets_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().data_sources_datasets_delete(&self.opt.arg_user_id, &self.opt.arg_data_source_id, &self.opt.arg_dataset_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "modified-time-millis" => {
                    call = call.modified_time_millis(value.unwrap_or(""));
                },
                "current-time-millis" => {
                    call = call.current_time_millis(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _users_data_sources_datasets_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().data_sources_datasets_get(&self.opt.arg_user_id, &self.opt.arg_data_source_id, &self.opt.arg_dataset_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _users_data_sources_datasets_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Dataset = Default::default();
        let mut call = self.hub.users().data_sources_datasets_patch(&request, &self.opt.arg_user_id, &self.opt.arg_data_source_id, &self.opt.arg_dataset_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "current-time-millis" => {
                    call = call.current_time_millis(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "min-start-time-ns" => {
                        request.min_start_time_ns = Some(value.unwrap_or("").to_string());
                    },
                "next-page-token" => {
                        request.next_page_token = Some(value.unwrap_or("").to_string());
                    },
                "max-end-time-ns" => {
                        request.max_end_time_ns = Some(value.unwrap_or("").to_string());
                    },
                "data-source-id" => {
                        request.data_source_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _users_data_sources_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().data_sources_get(&self.opt.arg_user_id, &self.opt.arg_data_source_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _users_data_sources_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().data_sources_list(&self.opt.arg_user_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "data-type-name" => {
                    call = call.add_data_type_name(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _users_data_sources_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::DataSource = Default::default();
        let mut call = self.hub.users().data_sources_patch(&request, &self.opt.arg_user_id, &self.opt.arg_data_source_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_application_init(request: &mut api::DataSource) {
                if request.application.is_none() {
                    request.application = Some(Default::default());
                }
            }
            
            fn request_data_type_init(request: &mut api::DataSource) {
                if request.data_type.is_none() {
                    request.data_type = Some(Default::default());
                }
            }
            
            fn request_device_init(request: &mut api::DataSource) {
                if request.device.is_none() {
                    request.device = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "data-stream-name" => {
                        request.data_stream_name = Some(value.unwrap_or("").to_string());
                    },
                "data-type.name" => {
                        request_data_type_init(&mut request);
                        request.data_type.as_mut().unwrap().name = value.unwrap_or("").to_string();
                    },
                "application.package-name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().package_name = value.unwrap_or("").to_string();
                    },
                "application.version" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().version = value.unwrap_or("").to_string();
                    },
                "application.name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().name = value.unwrap_or("").to_string();
                    },
                "application.details-url" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().details_url = value.unwrap_or("").to_string();
                    },
                "device.model" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().model = value.unwrap_or("").to_string();
                    },
                "device.version" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().version = value.unwrap_or("").to_string();
                    },
                "device.type" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "device.uid" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().uid = value.unwrap_or("").to_string();
                    },
                "device.manufacturer" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().manufacturer = value.unwrap_or("").to_string();
                    },
                "data-stream-id" => {
                        request_device_init(&mut request);
                        request.data_stream_id = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_device_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _users_data_sources_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::DataSource = Default::default();
        let mut call = self.hub.users().data_sources_update(&request, &self.opt.arg_user_id, &self.opt.arg_data_source_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_application_init(request: &mut api::DataSource) {
                if request.application.is_none() {
                    request.application = Some(Default::default());
                }
            }
            
            fn request_data_type_init(request: &mut api::DataSource) {
                if request.data_type.is_none() {
                    request.data_type = Some(Default::default());
                }
            }
            
            fn request_device_init(request: &mut api::DataSource) {
                if request.device.is_none() {
                    request.device = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "data-stream-name" => {
                        request.data_stream_name = Some(value.unwrap_or("").to_string());
                    },
                "data-type.name" => {
                        request_data_type_init(&mut request);
                        request.data_type.as_mut().unwrap().name = value.unwrap_or("").to_string();
                    },
                "application.package-name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().package_name = value.unwrap_or("").to_string();
                    },
                "application.version" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().version = value.unwrap_or("").to_string();
                    },
                "application.name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().name = value.unwrap_or("").to_string();
                    },
                "application.details-url" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().details_url = value.unwrap_or("").to_string();
                    },
                "device.model" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().model = value.unwrap_or("").to_string();
                    },
                "device.version" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().version = value.unwrap_or("").to_string();
                    },
                "device.type" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "device.uid" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().uid = value.unwrap_or("").to_string();
                    },
                "device.manufacturer" => {
                        request_device_init(&mut request);
                        request.device.as_mut().unwrap().manufacturer = value.unwrap_or("").to_string();
                    },
                "data-stream-id" => {
                        request_device_init(&mut request);
                        request.data_stream_id = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_device_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _users_sessions_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().sessions_delete(&self.opt.arg_user_id, &self.opt.arg_session_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "current-time-millis" => {
                    call = call.current_time_millis(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _users_sessions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().sessions_list(&self.opt.arg_user_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "start-time" => {
                    call = call.start_time(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
                },
                "end-time" => {
                    call = call.end_time(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _users_sessions_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Session = Default::default();
        let mut call = self.hub.users().sessions_update(&request, &self.opt.arg_user_id, &self.opt.arg_session_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "current-time-millis" => {
                    call = call.current_time_millis(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_application_init(request: &mut api::Session) {
                if request.application.is_none() {
                    request.application = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "modified-time-millis" => {
                        request.modified_time_millis = Some(value.unwrap_or("").to_string());
                    },
                "end-time-millis" => {
                        request.end_time_millis = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "activity-type" => {
                        request.activity_type = Some(arg_from_str(value.unwrap_or("-0"), err, "activity-type", "integer"));
                    },
                "application.package-name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().package_name = value.unwrap_or("").to_string();
                    },
                "application.version" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().version = value.unwrap_or("").to_string();
                    },
                "application.name" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().name = value.unwrap_or("").to_string();
                    },
                "application.details-url" => {
                        request_application_init(&mut request);
                        request.application.as_mut().unwrap().details_url = value.unwrap_or("").to_string();
                    },
                "start-time-millis" => {
                        request_application_init(&mut request);
                        request.start_time_millis = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_application_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_application_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_users {
            if self.opt.cmd_data_sources_create {
                call_result = self._users_data_sources_create(dry_run, &mut err);
            } else if self.opt.cmd_data_sources_datasets_delete {
                call_result = self._users_data_sources_datasets_delete(dry_run, &mut err);
            } else if self.opt.cmd_data_sources_datasets_get {
                call_result = self._users_data_sources_datasets_get(dry_run, &mut err);
            } else if self.opt.cmd_data_sources_datasets_patch {
                call_result = self._users_data_sources_datasets_patch(dry_run, &mut err);
            } else if self.opt.cmd_data_sources_get {
                call_result = self._users_data_sources_get(dry_run, &mut err);
            } else if self.opt.cmd_data_sources_list {
                call_result = self._users_data_sources_list(dry_run, &mut err);
            } else if self.opt.cmd_data_sources_patch {
                call_result = self._users_data_sources_patch(dry_run, &mut err);
            } else if self.opt.cmd_data_sources_update {
                call_result = self._users_data_sources_update(dry_run, &mut err);
            } else if self.opt.cmd_sessions_delete {
                call_result = self._users_sessions_delete(dry_run, &mut err);
            } else if self.opt.cmd_sessions_list {
                call_result = self._users_sessions_list(dry_run, &mut err);
            } else if self.opt.cmd_sessions_update {
                call_result = self._users_sessions_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
        }
        (call_result, err_opt)
    }

    // Please note that this call will fail if any part of the opt can't be handled
    fn new(opt: Options) -> Result<Engine, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(&opt.flag_config_dir) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "fitness1-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "fitness1",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::Fitness::new(hyper::Client::new(), auth),
        };

        match engine._doit(true) {
            (_, Some(err)) => Err(err),
            _ => Ok(engine),
        }
    }

    // Execute the call with all the bells and whistles, informing the caller only if there was an error.
    // The absense of one indicates success.
    fn doit(&self) -> Option<api::Error> {
        self._doit(false).0
    }
}

fn main() {
    let opts: Options = Options::docopt().decode().unwrap_or_else(|e| e.exit());
    match Engine::new(opts) {
        Err(err) => {
            write!(io::stderr(), "{}", err).ok();
            env::set_exit_status(err.exit_code);
        },
        Ok(engine) => {
            if let Some(err) = engine.doit() {
                write!(io::stderr(), "{}", err).ok();
                env::set_exit_status(1);
            }
        }
    }
}