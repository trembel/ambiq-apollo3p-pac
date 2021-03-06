#[doc = "Reader of register PADREGF"]
pub type R = crate::R<u32, super::PADREGF>;
#[doc = "Writer for register PADREGF"]
pub type W = crate::W<u32, super::PADREGF>;
#[doc = "Register PADREGF `reset()`'s with value 0x1818_0202"]
impl crate::ResetValue for super::PADREGF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_0202
    }
}
#[doc = "Pad 23 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD23FNCSEL_A {
    #[doc = "0: Configure as the UART0 RX signal"]
    UART0RX = 0,
    #[doc = "1: IOM/MSPI nCE group 23"]
    NCE23 = 1,
    #[doc = "2: CTIMER connection 14"]
    CT14 = 2,
    #[doc = "3: Configure as GPIO23"]
    GPIO23 = 3,
    #[doc = "4: I2S word clock input"]
    I2SWCLK = 4,
    #[doc = "5: Configure as voltage comparator output"]
    CMPOUT = 5,
    #[doc = "6: MSPI data connection 3"]
    MSPI3 = 6,
    #[doc = "7: External XTAL oscillator input"]
    EXTXT = 7,
}
impl From<PAD23FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD23FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD23FNCSEL`"]
pub type PAD23FNCSEL_R = crate::R<u8, PAD23FNCSEL_A>;
impl PAD23FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD23FNCSEL_A {
        match self.bits {
            0 => PAD23FNCSEL_A::UART0RX,
            1 => PAD23FNCSEL_A::NCE23,
            2 => PAD23FNCSEL_A::CT14,
            3 => PAD23FNCSEL_A::GPIO23,
            4 => PAD23FNCSEL_A::I2SWCLK,
            5 => PAD23FNCSEL_A::CMPOUT,
            6 => PAD23FNCSEL_A::MSPI3,
            7 => PAD23FNCSEL_A::EXTXT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD23FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `NCE23`"]
    #[inline(always)]
    pub fn is_nce23(&self) -> bool {
        *self == PAD23FNCSEL_A::NCE23
    }
    #[doc = "Checks if the value of the field is `CT14`"]
    #[inline(always)]
    pub fn is_ct14(&self) -> bool {
        *self == PAD23FNCSEL_A::CT14
    }
    #[doc = "Checks if the value of the field is `GPIO23`"]
    #[inline(always)]
    pub fn is_gpio23(&self) -> bool {
        *self == PAD23FNCSEL_A::GPIO23
    }
    #[doc = "Checks if the value of the field is `I2SWCLK`"]
    #[inline(always)]
    pub fn is_i2swclk(&self) -> bool {
        *self == PAD23FNCSEL_A::I2SWCLK
    }
    #[doc = "Checks if the value of the field is `CMPOUT`"]
    #[inline(always)]
    pub fn is_cmpout(&self) -> bool {
        *self == PAD23FNCSEL_A::CMPOUT
    }
    #[doc = "Checks if the value of the field is `MSPI3`"]
    #[inline(always)]
    pub fn is_mspi3(&self) -> bool {
        *self == PAD23FNCSEL_A::MSPI3
    }
    #[doc = "Checks if the value of the field is `EXTXT`"]
    #[inline(always)]
    pub fn is_extxt(&self) -> bool {
        *self == PAD23FNCSEL_A::EXTXT
    }
}
#[doc = "Write proxy for field `PAD23FNCSEL`"]
pub struct PAD23FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD23FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD23FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the UART0 RX signal"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD23FNCSEL_A::UART0RX)
    }
    #[doc = "IOM/MSPI nCE group 23"]
    #[inline(always)]
    pub fn nce23(self) -> &'a mut W {
        self.variant(PAD23FNCSEL_A::NCE23)
    }
    #[doc = "CTIMER connection 14"]
    #[inline(always)]
    pub fn ct14(self) -> &'a mut W {
        self.variant(PAD23FNCSEL_A::CT14)
    }
    #[doc = "Configure as GPIO23"]
    #[inline(always)]
    pub fn gpio23(self) -> &'a mut W {
        self.variant(PAD23FNCSEL_A::GPIO23)
    }
    #[doc = "I2S word clock input"]
    #[inline(always)]
    pub fn i2swclk(self) -> &'a mut W {
        self.variant(PAD23FNCSEL_A::I2SWCLK)
    }
    #[doc = "Configure as voltage comparator output"]
    #[inline(always)]
    pub fn cmpout(self) -> &'a mut W {
        self.variant(PAD23FNCSEL_A::CMPOUT)
    }
    #[doc = "MSPI data connection 3"]
    #[inline(always)]
    pub fn mspi3(self) -> &'a mut W {
        self.variant(PAD23FNCSEL_A::MSPI3)
    }
    #[doc = "External XTAL oscillator input"]
    #[inline(always)]
    pub fn extxt(self) -> &'a mut W {
        self.variant(PAD23FNCSEL_A::EXTXT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 23 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD23STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD23STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD23STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD23STRNG`"]
