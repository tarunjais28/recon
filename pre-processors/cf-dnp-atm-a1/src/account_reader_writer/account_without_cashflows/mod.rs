// This file is generated by rust-protobuf 2.11.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `recon_dnp_exam.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_11_0;

#[derive(PartialEq, Clone, Default)]
pub struct OutputAccount {
    // message fields
    pub tran_no: i64,
    pub description: ::std::string::String,
    pub amount: f64,
    pub date: i64,
    pub time: i64,
    pub balance: f64,
    pub account_number: i64,
    pub unwanted: ::std::string::String,
    pub datetime: i64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a OutputAccount {
    fn default() -> &'a OutputAccount {
        <OutputAccount as ::protobuf::Message>::default_instance()
    }
}

impl OutputAccount {
    pub fn new() -> OutputAccount {
        ::std::default::Default::default()
    }

    // int64 tran_no = 1;

    pub fn get_tran_no(&self) -> i64 {
        self.tran_no
    }
    pub fn clear_tran_no(&mut self) {
        self.tran_no = 0;
    }

    // Param is passed by value, moved
    pub fn set_tran_no(&mut self, v: i64) {
        self.tran_no = v;
    }

    // string description = 2;

    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    // double amount = 3;

    pub fn get_amount(&self) -> f64 {
        self.amount
    }
    pub fn clear_amount(&mut self) {
        self.amount = 0.;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: f64) {
        self.amount = v;
    }

    // int64 date = 4;

    pub fn get_date(&self) -> i64 {
        self.date
    }
    pub fn clear_date(&mut self) {
        self.date = 0;
    }

    // Param is passed by value, moved
    pub fn set_date(&mut self, v: i64) {
        self.date = v;
    }

    // int64 time = 5;

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

    // double balance = 6;

    pub fn get_balance(&self) -> f64 {
        self.balance
    }
    pub fn clear_balance(&mut self) {
        self.balance = 0.;
    }

    // Param is passed by value, moved
    pub fn set_balance(&mut self, v: f64) {
        self.balance = v;
    }

    // int64 account_number = 7;

    pub fn get_account_number(&self) -> i64 {
        self.account_number
    }
    pub fn clear_account_number(&mut self) {
        self.account_number = 0;
    }

    // Param is passed by value, moved
    pub fn set_account_number(&mut self, v: i64) {
        self.account_number = v;
    }

    // string unwanted = 8;

    pub fn get_unwanted(&self) -> &str {
        &self.unwanted
    }
    pub fn clear_unwanted(&mut self) {
        self.unwanted.clear();
    }

    // Param is passed by value, moved
    pub fn set_unwanted(&mut self, v: ::std::string::String) {
        self.unwanted = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unwanted(&mut self) -> &mut ::std::string::String {
        &mut self.unwanted
    }

    // Take field
    pub fn take_unwanted(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.unwanted, ::std::string::String::new())
    }

    // int64 datetime = 9;

    pub fn get_datetime(&self) -> i64 {
        self.datetime
    }
    pub fn clear_datetime(&mut self) {
        self.datetime = 0;
    }

    // Param is passed by value, moved
    pub fn set_datetime(&mut self, v: i64) {
        self.datetime = v;
    }
}

