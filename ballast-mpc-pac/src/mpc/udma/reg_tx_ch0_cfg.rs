#[doc = "Register `REG_TX_CH0_CFG` reader"]
pub struct R(crate::R<REG_TX_CH0_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_TX_CH0_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_TX_CH0_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_TX_CH0_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_TX_CH0_CFG` writer"]
pub struct W(crate::W<REG_TX_CH0_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_TX_CH0_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<REG_TX_CH0_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_TX_CH0_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Data transfer format: - 2'b00: 8-bit - 2'b01:16-bit - 2;b10:32-bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "0: `0`"]
    _8BITS = 0,
    #[doc = "1: `1`"]
    _16BITS = 1,
    #[doc = "2: `10`"]
    _32BITS = 2,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SIZE` reader - Data transfer format: - 2'b00: 8-bit - 2'b01:16-bit - 2;b10:32-bit"]
pub struct SIZE_R(crate::FieldReader<u8>);
impl SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIZE_A> {
        match self.bits {
            0 => Some(SIZE_A::_8BITS),
            1 => Some(SIZE_A::_16BITS),
            2 => Some(SIZE_A::_32BITS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8BITS`"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        **self == SIZE_A::_8BITS
    }
    #[doc = "Checks if the value of the field is `_16BITS`"]
    #[inline(always)]
    pub fn is_16bits(&self) -> bool {
        **self == SIZE_A::_16BITS
    }
    #[doc = "Checks if the value of the field is `_32BITS`"]
    #[inline(always)]
    pub fn is_32bits(&self) -> bool {
        **self == SIZE_A::_32BITS
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE` writer - Data transfer format: - 2'b00: 8-bit - 2'b01:16-bit - 2;b10:32-bit"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut W {
        self.variant(SIZE_A::_8BITS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16bits(self) -> &'a mut W {
        self.variant(SIZE_A::_16BITS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _32bits(self) -> &'a mut W {
        self.variant(SIZE_A::_32BITS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Data transfer mode: - 2'b00: Linear - 2'b01: Sliding - 2;b10:Circular - 2;b11:2D\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "1: `1`"]
    SLIDING = 1,
    #[doc = "0: `0`"]
    LINEAR = 0,
    #[doc = "2: `10`"]
    CIRCULAR = 2,
    #[doc = "3: `11`"]
    _2D = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Data transfer mode: - 2'b00: Linear - 2'b01: Sliding - 2;b10:Circular - 2;b11:2D"]
pub struct MODE_R(crate::FieldReader<u8>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            1 => MODE_A::SLIDING,
            0 => MODE_A::LINEAR,
            2 => MODE_A::CIRCULAR,
            3 => MODE_A::_2D,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLIDING`"]
    #[inline(always)]
    pub fn is_sliding(&self) -> bool {
        **self == MODE_A::SLIDING
    }
    #[doc = "Checks if the value of the field is `LINEAR`"]
    #[inline(always)]
    pub fn is_linear(&self) -> bool {
        **self == MODE_A::LINEAR
    }
    #[doc = "Checks if the value of the field is `CIRCULAR`"]
    #[inline(always)]
    pub fn is_circular(&self) -> bool {
        **self == MODE_A::CIRCULAR
    }
    #[doc = "Checks if the value of the field is `_2D`"]
    #[inline(always)]
    pub fn is_2d(&self) -> bool {
        **self == MODE_A::_2D
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Data transfer mode: - 2'b00: Linear - 2'b01: Sliding - 2;b10:Circular - 2;b11:2D"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sliding(self) -> &'a mut W {
        self.variant(MODE_A::SLIDING)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn linear(self) -> &'a mut W {
        self.variant(MODE_A::LINEAR)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn circular(self) -> &'a mut W {
        self.variant(MODE_A::CIRCULAR)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _2d(self) -> &'a mut W {
        self.variant(MODE_A::_2D)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Data transfer format: - 2'b00: 8-bit - 2'b01:16-bit - 2;b10:32-bit"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Data transfer mode: - 2'b00: Linear - 2'b01: Sliding - 2;b10:Circular - 2;b11:2D"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data transfer format: - 2'b00: 8-bit - 2'b01:16-bit - 2;b10:32-bit"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bits 8:9 - Data transfer mode: - 2'b00: Linear - 2'b01: Sliding - 2;b10:Circular - 2;b11:2D"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER tx channel configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_tx_ch0_cfg](index.html) module"]
pub struct REG_TX_CH0_CFG_SPEC;
impl crate::RegisterSpec for REG_TX_CH0_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_tx_ch0_cfg::R](R) reader structure"]
impl crate::Readable for REG_TX_CH0_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_tx_ch0_cfg::W](W) writer structure"]
impl crate::Writable for REG_TX_CH0_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_TX_CH0_CFG to value 0"]
impl crate::Resettable for REG_TX_CH0_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
