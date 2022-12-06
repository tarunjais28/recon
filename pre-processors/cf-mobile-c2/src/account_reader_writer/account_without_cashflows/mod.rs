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
//! Generated file from `recon_mobile_c2.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_11_0;

#[derive(PartialEq, Clone, Default)]
pub struct OutputAccount {
    // message fields
    pub receipt_no: ::std::string::String,
    pub completion_datetime: i64,
    pub initiation_datetime: i64,
    pub details: i64,
    pub tran_status: ::std::string::String,
    pub paid_in: f64,
    pub withdrawn: f64,
    pub balance: f64,
    pub balance_confirmed: ::std::string::String,
    pub reason_type: ::std::string::String,
    pub other_party_info: ::std::string::String,
    pub linked_tran_id: ::std::string::String,
    pub ac_no: i64,
    pub completion_date: i64,
    pub completion_time: i64,
    pub initiation_date: i64,
    pub initiation_time: i64,
    pub amount: f64,
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

    // string receipt_no = 1;

    pub fn get_receipt_no(&self) -> &str {
        &self.receipt_no
    }
    pub fn clear_receipt_no(&mut self) {
        self.receipt_no.clear();
    }

    // Param is passed by value, moved
    pub fn set_receipt_no(&mut self, v: ::std::string::String) {
        self.receipt_no = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_receipt_no(&mut self) -> &mut ::std::string::String {
        &mut self.receipt_no
    }

    // Take field
    pub fn take_receipt_no(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.receipt_no, ::std::string::String::new())
    }

    // int64 completion_datetime = 2;

    pub fn get_completion_datetime(&self) -> i64 {
        self.completion_datetime
    }
    pub fn clear_completion_datetime(&mut self) {
        self.completion_datetime = 0;
    }

    // Param is passed by value, moved
    pub fn set_completion_datetime(&mut self, v: i64) {
        self.completion_datetime = v;
    }

    // int64 initiation_datetime = 3;

    pub fn get_initiation_datetime(&self) -> i64 {
        self.initiation_datetime
    }
    pub fn clear_initiation_datetime(&mut self) {
        self.initiation_datetime = 0;
    }

    // Param is passed by value, moved
    pub fn set_initiation_datetime(&mut self, v: i64) {
        self.initiation_datetime = v;
    }

    // int64 details = 4;

    pub fn get_details(&self) -> i64 {
        self.details
    }
    pub fn clear_details(&mut self) {
        self.details = 0;
    }

    // Param is passed by value, moved
    pub fn set_details(&mut self, v: i64) {
        self.details = v;
    }

    // string tran_status = 5;

    pub fn get_tran_status(&self) -> &str {
        &self.tran_status
    }
    pub fn clear_tran_status(&mut self) {
        self.tran_status.clear();
    }

    // Param is passed by value, moved
    pub fn set_tran_status(&mut self, v: ::std::string::String) {
        self.tran_status = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tran_status(&mut self) -> &mut ::std::string::String {
        &mut self.tran_status
    }

    // Take field
    pub fn take_tran_status(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tran_status, ::std::string::String::new())
    }

    // double paid_in = 6;

    pub fn get_paid_in(&self) -> f64 {
        self.paid_in
    }
    pub fn clear_paid_in(&mut self) {
        self.paid_in = 0.;
    }

    // Param is passed by value, moved
    pub fn set_paid_in(&mut self, v: f64) {
        self.paid_in = v;
    }

    // double withdrawn = 7;

    pub fn get_withdrawn(&self) -> f64 {
        self.withdrawn
    }
    pub fn clear_withdrawn(&mut self) {
        self.withdrawn = 0.;
    }

    // Param is passed by value, moved
    pub fn set_withdrawn(&mut self, v: f64) {
        self.withdrawn = v;
    }

    // double balance = 8;

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

    // string balance_confirmed = 9;

    pub fn get_balance_confirmed(&self) -> &str {
        &self.balance_confirmed
    }
    pub fn clear_balance_confirmed(&mut self) {
        self.balance_confirmed.clear();
    }