impl ::protobuf::Message for OutputAccount {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(
        &mut self,
        is: &mut ::protobuf::CodedInputStream<'_>,
    ) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.tran_no = tmp;
                }
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.description,
                    )?;
                }
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_double()?;
                    self.amount = tmp;
                }
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.date = tmp;
                }
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.time = tmp;
                }
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_double()?;
                    self.balance = tmp;
                }
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.account_number = tmp;
                }
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.unwanted,
                    )?;
                }
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.datetime = tmp;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(
                        field_number,
                        wire_type,
                        is,
                        self.mut_unknown_fields(),
                    )?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.tran_no != 0 {
            my_size += ::protobuf::rt::value_size(
                1,
                self.tran_no,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.description);
        }
        if self.amount != 0. {
            my_size += 9;
        }
        if self.date != 0 {
            my_size +=
                ::protobuf::rt::value_size(4, self.date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.time != 0 {
            my_size +=
                ::protobuf::rt::value_size(5, self.time, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.balance != 0. {
            my_size += 9;
        }
        if self.account_number != 0 {
            my_size += ::protobuf::rt::value_size(
                7,
                self.account_number,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if !self.unwanted.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.unwanted);
        }
        if self.datetime != 0 {
            my_size += ::protobuf::rt::value_size(
                9,
                self.datetime,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(
        &self,
        os: &mut ::protobuf::CodedOutputStream<'_>,
    ) -> ::protobuf::ProtobufResult<()> {
        if self.tran_no != 0 {
            os.write_int64(1, self.tran_no)?;
        }
        if !self.description.is_empty() {
            os.write_string(2, &self.description)?;
        }
        if self.amount != 0. {
            os.write_double(3, self.amount)?;
        }
        if self.date != 0 {
            os.write_int64(4, self.date)?;
        }
        if self.time != 0 {
            os.write_int64(5, self.time)?;
        }
        if self.balance != 0. {
            os.write_double(6, self.balance)?;
        }
        if self.account_number != 0 {
            os.write_int64(7, self.account_number)?;
        }
        if !self.unwanted.is_empty() {
            os.write_string(8, &self.unwanted)?;
        }
        if self.datetime != 0 {
            os.write_int64(9, self.datetime)?;
        }
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> OutputAccount {
        OutputAccount::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "tran_no",
                    |m: &OutputAccount| &m.tran_no,
                    |m: &mut OutputAccount| &mut m.tran_no,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "description",
                    |m: &OutputAccount| &m.description,
                    |m: &mut OutputAccount| &mut m.description,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeDouble,
                >(
                    "amount",
                    |m: &OutputAccount| &m.amount,
                    |m: &mut OutputAccount| &mut m.amount,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "date",
                    |m: &OutputAccount| &m.date,
                    |m: &mut OutputAccount| &mut m.date,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "time",
                    |m: &OutputAccount| &m.time,
                    |m: &mut OutputAccount| &mut m.time,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeDouble,
                >(
                    "balance",
                    |m: &OutputAccount| &m.balance,
                    |m: &mut OutputAccount| &mut m.balance,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "account_number",
                    |m: &OutputAccount| &m.account_number,
                    |m: &mut OutputAccount| &mut m.account_number,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "unwanted",
                    |m: &OutputAccount| &m.unwanted,
                    |m: &mut OutputAccount| &mut m.unwanted,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "datetime",
                    |m: &OutputAccount| &m.datetime,
                    |m: &mut OutputAccount| &mut m.datetime,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OutputAccount>(
                    "OutputAccount",
                    fields,
                    file_descriptor_proto(),
                )
            })
        }
    }

    fn default_instance() -> &'static OutputAccount {
        static mut instance: ::protobuf::lazy::Lazy<OutputAccount> = ::protobuf::lazy::Lazy::INIT;
        unsafe { instance.get(OutputAccount::new) }
    }
}

impl ::protobuf::Clear for OutputAccount {
    fn clear(&mut self) {
        self.tran_no = 0;
        self.description.clear();
        self.amount = 0.;
        self.date = 0;
        self.time = 0;
        self.balance = 0.;
        self.account_number = 0;
        self.unwanted.clear();
        self.datetime = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OutputAccount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputAccount {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14recon_dnp_exam.proto\"\x83\x02\n\rOutputAccount\x12\x17\n\x07tran_\
    no\x18\x01\x20\x01(\x03R\x06tranNo\x12\x20\n\x0bdescription\x18\x02\x20\
    \x01(\tR\x0bdescription\x12\x16\n\x06amount\x18\x03\x20\x01(\x01R\x06amo\
    unt\x12\x12\n\x04date\x18\x04\x20\x01(\x03R\x04date\x12\x12\n\x04time\
    \x18\x05\x20\x01(\x03R\x04time\x12\x18\n\x07balance\x18\x06\x20\x01(\x01\
    R\x07balance\x12%\n\x0eaccount_number\x18\x07\x20\x01(\x03R\raccountNumb\
    er\x12\x1a\n\x08unwanted\x18\x08\x20\x01(\tR\x08unwanted\x12\x1a\n\x08da\
    tetime\x18\t\x20\x01(\x03R\x08datetimeb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<
    ::protobuf::descriptor::FileDescriptorProto,
> = ::protobuf::lazy::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe { file_descriptor_proto_lazy.get(|| parse_descriptor_proto()) }
}