pub type PAD23STRNG_R = crate::R<bool, PAD23STRNG_A>;
impl PAD23STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD23STRNG_A {
        match self.bits {
            false => PAD23STRNG_A::LOW,
            true => PAD23STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD23STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD23STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD23STRNG`"]
pub struct PAD23STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD23STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD23STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD23STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD23STRNG_A::HIGH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Pad 23 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD23INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD23INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD23INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD23INPEN`"]
pub type PAD23INPEN_R = crate::R<bool, PAD23INPEN_A>;
impl PAD23INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD23INPEN_A {
        match self.bits {
            false => PAD23INPEN_A::DIS,
            true => PAD23INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD23INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD23INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD23INPEN`"]
pub struct PAD23INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD23INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD23INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD23INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD23INPEN_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Pad 23 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD23PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD23PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD23PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD23PULL`"]
pub type PAD23PULL_R = crate::R<bool, PAD23PULL_A>;
impl PAD23PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD23PULL_A {
        match self.bits {
            false => PAD23PULL_A::DIS,
            true => PAD23PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD23PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD23PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD23PULL`"]
pub struct PAD23PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD23PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD23PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD23PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD23PULL_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Pad 22 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD22FNCSEL_A {
    #[doc = "0: Configure as the UART0 TX signal"]
    UART0TX = 0,
    #[doc = "1: IOM/MSPI nCE group 22"]
    NCE22 = 1,
    #[doc = "2: CTIMER connection 12"]
    CT12 = 2,
    #[doc = "3: Configure as GPIO22"]
    GPIO22 = 3,
    #[doc = "4: Configure as the PDM CLK output"]
    PDM_CLK = 4,
    #[doc = "5: External LFRC input"]
    EXTLF = 5,
    #[doc = "6: MSPI data connection 0"]
    MSPI0 = 6,
    #[doc = "7: Configure as the serial trace data output signal"]
    SWO = 7,
}
impl From<PAD22FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD22FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD22FNCSEL`"]
pub type PAD22FNCSEL_R = crate::R<u8, PAD22FNCSEL_A>;
impl PAD22FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD22FNCSEL_A {
        match self.bits {
            0 => PAD22FNCSEL_A::UART0TX,
            1 => PAD22FNCSEL_A::NCE22,
            2 => PAD22FNCSEL_A::CT12,
            3 => PAD22FNCSEL_A::GPIO22,
            4 => PAD22FNCSEL_A::PDM_CLK,
            5 => PAD22FNCSEL_A::EXTLF,
            6 => PAD22FNCSEL_A::MSPI0,
            7 => PAD22FNCSEL_A::SWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD22FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `NCE22`"]
    #[inline(always)]
    pub fn is_nce22(&self) -> bool {
        *self == PAD22FNCSEL_A::NCE22
    }
    #[doc = "Checks if the value of the field is `CT12`"]
    #[inline(always)]
    pub fn is_ct12(&self) -> bool {
        *self == PAD22FNCSEL_A::CT12
    }
    #[doc = "Checks if the value of the field is `GPIO22`"]
    #[inline(always)]
    pub fn is_gpio22(&self) -> bool {
        *self == PAD22FNCSEL_A::GPIO22
    }
    #[doc = "Checks if the value of the field is `PDM_CLK`"]
    #[inline(always)]
    pub fn is_pdm_clk(&self) -> bool {
        *self == PAD22FNCSEL_A::PDM_CLK
    }
    #[doc = "Checks if the value of the field is `EXTLF`"]
    #[inline(always)]
    pub fn is_extlf(&self) -> bool {
        *self == PAD22FNCSEL_A::EXTLF
    }
    #[doc = "Checks if the value of the field is `MSPI0`"]
    #[inline(always)]
    pub fn is_mspi0(&self) -> bool {
        *self == PAD22FNCSEL_A::MSPI0
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD22FNCSEL_A::SWO
    }
}
#[doc = "Write proxy for field `PAD22FNCSEL`"]
pub struct PAD22FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD22FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD22FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the UART0 TX signal"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD22FNCSEL_A::UART0TX)
    }
    #[doc = "IOM/MSPI nCE group 22"]
    #[inline(always)]
    pub fn nce22(self) -> &'a mut W {
        self.variant(PAD22FNCSEL_A::NCE22)
    }
    #[doc = "CTIMER connection 12"]
    #[inline(always)]
    pub fn ct12(self) -> &'a mut W {
        self.variant(PAD22FNCSEL_A::CT12)
    }
    #[doc = "Configure as GPIO22"]
    #[inline(always)]
    pub fn gpio22(self) -> &'a mut W {
        self.variant(PAD22FNCSEL_A::GPIO22)
    }
    #[doc = "Configure as the PDM CLK output"]
    #[inline(always)]
    pub fn pdm_clk(self) -> &'a mut W {
        self.variant(PAD22FNCSEL_A::PDM_CLK)
    }
    #[doc = "External LFRC input"]
    #[inline(always)]
    pub fn extlf(self) -> &'a mut W {
        self.variant(PAD22FNCSEL_A::EXTLF)
    }
    #[doc = "MSPI data connection 0"]
    #[inline(always)]
    pub fn mspi0(self) -> &'a mut W {
        self.variant(PAD22FNCSEL_A::MSPI0)
    }
    #[doc = "Configure as the serial trace data output signal"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD22FNCSEL_A::SWO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 22 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD22STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD22STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD22STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD22STRNG`"]
