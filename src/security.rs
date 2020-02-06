#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Source Address"]
    pub srcaddr: SRCADDR,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - Length"]
    pub len: LEN,
    _reserved3: [u8; 12usize],
    #[doc = "0x30 - CRC Seed/Result"]
    pub result: RESULT,
    _reserved4: [u8; 68usize],
    #[doc = "0x78 - LOCK Control"]
    pub lockctrl: LOCKCTRL,
    #[doc = "0x7c - LOCK Status"]
    pub lockstat: LOCKSTAT,
    #[doc = "0x80 - Key0"]
    pub key0: KEY0,
    #[doc = "0x84 - Key1"]
    pub key1: KEY1,
    #[doc = "0x88 - Key2"]
    pub key2: KEY2,
    #[doc = "0x8c - Key3"]
    pub key3: KEY3,
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcaddr](srcaddr) module"]
pub type SRCADDR = crate::Reg<u32, _SRCADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCADDR;
#[doc = "`read()` method returns [srcaddr::R](srcaddr::R) reader structure"]
impl crate::Readable for SRCADDR {}
#[doc = "`write(|w| ..)` method takes [srcaddr::W](srcaddr::W) writer structure"]
impl crate::Writable for SRCADDR {}
#[doc = "Source Address"]
pub mod srcaddr;
#[doc = "Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [len](len) module"]
pub type LEN = crate::Reg<u32, _LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEN;
#[doc = "`read()` method returns [len::R](len::R) reader structure"]
impl crate::Readable for LEN {}
#[doc = "`write(|w| ..)` method takes [len::W](len::W) writer structure"]
impl crate::Writable for LEN {}
#[doc = "Length"]
pub mod len;
#[doc = "CRC Seed/Result\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](result) module"]
pub type RESULT = crate::Reg<u32, _RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT;
#[doc = "`read()` method returns [result::R](result::R) reader structure"]
impl crate::Readable for RESULT {}
#[doc = "`write(|w| ..)` method takes [result::W](result::W) writer structure"]
impl crate::Writable for RESULT {}
#[doc = "CRC Seed/Result"]
pub mod result;
#[doc = "LOCK Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockctrl](lockctrl) module"]
pub type LOCKCTRL = crate::Reg<u32, _LOCKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKCTRL;
#[doc = "`read()` method returns [lockctrl::R](lockctrl::R) reader structure"]
impl crate::Readable for LOCKCTRL {}
#[doc = "`write(|w| ..)` method takes [lockctrl::W](lockctrl::W) writer structure"]
impl crate::Writable for LOCKCTRL {}
#[doc = "LOCK Control"]
pub mod lockctrl;
#[doc = "LOCK Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockstat](lockstat) module"]
pub type LOCKSTAT = crate::Reg<u32, _LOCKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKSTAT;
#[doc = "`read()` method returns [lockstat::R](lockstat::R) reader structure"]
impl crate::Readable for LOCKSTAT {}
#[doc = "`write(|w| ..)` method takes [lockstat::W](lockstat::W) writer structure"]
impl crate::Writable for LOCKSTAT {}
#[doc = "LOCK Status"]
pub mod lockstat;
#[doc = "Key0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key0](key0) module"]
pub type KEY0 = crate::Reg<u32, _KEY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY0;
#[doc = "`read()` method returns [key0::R](key0::R) reader structure"]
impl crate::Readable for KEY0 {}
#[doc = "`write(|w| ..)` method takes [key0::W](key0::W) writer structure"]
impl crate::Writable for KEY0 {}
#[doc = "Key0"]
pub mod key0;
#[doc = "Key1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1](key1) module"]
pub type KEY1 = crate::Reg<u32, _KEY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1;
#[doc = "`read()` method returns [key1::R](key1::R) reader structure"]
impl crate::Readable for KEY1 {}
#[doc = "`write(|w| ..)` method takes [key1::W](key1::W) writer structure"]
impl crate::Writable for KEY1 {}
#[doc = "Key1"]
pub mod key1;
#[doc = "Key2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key2](key2) module"]
pub type KEY2 = crate::Reg<u32, _KEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY2;
#[doc = "`read()` method returns [key2::R](key2::R) reader structure"]
impl crate::Readable for KEY2 {}
#[doc = "`write(|w| ..)` method takes [key2::W](key2::W) writer structure"]
impl crate::Writable for KEY2 {}
#[doc = "Key2"]
pub mod key2;
#[doc = "Key3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key3](key3) module"]
pub type KEY3 = crate::Reg<u32, _KEY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY3;
#[doc = "`read()` method returns [key3::R](key3::R) reader structure"]
impl crate::Readable for KEY3 {}
#[doc = "`write(|w| ..)` method takes [key3::W](key3::W) writer structure"]
impl crate::Writable for KEY3 {}
#[doc = "Key3"]
pub mod key3;
