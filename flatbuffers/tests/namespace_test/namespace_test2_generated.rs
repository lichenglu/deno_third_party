// automatically generated by the FlatBuffers compiler, do not modify


// #include "namespace_test1_generated.rs"

pub mod NamespaceA {
  #[allow(unused_imports)]
  use std::mem;
  #[allow(unused_imports)]
  use std::marker::PhantomData;
  #[allow(unused_imports)]
  #[allow(unreachable_code)]
  extern crate flatbuffers;
  #[allow(unused_imports)]
  use self::flatbuffers::flexbuffers;
  #[allow(unused_imports)]
  use std::cmp::Ordering;

pub enum TableInFirstNSOffset {}
#[derive(Copy, Clone, PartialEq)]
pub struct TableInFirstNS<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}
impl<'a> flatbuffers::Follow<'a> for TableInFirstNS<'a> {
    type Inner = TableInFirstNS<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}
impl<'a> TableInFirstNS<'a> /* private flatbuffers::Table */ {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TableInFirstNS {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    pub const VT_FOO_TABLE: flatbuffers::VOffsetT = 4;
    pub const VT_FOO_ENUM: flatbuffers::VOffsetT = 6;
    pub const VT_FOO_STRUCT: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn foo_table(&'a self) -> Option<NamespaceB::TableInNestedNS<'a>> {
    self._tab.get::<flatbuffers::ForwardsU32Offset<NamespaceB::TableInNestedNS<'a>>>(TableInFirstNS::VT_FOO_TABLE, None)
  }
  #[inline]
  pub fn foo_enum(&'a self) -> NamespaceB::EnumInNestedNS {
    unsafe { ::std::mem::transmute(self._tab.get::<i8>(TableInFirstNS::VT_FOO_ENUM, Some(NamespaceB::EnumInNestedNS::A as i8)).unwrap()) }
  }
  #[inline]
  pub fn foo_struct(&'a self) -> Option<&'a NamespaceB::StructInNestedNS> {
    self._tab.get::<&'a NamespaceB::StructInNestedNS>(TableInFirstNS::VT_FOO_STRUCT, None)
  }
}

pub struct TableInFirstNSArgs<'a> {
    pub foo_table: Option<flatbuffers::Offset<&'a  NamespaceB::TableInNestedNS<'a >>>,
    pub foo_enum: NamespaceB::EnumInNestedNS,
    pub foo_struct: Option<&'a  NamespaceB::StructInNestedNS>,
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for TableInFirstNSArgs<'a> {
    fn default() -> Self {
        TableInFirstNSArgs {
            foo_table: None,
            foo_enum: NamespaceB::EnumInNestedNS::A,
            foo_struct: None,
            _phantom: PhantomData,
        }
    }
}
pub struct TableInFirstNSBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableOffset>,
}
impl<'a: 'b, 'b> TableInFirstNSBuilder<'a, 'b> {
  pub fn add_foo_table(&mut self, foo_table: flatbuffers::Offset<&'b  NamespaceB::TableInNestedNS<'b >>) {
    self.fbb_.push_slot_offset_relative::<&NamespaceB::TableInNestedNS>(TableInFirstNS::VT_FOO_TABLE, foo_table);
  }
  pub fn add_foo_enum(&mut self, foo_enum: NamespaceB::EnumInNestedNS) {
    self.fbb_.push_slot_scalar::<i8>(TableInFirstNS::VT_FOO_ENUM, foo_enum as i8, NamespaceB::EnumInNestedNS::A as i8);
  }
  pub fn add_foo_struct(&mut self, foo_struct: &'b  NamespaceB::StructInNestedNS) {
    self.fbb_.push_slot_struct::<NamespaceB::StructInNestedNS/* foo */>(TableInFirstNS::VT_FOO_STRUCT, foo_struct);
  }
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TableInFirstNSBuilder<'a, 'b> {
    let start = _fbb.start_table(3);
    TableInFirstNSBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  // TableInFirstNSBuilder &operator=(const TableInFirstNSBuilder &);
  //pub fn finish<'c>(mut self) -> flatbuffers::Offset<flatbuffers::TableOffset> {
  pub fn finish<'c>(mut self) -> flatbuffers::Offset<TableInFirstNS<'a>> {
    let o = self.fbb_.end_table(self.start_);
    //let o = flatbuffers::Offset::<TableInFirstNS<'a>>::new(end);
    flatbuffers::Offset::new(o.value())
  }
}

#[inline]
pub fn CreateTableInFirstNS<'a: 'b, 'b: 'c, 'c>(
    _fbb: &'c mut flatbuffers::FlatBufferBuilder<'a>,
    args: &'b TableInFirstNSArgs<'b>) -> flatbuffers::Offset<TableInFirstNS<'a>> {
  let mut builder = TableInFirstNSBuilder::new(_fbb);
  if let Some(x) = args.foo_struct { builder.add_foo_struct(x); }
  if let Some(x) = args.foo_table { builder.add_foo_table(x); }
  builder.add_foo_enum(args.foo_enum);
  builder.finish()
}