pub type PAD22STRNG_R = crate::R<bool, PAD22STRNG_A>;
impl PAD22STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD22STRNG_A {
        match self.bits {
            false => PAD22STRNG_A::LOW,
            true => PAD22STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD22STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD22STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD22STRNG`"]
pub struct PAD22STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD22STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD22STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD22STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD22STRNG_A::HIGH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Pad 22 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD22INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD22INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD22INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD22INPEN`"]
pub type PAD22INPEN_R = crate::R<bool, PAD22INPEN_A>;
impl PAD22INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD22INPEN_A {
        match self.bits {
            false => PAD22INPEN_A::DIS,
            true => PAD22INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD22INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD22INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD22INPEN`"]
pub struct PAD22INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD22INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD22INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD22INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD22INPEN_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Pad 22 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD22PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD22PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD22PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD22PULL`"]
pub type PAD22PULL_R = crate::R<bool, PAD22PULL_A>;
impl PAD22PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD22PULL_A {
        match self.bits {
            false => PAD22PULL_A::DIS,
            true => PAD22PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD22PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD22PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD22PULL`"]
pub struct PAD22PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD22PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD22PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD22PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD22PULL_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Pad 21 function select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD21FNCSEL_A {
    #[doc = "0: Configure as the serial wire debug data signal"]
    SWDIO = 0,
    #[doc = "1: IOM/MSPI nCE group 21"]
    NCE21 = 1,
    #[doc = "3: Configure as GPIO21"]
    GPIO21 = 3,
    #[doc = "4: Configure as UART0 RX input signal"]
    UART0RX = 4,
    #[doc = "5: Configure as UART1 RX input signal"]
    UART1RX = 5,
    #[doc = "6: I2S byte clock input"]
    I2SBCLK = 6,
    #[doc = "7: Configure as UART1 CTS input signal"]
    UA1CTS = 7,
}
impl From<PAD21FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD21FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD21FNCSEL`"]
pub type PAD21FNCSEL_R = crate::R<u8, PAD21FNCSEL_A>;
impl PAD21FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD21FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD21FNCSEL_A::SWDIO),
            1 => Val(PAD21FNCSEL_A::NCE21),
            3 => Val(PAD21FNCSEL_A::GPIO21),
            4 => Val(PAD21FNCSEL_A::UART0RX),
            5 => Val(PAD21FNCSEL_A::UART1RX),
            6 => Val(PAD21FNCSEL_A::I2SBCLK),
            7 => Val(PAD21FNCSEL_A::UA1CTS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWDIO`"]
    #[inline(always)]
    pub fn is_swdio(&self) -> bool {
        *self == PAD21FNCSEL_A::SWDIO
    }
    #[doc = "Checks if the value of the field is `NCE21`"]
    #[inline(always)]
    pub fn is_nce21(&self) -> bool {
        *self == PAD21FNCSEL_A::NCE21
    }
    #[doc = "Checks if the value of the field is `GPIO21`"]
    #[inline(always)]
    pub fn is_gpio21(&self) -> bool {
        *self == PAD21FNCSEL_A::GPIO21
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD21FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD21FNCSEL_A::UART1RX
    }
    #[doc = "Checks if the value of the field is `I2SBCLK`"]
    #[inline(always)]
    pub fn is_i2sbclk(&self) -> bool {
        *self == PAD21FNCSEL_A::I2SBCLK
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline(always)]
    pub fn is_ua1cts(&self) -> bool {
        *self == PAD21FNCSEL_A::UA1CTS
    }
}
#[doc = "Write proxy for field `PAD21FNCSEL`"]
pub struct PAD21FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD21FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD21FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the serial wire debug data signal"]
    #[inline(always)]
    pub fn swdio(self) -> &'a mut W {
        self.variant(PAD21FNCSEL_A::SWDIO)
    }
    #[doc = "IOM/MSPI nCE group 21"]
    #[inline(always)]
    pub fn nce21(self) -> &'a mut W {
        self.variant(PAD21FNCSEL_A::NCE21)
    }
    #[doc = "Configure as GPIO21"]
    #[inline(always)]
    pub fn gpio21(self) -> &'a mut W {
        self.variant(PAD21FNCSEL_A::GPIO21)
    }
    #[doc = "Configure as UART0 RX input signal"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD21FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as UART1 RX input signal"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD21FNCSEL_A::UART1RX)
    }
    #[doc = "I2S byte clock input"]
    #[inline(always)]
    pub fn i2sbclk(self) -> &'a mut W {
        self.variant(PAD21FNCSEL_A::I2SBCLK)
    }
    #[doc = "Configure as UART1 CTS input signal"]
    #[inline(always)]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD21FNCSEL_A::UA1CTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 21 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD21STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD21STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD21STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD21STRNG`"]
pub type PAD21STRNG_R = crate::R<bool, PAD21STRNG_A>;
impl PAD21STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD21STRNG_A {
        match self.bits {
            false => PAD21STRNG_A::LOW,
            true => PAD21STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD21STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD21STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD21STRNG`"]
pub struct PAD21STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD21STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD21STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD21STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD21STRNG_A::HIGH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Pad 21 input enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD21INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD21INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD21INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD21INPEN`"]
pub type PAD21INPEN_R = crate::R<bool, PAD21INPEN_A>;
impl PAD21INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD21INPEN_A {
        match self.bits {
            false => PAD21INPEN_A::DIS,
            true => PAD21INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD21INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD21INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD21INPEN`"]
pub struct PAD21INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD21INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD21INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD21INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD21INPEN_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Pad 21 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD21PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD21PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD21PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD21PULL`"]
pub type PAD21PULL_R = crate::R<bool, PAD21PULL_A>;
impl PAD21PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD21PULL_A {
        match self.bits {
            false => PAD21PULL_A::DIS,
            true => PAD21PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD21PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD21PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD21PULL`"]
pub struct PAD21PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD21PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD21PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD21PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD21PULL_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Pad 20 function select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD20FNCSEL_A {
    #[doc = "0: Configure as the serial wire debug clock signal"]
    SWDCK = 0,
    #[doc = "1: IOM/MSPI nCE group 20"]
    NCE20 = 1,
    #[doc = "3: Configure as GPIO20"]
    GPIO20 = 3,
    #[doc = "4: Configure as UART0 TX output signal"]
    UART0TX = 4,
    #[doc = "5: Configure as UART1 TX output signal"]
    UART1TX = 5,
    #[doc = "6: I2S byte clock input"]
    I2SBCLK = 6,
    #[doc = "7: Configure as UART1 RTS output signal"]
    UA1RTS = 7,
}
impl From<PAD20FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD20FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD20FNCSEL`"]
pub type PAD20FNCSEL_R = crate::R<u8, PAD20FNCSEL_A>;
impl PAD20FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD20FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD20FNCSEL_A::SWDCK),
            1 => Val(PAD20FNCSEL_A::NCE20),
            3 => Val(PAD20FNCSEL_A::GPIO20),
            4 => Val(PAD20FNCSEL_A::UART0TX),
            5 => Val(PAD20FNCSEL_A::UART1TX),
            6 => Val(PAD20FNCSEL_A::I2SBCLK),
            7 => Val(PAD20FNCSEL_A::UA1RTS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWDCK`"]
    #[inline(always)]
    pub fn is_swdck(&self) -> bool {
        *self == PAD20FNCSEL_A::SWDCK
    }
    #[doc = "Checks if the value of the field is `NCE20`"]
    #[inline(always)]
    pub fn is_nce20(&self) -> bool {
        *self == PAD20FNCSEL_A::NCE20
    }
    #[doc = "Checks if the value of the field is `GPIO20`"]
    #[inline(always)]
    pub fn is_gpio20(&self) -> bool {
        *self == PAD20FNCSEL_A::GPIO20
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD20FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD20FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `I2SBCLK`"]
    #[inline(always)]
    pub fn is_i2sbclk(&self) -> bool {
        *self == PAD20FNCSEL_A::I2SBCLK
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline(always)]
    pub fn is_ua1rts(&self) -> bool {
        *self == PAD20FNCSEL_A::UA1RTS
    }
}
#[doc = "Write proxy for field `PAD20FNCSEL`"]
pub struct PAD20FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD20FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD20FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the serial wire debug clock signal"]
    #[inline(always)]
    pub fn swdck(self) -> &'a mut W {
        self.variant(PAD20FNCSEL_A::SWDCK)
    }
    #[doc = "IOM/MSPI nCE group 20"]
    #[inline(always)]
    pub fn nce20(self) -> &'a mut W {
        self.variant(PAD20FNCSEL_A::NCE20)
    }
    #[doc = "Configure as GPIO20"]
    #[inline(always)]
    pub fn gpio20(self) -> &'a mut W {
        self.variant(PAD20FNCSEL_A::GPIO20)
    }
    #[doc = "Configure as UART0 TX output signal"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD20FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as UART1 TX output signal"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD20FNCSEL_A::UART1TX)
    }
    #[doc = "I2S byte clock input"]
    #[inline(always)]
    pub fn i2sbclk(self) -> &'a mut W {
        self.variant(PAD20FNCSEL_A::I2SBCLK)
    }
    #[doc = "Configure as UART1 RTS output signal"]
    #[inline(always)]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD20FNCSEL_A::UA1RTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 20 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD20STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD20STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD20STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD20STRNG`"]
pub type PAD20STRNG_R = crate::R<bool, PAD20STRNG_A>;
impl PAD20STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD20STRNG_A {
        match self.bits {
            false => PAD20STRNG_A::LOW,
            true => PAD20STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD20STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD20STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD20STRNG`"]
pub struct PAD20STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD20STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD20STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD20STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD20STRNG_A::HIGH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Pad 20 input enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD20INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD20INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD20INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD20INPEN`"]
pub type PAD20INPEN_R = crate::R<bool, PAD20INPEN_A>;
impl PAD20INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD20INPEN_A {
        match self.bits {
            false => PAD20INPEN_A::DIS,
            true => PAD20INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD20INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD20INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD20INPEN`"]
pub struct PAD20INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD20INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD20INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD20INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD20INPEN_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Pad 20 pulldown enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD20PULL_A {
    #[doc = "0: Pulldown disabled"]
    DIS = 0,
    #[doc = "1: Pulldown enabled"]
    EN = 1,
}
impl From<PAD20PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD20PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD20PULL`"]
pub type PAD20PULL_R = crate::R<bool, PAD20PULL_A>;
impl PAD20PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD20PULL_A {
        match self.bits {
            false => PAD20PULL_A::DIS,
            true => PAD20PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD20PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD20PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD20PULL`"]
pub struct PAD20PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD20PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD20PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pulldown disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD20PULL_A::DIS)
    }
    #[doc = "Pulldown enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD20PULL_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:29 - Pad 23 function select"]
    #[inline(always)]
    pub fn pad23fncsel(&self) -> PAD23FNCSEL_R {
        PAD23FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 23 drive strength"]
    #[inline(always)]
    pub fn pad23strng(&self) -> PAD23STRNG_R {
        PAD23STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 23 input enable"]
    #[inline(always)]
    pub fn pad23inpen(&self) -> PAD23INPEN_R {
        PAD23INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 23 pullup enable"]
    #[inline(always)]
    pub fn pad23pull(&self) -> PAD23PULL_R {
        PAD23PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 22 function select"]
    #[inline(always)]
    pub fn pad22fncsel(&self) -> PAD22FNCSEL_R {
        PAD22FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 22 drive strength"]
    #[inline(always)]
    pub fn pad22strng(&self) -> PAD22STRNG_R {
        PAD22STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 22 input enable"]
    #[inline(always)]
    pub fn pad22inpen(&self) -> PAD22INPEN_R {
        PAD22INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 22 pullup enable"]
    #[inline(always)]
    pub fn pad22pull(&self) -> PAD22PULL_R {
        PAD22PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 21 function select"]
    #[inline(always)]
    pub fn pad21fncsel(&self) -> PAD21FNCSEL_R {
        PAD21FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 21 drive strength"]
    #[inline(always)]
    pub fn pad21strng(&self) -> PAD21STRNG_R {
        PAD21STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 21 input enable"]
    #[inline(always)]
    pub fn pad21inpen(&self) -> PAD21INPEN_R {
        PAD21INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 21 pullup enable"]
    #[inline(always)]
    pub fn pad21pull(&self) -> PAD21PULL_R {
        PAD21PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 20 function select"]
    #[inline(always)]
    pub fn pad20fncsel(&self) -> PAD20FNCSEL_R {
        PAD20FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 20 drive strength"]
    #[inline(always)]
    pub fn pad20strng(&self) -> PAD20STRNG_R {
        PAD20STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 20 input enable"]
    #[inline(always)]
    pub fn pad20inpen(&self) -> PAD20INPEN_R {
        PAD20INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 20 pulldown enable"]
    #[inline(always)]
    pub fn pad20pull(&self) -> PAD20PULL_R {
        PAD20PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 23 function select"]
    #[inline(always)]
    pub fn pad23fncsel(&mut self) -> PAD23FNCSEL_W {
        PAD23FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 23 drive strength"]
    #[inline(always)]
    pub fn pad23strng(&mut self) -> PAD23STRNG_W {
        PAD23STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 23 input enable"]
    #[inline(always)]
    pub fn pad23inpen(&mut self) -> PAD23INPEN_W {
        PAD23INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 23 pullup enable"]
    #[inline(always)]
    pub fn pad23pull(&mut self) -> PAD23PULL_W {
        PAD23PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 22 function select"]
    #[inline(always)]
    pub fn pad22fncsel(&mut self) -> PAD22FNCSEL_W {
        PAD22FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 22 drive strength"]
    #[inline(always)]
    pub fn pad22strng(&mut self) -> PAD22STRNG_W {
        PAD22STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 22 input enable"]
    #[inline(always)]
    pub fn pad22inpen(&mut self) -> PAD22INPEN_W {
        PAD22INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 22 pullup enable"]
    #[inline(always)]
    pub fn pad22pull(&mut self) -> PAD22PULL_W {
        PAD22PULL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 21 function select"]
    #[inline(always)]
    pub fn pad21fncsel(&mut self) -> PAD21FNCSEL_W {
        PAD21FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 21 drive strength"]
    #[inline(always)]
    pub fn pad21strng(&mut self) -> PAD21STRNG_W {
        PAD21STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 21 input enable"]
    #[inline(always)]
    pub fn pad21inpen(&mut self) -> PAD21INPEN_W {
        PAD21INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 21 pullup enable"]
    #[inline(always)]
    pub fn pad21pull(&mut self) -> PAD21PULL_W {
        PAD21PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 20 function select"]
    #[inline(always)]
    pub fn pad20fncsel(&mut self) -> PAD20FNCSEL_W {
        PAD20FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 20 drive strength"]
    #[inline(always)]
    pub fn pad20strng(&mut self) -> PAD20STRNG_W {
        PAD20STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 20 input enable"]
    #[inline(always)]
    pub fn pad20inpen(&mut self) -> PAD20INPEN_W {
        PAD20INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 20 pulldown enable"]
    #[inline(always)]
    pub fn pad20pull(&mut self) -> PAD20PULL_W {
        PAD20PULL_W { w: self }
    }
}
