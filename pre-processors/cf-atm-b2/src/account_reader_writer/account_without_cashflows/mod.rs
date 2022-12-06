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
//! Generated file from `recon_atm_b2.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_11_0;

#[derive(PartialEq, Clone, Default)]
pub struct OutputAccount {
    // message fields
    pub gl_date: i64,
    pub tran_id: ::std::string::String,
    pub ref_num: ::std::string::String,
    pub particulars: ::std::string::String,
    pub tran_dr_amt: f64,
    pub tran_cr_amt: f64,
    pub ccy_code: ::std::string::String,
    pub balance: f64,
    pub remarks: ::std::string::String,
    pub branch: ::std::string::String,
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

    // int64 gl_date = 1;

    pub fn get_gl_date(&self) -> i64 {
        self.gl_date
    }
    pub fn clear_gl_date(&mut self) {
        self.gl_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_gl_date(&mut self, v: i64) {
        self.gl_date = v;
    }

    // string tran_id = 2;

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

    // string ref_num = 3;

    pub fn get_ref_num(&self) -> &str {
        &self.ref_num
    }
    pub fn clear_ref_num(&mut self) {
        self.ref_num.clear();
    }

    // Param is passed by value, moved
    pub fn set_ref_num(&mut self, v: ::std::string::String) {
        self.ref_num = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ref_num(&mut self) -> &mut ::std::string::String {
        &mut self.ref_num
    }

    // Take field
    pub fn take_ref_num(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ref_num, ::std::string::String::new())
    }

    // string particulars = 4;

    pub fn get_particulars(&self) -> &str {
        &self.particulars
    }
    pub fn clear_particulars(&mut self) {
        self.particulars.clear();
    }

    // Param is passed by value, moved
    pub fn set_particulars(&mut self, v: ::std::string::String) {
        self.particulars = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_particulars(&mut self) -> &mut ::std::string::String {
        &mut self.particulars
    }

    // Take field
    pub fn take_particulars(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.particulars, ::std::string::String::new())
    }

    // double tran_dr_amt = 5;

    pub fn get_tran_dr_amt(&self) -> f64 {
        self.tran_dr_amt
    }
    pub fn clear_tran_dr_amt(&mut self) {
        self.tran_dr_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_tran_dr_amt(&mut self, v: f64) {
        self.tran_dr_amt = v;
    }

    // double tran_cr_amt = 6;

    pub fn get_tran_cr_amt(&self) -> f64 {
        self.tran_cr_amt
    }
    pub fn clear_tran_cr_amt(&mut self) {
        self.tran_cr_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_tran_cr_amt(&mut self, v: f64) {
        self.tran_cr_amt = v;
    }

    // string ccy_code = 7;

    pub fn get_ccy_code(&self) -> &str {
        &self.ccy_code
    }
    pub fn clear_ccy_code(&mut self) {
        self.ccy_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_ccy_code(&mut self, v: ::std::string::String) {
        self.ccy_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ccy_code(&mut self) -> &mut ::std::string::String {
        &mut self.ccy_code
    }

    // Take field
    pub fn take_ccy_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ccy_code, ::std::string::String::new())
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

    // string remarks = 9;

    pub fn get_remarks(&self) -> &str {
        &self.remarks
    }
    pub fn clear_remarks(&mut self) {
        self.remarks.clear();
    }

    // Param is passed by value, moved
    pub fn set_remarks(&mut self, v: ::std::string::String) {
        self.remarks = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_remarks(&mut self) -> &mut ::std::string::String {
        &mut self.remarks
    }

    // Take field
    pub fn take_remarks(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.remarks, ::std::string::String::new())
    }

    // string branch = 10;

    pub fn get_branch(&self) -> &str {
        &self.branch
    }
    pub fn clear_branch(&mut self) {
        self.branch.clear();
    }

    // Param is passed by value, moved
    pub fn set_branch(&mut self, v: ::std::string::String) {
        self.branch = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_branch(&mut self) -> &mut ::std::string::String {
        &mut self.branch
    }

    // Take field
    pub fn take_branch(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.branch, ::std::string::String::new())
    }

    // double amount = 11;

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
                    self.gl_date = tmp;
                }
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.tran_id,
                    )?;
                }
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.ref_num,
                    )?;
                }
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.particulars,
                    )?;
                }
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_double()?;
                    self.tran_dr_amt = tmp;
                }
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_double()?;
                    self.tran_cr_amt = tmp;
                }
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.ccy_code,
                    )?;
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
                        &mut self.remarks,
                    )?;
                }
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.branch,
                    )?;
                }
                11 => {
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
        if self.gl_date != 0 {
            my_size += ::protobuf::rt::value_size(
                1,
                self.gl_date,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if !self.tran_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.tran_id);
        }
        if !self.ref_num.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.ref_num);
        }
        if !self.particulars.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.particulars);
        }
        if self.tran_dr_amt != 0. {
            my_size += 9;
        }
        if self.tran_cr_amt != 0. {
            my_size += 9;
        }
        if !self.ccy_code.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.ccy_code);
        }
        if self.balance != 0. {
            my_size += 9;
        }
        if !self.remarks.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.remarks);
        }
        if !self.branch.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.branch);
        }
        if self.amount != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(
        &self,
        os: &mut ::protobuf::CodedOutputStream<'_>,
    ) -> ::protobuf::ProtobufResult<()> {
        if self.gl_date != 0 {
            os.write_int64(1, self.gl_date)?;
        }
        if !self.tran_id.is_empty() {
            os.write_string(2, &self.tran_id)?;
        }
        if !self.ref_num.is_empty() {
            os.write_string(3, &self.ref_num)?;
        }
        if !self.particulars.is_empty() {
            os.write_string(4, &self.particulars)?;
        }
        if self.tran_dr_amt != 0. {
            os.write_double(5, self.tran_dr_amt)?;
        }
        if self.tran_cr_amt != 0. {
            os.write_double(6, self.tran_cr_amt)?;
        }
        if !self.ccy_code.is_empty() {
            os.write_string(7, &self.ccy_code)?;
        }
        if self.balance != 0. {
            os.write_double(8, self.balance)?;
        }
        if !self.remarks.is_empty() {
            os.write_string(9, &self.remarks)?;
        }
        if !self.branch.is_empty() {
            os.write_string(10, &self.branch)?;
        }
        if self.amount != 0. {
            os.write_double(11, self.amount)?;
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
                    "gl_date",
                    |m: &OutputAccount| &m.gl_date,
                    |m: &mut OutputAccount| &mut m.gl_date,
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
                    "ref_num",
                    |m: &OutputAccount| &m.ref_num,
                    |m: &mut OutputAccount| &mut m.ref_num,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "particulars",
                    |m: &OutputAccount| &m.particulars,
                    |m: &mut OutputAccount| &mut m.particulars,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeDouble,
                >(
                    "tran_dr_amt",
                    |m: &OutputAccount| &m.tran_dr_amt,
                    |m: &mut OutputAccount| &mut m.tran_dr_amt,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeDouble,
                >(
                    "tran_cr_amt",
                    |m: &OutputAccount| &m.tran_cr_amt,
                    |m: &mut OutputAccount| &mut m.tran_cr_amt,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "ccy_code",
                    |m: &OutputAccount| &m.ccy_code,
                    |m: &mut OutputAccount| &mut m.ccy_code,
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
                    "remarks",
                    |m: &OutputAccount| &m.remarks,
                    |m: &mut OutputAccount| &mut m.remarks,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "branch",
                    |m: &OutputAccount| &m.branch,
                    |m: &mut OutputAccount| &mut m.branch,
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
        self.gl_date = 0;
        self.tran_id.clear();
        self.ref_num.clear();
        self.particulars.clear();
        self.tran_dr_amt = 0.;
        self.tran_cr_amt = 0.;
        self.ccy_code.clear();
        self.balance = 0.;
        self.remarks.clear();
        self.branch.clear();
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
    \n\x12recon_atm_b2.proto\"\xbb\x02\n\rOutputAccount\x12\x17\n\x07gl_date\
    \x18\x01\x20\x01(\x03R\x06glDate\x12\x17\n\x07tran_id\x18\x02\x20\x01(\t\
    R\x06tranId\x12\x17\n\x07ref_num\x18\x03\x20\x01(\tR\x06refNum\x12\x20\n\
    \x0bparticulars\x18\x04\x20\x01(\tR\x0bparticulars\x12\x1e\n\x0btran_dr_\
    amt\x18\x05\x20\x01(\x01R\ttranDrAmt\x12\x1e\n\x0btran_cr_amt\x18\x06\
    \x20\x01(\x01R\ttranCrAmt\x12\x19\n\x08ccy_code\x18\x07\x20\x01(\tR\x07c\
    cyCode\x12\x18\n\x07balance\x18\x08\x20\x01(\x01R\x07balance\x12\x18\n\
    \x07remarks\x18\t\x20\x01(\tR\x07remarks\x12\x16\n\x06branch\x18\n\x20\
    \x01(\tR\x06branch\x12\x16\n\x06amount\x18\x0b\x20\x01(\x01R\x06amountb\
    \x06proto3\
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