pub enum SecondTableInAOffset {}
#[derive(Copy, Clone, PartialEq)]
pub struct SecondTableInA<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}
impl<'a> flatbuffers::Follow<'a> for SecondTableInA<'a> {
    type Inner = SecondTableInA<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}
impl<'a> SecondTableInA<'a> /* private flatbuffers::Table */ {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        SecondTableInA {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    pub const VT_REFER_TO_C: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn refer_to_c(&'a self) -> Option<super::NamespaceC::TableInC<'a>> {
    self._tab.get::<flatbuffers::ForwardsU32Offset<super::NamespaceC::TableInC<'a>>>(SecondTableInA::VT_REFER_TO_C, None)
  }
}

pub struct SecondTableInAArgs<'a> {
    pub refer_to_c: Option<flatbuffers::Offset<&'a  super::NamespaceC::TableInC<'a >>>,
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for SecondTableInAArgs<'a> {
    fn default() -> Self {
        SecondTableInAArgs {
            refer_to_c: None,
            _phantom: PhantomData,
        }
    }
}
pub struct SecondTableInABuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableOffset>,
}
impl<'a: 'b, 'b> SecondTableInABuilder<'a, 'b> {
  pub fn add_refer_to_c(&mut self, refer_to_c: flatbuffers::Offset<&'b  super::NamespaceC::TableInC<'b >>) {
    self.fbb_.push_slot_offset_relative::<&super::NamespaceC::TableInC>(SecondTableInA::VT_REFER_TO_C, refer_to_c);
  }
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SecondTableInABuilder<'a, 'b> {
    let start = _fbb.start_table(1);
    SecondTableInABuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  // SecondTableInABuilder &operator=(const SecondTableInABuilder &);
  //pub fn finish<'c>(mut self) -> flatbuffers::Offset<flatbuffers::TableOffset> {
  pub fn finish<'c>(mut self) -> flatbuffers::Offset<SecondTableInA<'a>> {
    let o = self.fbb_.end_table(self.start_);
    //let o = flatbuffers::Offset::<SecondTableInA<'a>>::new(end);
    flatbuffers::Offset::new(o.value())
  }
}

#[inline]
pub fn CreateSecondTableInA<'a: 'b, 'b: 'c, 'c>(
    _fbb: &'c mut flatbuffers::FlatBufferBuilder<'a>,
    args: &'b SecondTableInAArgs<'b>) -> flatbuffers::Offset<SecondTableInA<'a>> {
  let mut builder = SecondTableInABuilder::new(_fbb);
  if let Some(x) = args.refer_to_c { builder.add_refer_to_c(x); }
  builder.finish()
}

}  // pub mod NamespaceA

