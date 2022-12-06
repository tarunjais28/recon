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
//! Generated file from `recon_atm_a1.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_11_0;

#[derive(PartialEq, Clone, Default)]
pub struct OutputAccount {
    // message fields
    pub tran_date: i64,
    pub tran_time: ::std::string::String,
    pub value_date: i64,
    pub tran_id: ::std::string::String,
    pub tran_particular: ::std::string::String,
    pub tran_remarks: i64,
    pub ref_num: i64,
    pub stan: i64,
    pub term_id: i64,
    pub debit_credit: ::std::string::String,
    pub cr: f64,
    pub dr: f64,
    pub outstanding: f64,
    pub customer_account: i64,
    pub card_no: ::std::string::String,
    pub datetime: i64,
    pub ccy: ::std::string::String,
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

    // int64 tran_date = 1;

    pub fn get_tran_date(&self) -> i64 {
        self.tran_date
    }
    pub fn clear_tran_date(&mut self) {
        self.tran_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_tran_date(&mut self, v: i64) {
        self.tran_date = v;
    }

    // string tran_time = 2;

    pub fn get_tran_time(&self) -> &str {
        &self.tran_time
    }
    pub fn clear_tran_time(&mut self) {
        self.tran_time.clear();
    }

    // Param is passed by value, moved
    pub fn set_tran_time(&mut self, v: ::std::string::String) {
        self.tran_time = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tran_time(&mut self) -> &mut ::std::string::String {
        &mut self.tran_time
    }

    // Take field
    pub fn take_tran_time(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tran_time, ::std::string::String::new())
    }

    // int64 value_date = 3;

    pub fn get_value_date(&self) -> i64 {
        self.value_date
    }
    pub fn clear_value_date(&mut self) {
        self.value_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_value_date(&mut self, v: i64) {
        self.value_date = v;
    }

    // string tran_id = 4;

    pub fn get_tran_id(&self) -> &str {
        &self.tran_id
    }
    pub fn clear_tran_id(&mut self) {
        self.tran_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_tran_id(&mut self, v: ::std::string::String) {
        self.tran_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tran_id(&mut self) -> &mut ::std::string::String {
        &mut self.tran_id
    }

    // Take field
    pub fn take_tran_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tran_id, ::std::string::String::new())
    }

    // string tran_particular = 5;

    pub fn get_tran_particular(&self) -> &str {
        &self.tran_particular
    }
    pub fn clear_tran_particular(&mut self) {
        self.tran_particular.clear();
    }

    // Param is passed by value, moved
    pub fn set_tran_particular(&mut self, v: ::std::string::String) {
        self.tran_particular = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tran_particular(&mut self) -> &mut ::std::string::String {
        &mut self.tran_particular
    }

    // Take field
    pub fn take_tran_particular(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tran_particular, ::std::string::String::new())
    }

    // int64 tran_remarks = 6;

    pub fn get_tran_remarks(&self) -> i64 {
        self.tran_remarks
    }
    pub fn clear_tran_remarks(&mut self) {
        self.tran_remarks = 0;
    }

    // Param is passed by value, moved
    pub fn set_tran_remarks(&mut self, v: i64) {
        self.tran_remarks = v;
    }

    // int64 ref_num = 7;

    pub fn get_ref_num(&self) -> i64 {
        self.ref_num
    }
    pub fn clear_ref_num(&mut self) {
        self.ref_num = 0;
    }

    // Param is passed by value, moved
    pub fn set_ref_num(&mut self, v: i64) {
        self.ref_num = v;
    }

    // int64 stan = 8;

    pub fn get_stan(&self) -> i64 {
        self.stan
    }
    pub fn clear_stan(&mut self) {
        self.stan = 0;
    }

    // Param is passed by value, moved
    pub fn set_stan(&mut self, v: i64) {
        self.stan = v;
    }

    // int64 term_id = 9;

    pub fn get_term_id(&self) -> i64 {
        self.term_id
    }
    pub fn clear_term_id(&mut self) {
        self.term_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_term_id(&mut self, v: i64) {
        self.term_id = v;
    }

    // string debit_credit = 10;

    pub fn get_debit_credit(&self) -> &str {
        &self.debit_credit
    }
    pub fn clear_debit_credit(&mut self) {
        self.debit_credit.clear();
    }

    // Param is passed by value, moved
    pub fn set_debit_credit(&mut self, v: ::std::string::String) {
        self.debit_credit = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_debit_credit(&mut self) -> &mut ::std::string::String {
        &mut self.debit_credit
    }

    // Take field
    pub fn take_debit_credit(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.debit_credit, ::std::string::String::new())
    }

    // double cr = 11;

    pub fn get_cr(&self) -> f64 {
        self.cr
    }
    pub fn clear_cr(&mut self) {
        self.cr = 0.;
    }

    // Param is passed by value, moved
    pub fn set_cr(&mut self, v: f64) {
        self.cr = v;
    }

    // double dr = 12;

    pub fn get_dr(&self) -> f64 {
        self.dr
    }
    pub fn clear_dr(&mut self) {
        self.dr = 0.;
    }

    // Param is passed by value, moved
    pub fn set_dr(&mut self, v: f64) {
        self.dr = v;
    }

    // double outstanding = 13;

    pub fn get_outstanding(&self) -> f64 {
        self.outstanding
    }
    pub fn clear_outstanding(&mut self) {
        self.outstanding = 0.;
    }

    // Param is passed by value, moved
    pub fn set_outstanding(&mut self, v: f64) {
        self.outstanding = v;
    }

    // int64 customer_account = 14;

    pub fn get_customer_account(&self) -> i64 {
        self.customer_account
    }
    pub fn clear_customer_account(&mut self) {
        self.customer_account = 0;
    }

    // Param is passed by value, moved
    pub fn set_customer_account(&mut self, v: i64) {
        self.customer_account = v;
    }

    // string card_no = 15;

    pub fn get_card_no(&self) -> &str {
        &self.card_no
    }
    pub fn clear_card_no(&mut self) {
        self.card_no.clear();
    }

    // Param is passed by value, moved
    pub fn set_card_no(&mut self, v: ::std::string::String) {
        self.card_no = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_card_no(&mut self) -> &mut ::std::string::String {
        &mut self.card_no
    }

    // Take field
    pub fn take_card_no(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.card_no, ::std::string::String::new())
    }

    // int64 datetime = 16;

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

    // string ccy = 17;

    pub fn get_ccy(&self) -> &str {
        &self.ccy
    }
    pub fn clear_ccy(&mut self) {
        self.ccy.clear();
    }

    // Param is passed by value, moved
    pub fn set_ccy(&mut self, v: ::std::string::String) {
        self.ccy = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ccy(&mut self) -> &mut ::std::string::String {
        &mut self.ccy
    }

    // Take field
    pub fn take_ccy(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ccy, ::std::string::String::new())
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.tran_date = tmp;
                }
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.tran_time,
                    )?;
                }
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.value_date = tmp;
                }
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.tran_id,
                    )?;
                }
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.tran_particular,
                    )?;
                }
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.tran_remarks = tmp;
                }
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.ref_num = tmp;
                }
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.stan = tmp;
                }
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.term_id = tmp;
                }
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.debit_credit,
                    )?;
                }
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_double()?;
                    self.cr = tmp;
                }
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_double()?;
                    self.dr = tmp;
                }
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_double()?;
                    self.outstanding = tmp;
                }
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.customer_account = tmp;
                }
                15 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.card_no,
                    )?;
                }
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.datetime = tmp;
                }
                17 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ccy)?;
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
        if self.tran_date != 0 {
            my_size += ::protobuf::rt::value_size(
                1,
                self.tran_date,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if !self.tran_time.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.tran_time);
        }
        if self.value_date != 0 {
            my_size += ::protobuf::rt::value_size(
                3,
                self.value_date,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if !self.tran_id.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.tran_id);
        }
        if !self.tran_particular.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.tran_particular);
        }
        if self.tran_remarks != 0 {
            my_size += ::protobuf::rt::value_size(
                6,
                self.tran_remarks,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if self.ref_num != 0 {
            my_size += ::protobuf::rt::value_size(
                7,
                self.ref_num,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if self.stan != 0 {
            my_size +=
                ::protobuf::rt::value_size(8, self.stan, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.term_id != 0 {
            my_size += ::protobuf::rt::value_size(
                9,
                self.term_id,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if !self.debit_credit.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.debit_credit);
        }
        if self.cr != 0. {
            my_size += 9;
        }
        if self.dr != 0. {
            my_size += 9;
        }
        if self.outstanding != 0. {
            my_size += 9;
        }
        if self.customer_account != 0 {
            my_size += ::protobuf::rt::value_size(
                14,
                self.customer_account,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if !self.card_no.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.card_no);
        }
        if self.datetime != 0 {
            my_size += ::protobuf::rt::value_size(
                16,
                self.datetime,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if !self.ccy.is_empty() {
            my_size += ::protobuf::rt::string_size(17, &self.ccy);
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
        if self.tran_date != 0 {
            os.write_int64(1, self.tran_date)?;
        }
        if !self.tran_time.is_empty() {
            os.write_string(2, &self.tran_time)?;
        }
        if self.value_date != 0 {
            os.write_int64(3, self.value_date)?;
        }
        if !self.tran_id.is_empty() {
            os.write_string(4, &self.tran_id)?;
        }
        if !self.tran_particular.is_empty() {
            os.write_string(5, &self.tran_particular)?;
        }
        if self.tran_remarks != 0 {
            os.write_int64(6, self.tran_remarks)?;
        }
        if self.ref_num != 0 {
            os.write_int64(7, self.ref_num)?;
        }
        if self.stan != 0 {
            os.write_int64(8, self.stan)?;
        }
        if self.term_id != 0 {
            os.write_int64(9, self.term_id)?;
        }
        if !self.debit_credit.is_empty() {
            os.write_string(10, &self.debit_credit)?;
        }
        if self.cr != 0. {
            os.write_double(11, self.cr)?;
        }
        if self.dr != 0. {
            os.write_double(12, self.dr)?;
        }
        if self.outstanding != 0. {
            os.write_double(13, self.outstanding)?;
        }
        if self.customer_account != 0 {
            os.write_int64(14, self.customer_account)?;
        }
        if !self.card_no.is_empty() {
            os.write_string(15, &self.card_no)?;
        }
        if self.datetime != 0 {
            os.write_int64(16, self.datetime)?;
        }
        if !self.ccy.is_empty() {
            os.write_string(17, &self.ccy)?;
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
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "tran_date",
                    |m: &OutputAccount| &m.tran_date,
                    |m: &mut OutputAccount| &mut m.tran_date,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "tran_time",
                    |m: &OutputAccount| &m.tran_time,
                    |m: &mut OutputAccount| &mut m.tran_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "value_date",
                    |m: &OutputAccount| &m.value_date,
                    |m: &mut OutputAccount| &mut m.value_date,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "tran_id",
                    |m: &OutputAccount| &m.tran_id,
                    |m: &mut OutputAccount| &mut m.tran_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "tran_particular",
                    |m: &OutputAccount| &m.tran_particular,
                    |m: &mut OutputAccount| &mut m.tran_particular,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "tran_remarks",
                    |m: &OutputAccount| &m.tran_remarks,
                    |m: &mut OutputAccount| &mut m.tran_remarks,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "ref_num",
                    |m: &OutputAccount| &m.ref_num,
                    |m: &mut OutputAccount| &mut m.ref_num,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "stan",
                    |m: &OutputAccount| &m.stan,
                    |m: &mut OutputAccount| &mut m.stan,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "term_id",
                    |m: &OutputAccount| &m.term_id,
                    |m: &mut OutputAccount| &mut m.term_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "debit_credit",
                    |m: &OutputAccount| &m.debit_credit,
                    |m: &mut OutputAccount| &mut m.debit_credit,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeDouble,
                >(
                    "cr",
                    |m: &OutputAccount| &m.cr,
                    |m: &mut OutputAccount| &mut m.cr,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeDouble,
                >(
                    "dr",
                    |m: &OutputAccount| &m.dr,
                    |m: &mut OutputAccount| &mut m.dr,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeDouble,
                >(
                    "outstanding",
                    |m: &OutputAccount| &m.outstanding,
                    |m: &mut OutputAccount| &mut m.outstanding,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "customer_account",
                    |m: &OutputAccount| &m.customer_account,
                    |m: &mut OutputAccount| &mut m.customer_account,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "card_no",
                    |m: &OutputAccount| &m.card_no,
                    |m: &mut OutputAccount| &mut m.card_no,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "datetime",
                    |m: &OutputAccount| &m.datetime,
                    |m: &mut OutputAccount| &mut m.datetime,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "ccy",
                    |m: &OutputAccount| &m.ccy,
                    |m: &mut OutputAccount| &mut m.ccy,
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
        self.tran_date = 0;
        self.tran_time.clear();
        self.value_date = 0;
        self.tran_id.clear();
        self.tran_particular.clear();
        self.tran_remarks = 0;
        self.ref_num = 0;
        self.stan = 0;
        self.term_id = 0;
        self.debit_credit.clear();
        self.cr = 0.;
        self.dr = 0.;
        self.outstanding = 0.;
        self.customer_account = 0;
        self.card_no.clear();
        self.datetime = 0;
        self.ccy.clear();
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
    \n\x12recon_atm_a1.proto\"\x82\x04\n\rOutputAccount\x12\x1b\n\ttran_date\
    \x18\x01\x20\x01(\x03R\x08tranDate\x12\x1b\n\ttran_time\x18\x02\x20\x01(\
    \tR\x08tranTime\x12\x1d\n\nvalue_date\x18\x03\x20\x01(\x03R\tvalueDate\
    \x12\x17\n\x07tran_id\x18\x04\x20\x01(\tR\x06tranId\x12'\n\x0ftran_parti\
    cular\x18\x05\x20\x01(\tR\x0etranParticular\x12!\n\x0ctran_remarks\x18\
    \x06\x20\x01(\x03R\x0btranRemarks\x12\x17\n\x07ref_num\x18\x07\x20\x01(\
    \x03R\x06refNum\x12\x12\n\x04stan\x18\x08\x20\x01(\x03R\x04stan\x12\x17\
    \n\x07term_id\x18\t\x20\x01(\x03R\x06termId\x12!\n\x0cdebit_credit\x18\n\
    \x20\x01(\tR\x0bdebitCredit\x12\x0e\n\x02cr\x18\x0b\x20\x01(\x01R\x02cr\
    \x12\x0e\n\x02dr\x18\x0c\x20\x01(\x01R\x02dr\x12\x20\n\x0boutstanding\
    \x18\r\x20\x01(\x01R\x0boutstanding\x12)\n\x10customer_account\x18\x0e\
    \x20\x01(\x03R\x0fcustomerAccount\x12\x17\n\x07card_no\x18\x0f\x20\x01(\
    \tR\x06cardNo\x12\x1a\n\x08datetime\x18\x10\x20\x01(\x03R\x08datetime\
    \x12\x10\n\x03ccy\x18\x11\x20\x01(\tR\x03ccy\x12\x16\n\x06amount\x18\x12\
    \x20\x01(\x01R\x06amountb\x06proto3\
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