    // Param is passed by value, moved
    pub fn set_balance_confirmed(&mut self, v: ::std::string::String) {
        self.balance_confirmed = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_balance_confirmed(&mut self) -> &mut ::std::string::String {
        &mut self.balance_confirmed
    }

    // Take field
    pub fn take_balance_confirmed(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.balance_confirmed, ::std::string::String::new())
    }

    // string reason_type = 10;

    pub fn get_reason_type(&self) -> &str {
        &self.reason_type
    }
    pub fn clear_reason_type(&mut self) {
        self.reason_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_reason_type(&mut self, v: ::std::string::String) {
        self.reason_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reason_type(&mut self) -> &mut ::std::string::String {
        &mut self.reason_type
    }

    // Take field
    pub fn take_reason_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.reason_type, ::std::string::String::new())
    }

    // string other_party_info = 11;

    pub fn get_other_party_info(&self) -> &str {
        &self.other_party_info
    }
    pub fn clear_other_party_info(&mut self) {
        self.other_party_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_other_party_info(&mut self, v: ::std::string::String) {
        self.other_party_info = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_other_party_info(&mut self) -> &mut ::std::string::String {
        &mut self.other_party_info
    }

    // Take field
    pub fn take_other_party_info(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.other_party_info, ::std::string::String::new())
    }

    // string linked_tran_id = 12;

    pub fn get_linked_tran_id(&self) -> &str {
        &self.linked_tran_id
    }
    pub fn clear_linked_tran_id(&mut self) {
        self.linked_tran_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_linked_tran_id(&mut self, v: ::std::string::String) {
        self.linked_tran_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_linked_tran_id(&mut self) -> &mut ::std::string::String {
        &mut self.linked_tran_id
    }

    // Take field
    pub fn take_linked_tran_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.linked_tran_id, ::std::string::String::new())
    }

    // int64 ac_no = 13;

    pub fn get_ac_no(&self) -> i64 {
        self.ac_no
    }
    pub fn clear_ac_no(&mut self) {
        self.ac_no = 0;
    }

    // Param is passed by value, moved
    pub fn set_ac_no(&mut self, v: i64) {
        self.ac_no = v;
    }

    // int64 completion_date = 14;

    pub fn get_completion_date(&self) -> i64 {
        self.completion_date
    }
    pub fn clear_completion_date(&mut self) {
        self.completion_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_completion_date(&mut self, v: i64) {
        self.completion_date = v;
    }

    // int64 completion_time = 15;

    pub fn get_completion_time(&self) -> i64 {
        self.completion_time
    }
    pub fn clear_completion_time(&mut self) {
        self.completion_time = 0;
    }

    // Param is passed by value, moved
    pub fn set_completion_time(&mut self, v: i64) {
        self.completion_time = v;
    }

    // int64 initiation_date = 16;

    pub fn get_initiation_date(&self) -> i64 {
        self.initiation_date
    }
    pub fn clear_initiation_date(&mut self) {
        self.initiation_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_initiation_date(&mut self, v: i64) {
        self.initiation_date = v;
    }

    // int64 initiation_time = 17;

    pub fn get_initiation_time(&self) -> i64 {
        self.initiation_time
    }
    pub fn clear_initiation_time(&mut self) {
        self.initiation_time = 0;
    }

    // Param is passed by value, moved
    pub fn set_initiation_time(&mut self, v: i64) {
        self.initiation_time = v;
    }

    // double amount = 18;

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
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.receipt_no,
                    )?;
                }
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.completion_datetime = tmp;
                }
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.initiation_datetime = tmp;
                }
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.details = tmp;
                }
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.tran_status,
                    )?;
                }
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_double()?;
                    self.paid_in = tmp;
                }
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_double()?;
                    self.withdrawn = tmp;
                }
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_double()?;
                    self.balance = tmp;
                }
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.balance_confirmed,
                    )?;
                }
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.reason_type,
                    )?;
                }
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.other_party_info,
                    )?;
                }
                12 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.linked_tran_id,
                    )?;
                }
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.ac_no = tmp;
                }
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.completion_date = tmp;
                }
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.completion_time = tmp;
                }
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.initiation_date = tmp;
                }
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.initiation_time = tmp;
                }
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_double()?;
                    self.amount = tmp;
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
        if !self.receipt_no.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.receipt_no);
        }
        if self.completion_datetime != 0 {
            my_size += ::protobuf::rt::value_size(
                2,
                self.completion_datetime,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if self.initiation_datetime != 0 {
            my_size += ::protobuf::rt::value_size(
                3,
                self.initiation_datetime,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if self.details != 0 {
            my_size += ::protobuf::rt::value_size(
                4,
                self.details,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if !self.tran_status.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.tran_status);
        }
        if self.paid_in != 0. {
            my_size += 9;
        }
        if self.withdrawn != 0. {
            my_size += 9;
        }
        if self.balance != 0. {
            my_size += 9;
        }
        if !self.balance_confirmed.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.balance_confirmed);
        }
        if !self.reason_type.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.reason_type);
        }
        if !self.other_party_info.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.other_party_info);
        }
        if !self.linked_tran_id.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.linked_tran_id);
        }
        if self.ac_no != 0 {
            my_size +=
                ::protobuf::rt::value_size(13, self.ac_no, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.completion_date != 0 {
            my_size += ::protobuf::rt::value_size(
                14,
                self.completion_date,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if self.completion_time != 0 {
            my_size += ::protobuf::rt::value_size(
                15,
                self.completion_time,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if self.initiation_date != 0 {
            my_size += ::protobuf::rt::value_size(
                16,
                self.initiation_date,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if self.initiation_time != 0 {
            my_size += ::protobuf::rt::value_size(
                17,
                self.initiation_time,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if self.amount != 0. {
            my_size += 10;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(
        &self,
        os: &mut ::protobuf::CodedOutputStream<'_>,
    ) -> ::protobuf::ProtobufResult<()> {
        if !self.receipt_no.is_empty() {
            os.write_string(1, &self.receipt_no)?;
        }
        if self.completion_datetime != 0 {
            os.write_int64(2, self.completion_datetime)?;
        }
        if self.initiation_datetime != 0 {
            os.write_int64(3, self.initiation_datetime)?;
        }
        if self.details != 0 {
            os.write_int64(4, self.details)?;
        }
        if !self.tran_status.is_empty() {
            os.write_string(5, &self.tran_status)?;
        }
        if self.paid_in != 0. {
            os.write_double(6, self.paid_in)?;
        }
        if self.withdrawn != 0. {
            os.write_double(7, self.withdrawn)?;
        }
        if self.balance != 0. {
            os.write_double(8, self.balance)?;
        }
        if !self.balance_confirmed.is_empty() {
            os.write_string(9, &self.balance_confirmed)?;
        }
        if !self.reason_type.is_empty() {
            os.write_string(10, &self.reason_type)?;
        }
        if !self.other_party_info.is_empty() {
            os.write_string(11, &self.other_party_info)?;
        }
        if !self.linked_tran_id.is_empty() {
            os.write_string(12, &self.linked_tran_id)?;
        }
        if self.ac_no != 0 {
            os.write_int64(13, self.ac_no)?;
        }
        if self.completion_date != 0 {
            os.write_int64(14, self.completion_date)?;
        }
        if self.completion_time != 0 {
            os.write_int64(15, self.completion_time)?;
        }
        if self.initiation_date != 0 {
            os.write_int64(16, self.initiation_date)?;
        }
        if self.initiation_time != 0 {
            os.write_int64(17, self.initiation_time)?;
        }
        if self.amount != 0. {
            os.write_double(18, self.amount)?;
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
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "receipt_no",
                    |m: &OutputAccount| &m.receipt_no,
                    |m: &mut OutputAccount| &mut m.receipt_no,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "completion_datetime",
                    |m: &OutputAccount| &m.completion_datetime,
                    |m: &mut OutputAccount| &mut m.completion_datetime,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "initiation_datetime",
                    |m: &OutputAccount| &m.initiation_datetime,
                    |m: &mut OutputAccount| &mut m.initiation_datetime,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "details",
                    |m: &OutputAccount| &m.details,
                    |m: &mut OutputAccount| &mut m.details,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "tran_status",
                    |m: &OutputAccount| &m.tran_status,
                    |m: &mut OutputAccount| &mut m.tran_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeDouble,
                >(
                    "paid_in",
                    |m: &OutputAccount| &m.paid_in,
                    |m: &mut OutputAccount| &mut m.paid_in,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeDouble,
                >(
                    "withdrawn",
                    |m: &OutputAccount| &m.withdrawn,
                    |m: &mut OutputAccount| &mut m.withdrawn,
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
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "balance_confirmed",
                    |m: &OutputAccount| &m.balance_confirmed,
                    |m: &mut OutputAccount| &mut m.balance_confirmed,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "reason_type",
                    |m: &OutputAccount| &m.reason_type,
                    |m: &mut OutputAccount| &mut m.reason_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "other_party_info",
                    |m: &OutputAccount| &m.other_party_info,
                    |m: &mut OutputAccount| &mut m.other_party_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "linked_tran_id",
                    |m: &OutputAccount| &m.linked_tran_id,
                    |m: &mut OutputAccount| &mut m.linked_tran_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "ac_no",
                    |m: &OutputAccount| &m.ac_no,
                    |m: &mut OutputAccount| &mut m.ac_no,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "completion_date",
                    |m: &OutputAccount| &m.completion_date,
                    |m: &mut OutputAccount| &mut m.completion_date,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "completion_time",
                    |m: &OutputAccount| &m.completion_time,
                    |m: &mut OutputAccount| &mut m.completion_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "initiation_date",
                    |m: &OutputAccount| &m.initiation_date,
                    |m: &mut OutputAccount| &mut m.initiation_date,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "initiation_time",
                    |m: &OutputAccount| &m.initiation_time,
                    |m: &mut OutputAccount| &mut m.initiation_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeDouble,
                >(
                    "amount",
                    |m: &OutputAccount| &m.amount,
                    |m: &mut OutputAccount| &mut m.amount,
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
        self.receipt_no.clear();
        self.completion_datetime = 0;
        self.initiation_datetime = 0;
        self.details = 0;
        self.tran_status.clear();
        self.paid_in = 0.;
        self.withdrawn = 0.;
        self.balance = 0.;
        self.balance_confirmed.clear();
        self.reason_type.clear();
        self.other_party_info.clear();
        self.linked_tran_id.clear();
        self.ac_no = 0;
        self.completion_date = 0;
        self.completion_time = 0;
        self.initiation_date = 0;
        self.initiation_time = 0;
        self.amount = 0.;
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
    \n\x15recon_mobile_c2.proto\"\x8b\x05\n\rOutputAccount\x12\x1d\n\nreceip\
    t_no\x18\x01\x20\x01(\tR\treceiptNo\x12/\n\x13completion_datetime\x18\
    \x02\x20\x01(\x03R\x12completionDatetime\x12/\n\x13initiation_datetime\
    \x18\x03\x20\x01(\x03R\x12initiationDatetime\x12\x18\n\x07details\x18\
    \x04\x20\x01(\x03R\x07details\x12\x1f\n\x0btran_status\x18\x05\x20\x01(\
    \tR\ntranStatus\x12\x17\n\x07paid_in\x18\x06\x20\x01(\x01R\x06paidIn\x12\
    \x1c\n\twithdrawn\x18\x07\x20\x01(\x01R\twithdrawn\x12\x18\n\x07balance\
    \x18\x08\x20\x01(\x01R\x07balance\x12+\n\x11balance_confirmed\x18\t\x20\
    \x01(\tR\x10balanceConfirmed\x12\x1f\n\x0breason_type\x18\n\x20\x01(\tR\
    \nreasonType\x12(\n\x10other_party_info\x18\x0b\x20\x01(\tR\x0eotherPart\
    yInfo\x12$\n\x0elinked_tran_id\x18\x0c\x20\x01(\tR\x0clinkedTranId\x12\
    \x13\n\x05ac_no\x18\r\x20\x01(\x03R\x04acNo\x12'\n\x0fcompletion_date\
    \x18\x0e\x20\x01(\x03R\x0ecompletionDate\x12'\n\x0fcompletion_time\x18\
    \x0f\x20\x01(\x03R\x0ecompletionTime\x12'\n\x0finitiation_date\x18\x10\
    \x20\x01(\x03R\x0einitiationDate\x12'\n\x0finitiation_time\x18\x11\x20\
    \x01(\x03R\x0einitiationTime\x12\x16\n\x06amount\x18\x12\x20\x01(\x01R\
    \x06amountb\x06proto3\
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