pub mod NamespaceC {
  #[allow(unused_imports)]
  use std::mem;
  #[allow(unused_imports)]
  use std::marker::PhantomData;
  #[allow(unused_imports)]
  #[allow(unreachable_code)]
  extern crate flatbuffers;
  #[allow(unused_imports)]
  use self::flatbuffers::flexbuffers;
  #[allow(unused_imports)]
  use std::cmp::Ordering;

pub enum TableInCOffset {}
#[derive(Copy, Clone, PartialEq)]
pub struct TableInC<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}
impl<'a> flatbuffers::Follow<'a> for TableInC<'a> {
    type Inner = TableInC<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}
impl<'a> TableInC<'a> /* private flatbuffers::Table */ {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TableInC {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    pub const VT_REFER_TO_A1: flatbuffers::VOffsetT = 4;
    pub const VT_REFER_TO_A2: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn refer_to_a1(&'a self) -> Option<super::NamespaceA::TableInFirstNS<'a>> {
    self._tab.get::<flatbuffers::ForwardsU32Offset<super::NamespaceA::TableInFirstNS<'a>>>(TableInC::VT_REFER_TO_A1, None)
  }
  #[inline]
  pub fn refer_to_a2(&'a self) -> Option<super::NamespaceA::SecondTableInA<'a>> {
    self._tab.get::<flatbuffers::ForwardsU32Offset<super::NamespaceA::SecondTableInA<'a>>>(TableInC::VT_REFER_TO_A2, None)
  }
}

pub struct TableInCArgs<'a> {
    pub refer_to_a1: Option<flatbuffers::Offset<&'a  super::NamespaceA::TableInFirstNS<'a >>>,
    pub refer_to_a2: Option<flatbuffers::Offset<&'a  super::NamespaceA::SecondTableInA<'a >>>,
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for TableInCArgs<'a> {
    fn default() -> Self {
        TableInCArgs {
            refer_to_a1: None,
            refer_to_a2: None,
            _phantom: PhantomData,
        }
    }
}
pub struct TableInCBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableOffset>,
}
impl<'a: 'b, 'b> TableInCBuilder<'a, 'b> {
  pub fn add_refer_to_a1(&mut self, refer_to_a1: flatbuffers::Offset<&'b  super::NamespaceA::TableInFirstNS<'b >>) {
    self.fbb_.push_slot_offset_relative::<&super::NamespaceA::TableInFirstNS>(TableInC::VT_REFER_TO_A1, refer_to_a1);
  }
  pub fn add_refer_to_a2(&mut self, refer_to_a2: flatbuffers::Offset<&'b  super::NamespaceA::SecondTableInA<'b >>) {
    self.fbb_.push_slot_offset_relative::<&super::NamespaceA::SecondTableInA>(TableInC::VT_REFER_TO_A2, refer_to_a2);
  }
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TableInCBuilder<'a, 'b> {
    let start = _fbb.start_table(2);
    TableInCBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  // TableInCBuilder &operator=(const TableInCBuilder &);
  //pub fn finish<'c>(mut self) -> flatbuffers::Offset<flatbuffers::TableOffset> {
  pub fn finish<'c>(mut self) -> flatbuffers::Offset<TableInC<'a>> {
    let o = self.fbb_.end_table(self.start_);
    //let o = flatbuffers::Offset::<TableInC<'a>>::new(end);
    flatbuffers::Offset::new(o.value())
  }
}

#[inline]
pub fn CreateTableInC<'a: 'b, 'b: 'c, 'c>(
    _fbb: &'c mut flatbuffers::FlatBufferBuilder<'a>,
    args: &'b TableInCArgs<'b>) -> flatbuffers::Offset<TableInC<'a>> {
  let mut builder = TableInCBuilder::new(_fbb);
  if let Some(x) = args.refer_to_a2 { builder.add_refer_to_a2(x); }
  if let Some(x) = args.refer_to_a1 { builder.add_refer_to_a1(x); }
  builder.finish()
}

}  // pub mod NamespaceC

