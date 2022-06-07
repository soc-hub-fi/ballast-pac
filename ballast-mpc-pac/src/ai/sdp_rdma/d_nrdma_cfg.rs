#[doc = "Register `D_NRDMA_CFG` reader"]
pub struct R(crate::R<D_NRDMA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_NRDMA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_NRDMA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_NRDMA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_NRDMA_CFG` writer"]
pub struct W(crate::W<D_NRDMA_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_NRDMA_CFG_SPEC>;
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
impl From<crate::W<D_NRDMA_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_NRDMA_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRDMA_DISABLE_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<NRDMA_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: NRDMA_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRDMA_DISABLE` reader - "]
pub struct NRDMA_DISABLE_R(crate::FieldReader<bool>);
impl NRDMA_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRDMA_DISABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRDMA_DISABLE_A {
        match self.bits {
            true => NRDMA_DISABLE_A::YES,
            false => NRDMA_DISABLE_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == NRDMA_DISABLE_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == NRDMA_DISABLE_A::NO
    }
}
impl core::ops::Deref for NRDMA_DISABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRDMA_DISABLE` writer - "]
pub struct NRDMA_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NRDMA_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NRDMA_DISABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(NRDMA_DISABLE_A::YES)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(NRDMA_DISABLE_A::NO)
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NRDMA_DATA_USE_A {
    #[doc = "0: `0`"]
    MUL = 0,
    #[doc = "2: `10`"]
    BOTH = 2,
    #[doc = "1: `1`"]
    ALU = 1,
}
impl From<NRDMA_DATA_USE_A> for u8 {
    #[inline(always)]
    fn from(variant: NRDMA_DATA_USE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NRDMA_DATA_USE` reader - "]
pub struct NRDMA_DATA_USE_R(crate::FieldReader<u8>);
impl NRDMA_DATA_USE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NRDMA_DATA_USE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NRDMA_DATA_USE_A> {
        match self.bits {
            0 => Some(NRDMA_DATA_USE_A::MUL),
            2 => Some(NRDMA_DATA_USE_A::BOTH),
            1 => Some(NRDMA_DATA_USE_A::ALU),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MUL`"]
    #[inline(always)]
    pub fn is_mul(&self) -> bool {
        **self == NRDMA_DATA_USE_A::MUL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == NRDMA_DATA_USE_A::BOTH
    }
    #[doc = "Checks if the value of the field is `ALU`"]
    #[inline(always)]
    pub fn is_alu(&self) -> bool {
        **self == NRDMA_DATA_USE_A::ALU
    }
}
impl core::ops::Deref for NRDMA_DATA_USE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRDMA_DATA_USE` writer - "]
pub struct NRDMA_DATA_USE_W<'a> {
    w: &'a mut W,
}
impl<'a> NRDMA_DATA_USE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NRDMA_DATA_USE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mul(self) -> &'a mut W {
        self.variant(NRDMA_DATA_USE_A::MUL)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(NRDMA_DATA_USE_A::BOTH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn alu(self) -> &'a mut W {
        self.variant(NRDMA_DATA_USE_A::ALU)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 1)) | ((value as u32 & 3) << 1);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRDMA_DATA_SIZE_A {
    #[doc = "1: `1`"]
    TWO_BYTE = 1,
    #[doc = "0: `0`"]
    ONE_BYTE = 0,
}
impl From<NRDMA_DATA_SIZE_A> for bool {
    #[inline(always)]
    fn from(variant: NRDMA_DATA_SIZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRDMA_DATA_SIZE` reader - "]
pub struct NRDMA_DATA_SIZE_R(crate::FieldReader<bool>);
impl NRDMA_DATA_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRDMA_DATA_SIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRDMA_DATA_SIZE_A {
        match self.bits {
            true => NRDMA_DATA_SIZE_A::TWO_BYTE,
            false => NRDMA_DATA_SIZE_A::ONE_BYTE,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_BYTE`"]
    #[inline(always)]
    pub fn is_two_byte(&self) -> bool {
        **self == NRDMA_DATA_SIZE_A::TWO_BYTE
    }
    #[doc = "Checks if the value of the field is `ONE_BYTE`"]
    #[inline(always)]
    pub fn is_one_byte(&self) -> bool {
        **self == NRDMA_DATA_SIZE_A::ONE_BYTE
    }
}
impl core::ops::Deref for NRDMA_DATA_SIZE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRDMA_DATA_SIZE` writer - "]
pub struct NRDMA_DATA_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> NRDMA_DATA_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NRDMA_DATA_SIZE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn two_byte(self) -> &'a mut W {
        self.variant(NRDMA_DATA_SIZE_A::TWO_BYTE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn one_byte(self) -> &'a mut W {
        self.variant(NRDMA_DATA_SIZE_A::ONE_BYTE)
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRDMA_DATA_MODE_A {
    #[doc = "0: `0`"]
    PER_KERNEL = 0,
    #[doc = "1: `1`"]
    PER_ELEMENT = 1,
}
impl From<NRDMA_DATA_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: NRDMA_DATA_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRDMA_DATA_MODE` reader - "]
pub struct NRDMA_DATA_MODE_R(crate::FieldReader<bool>);
impl NRDMA_DATA_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRDMA_DATA_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRDMA_DATA_MODE_A {
        match self.bits {
            false => NRDMA_DATA_MODE_A::PER_KERNEL,
            true => NRDMA_DATA_MODE_A::PER_ELEMENT,
        }
    }
    #[doc = "Checks if the value of the field is `PER_KERNEL`"]
    #[inline(always)]
    pub fn is_per_kernel(&self) -> bool {
        **self == NRDMA_DATA_MODE_A::PER_KERNEL
    }
    #[doc = "Checks if the value of the field is `PER_ELEMENT`"]
    #[inline(always)]
    pub fn is_per_element(&self) -> bool {
        **self == NRDMA_DATA_MODE_A::PER_ELEMENT
    }
}
impl core::ops::Deref for NRDMA_DATA_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRDMA_DATA_MODE` writer - "]
pub struct NRDMA_DATA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> NRDMA_DATA_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NRDMA_DATA_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn per_kernel(self) -> &'a mut W {
        self.variant(NRDMA_DATA_MODE_A::PER_KERNEL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn per_element(self) -> &'a mut W {
        self.variant(NRDMA_DATA_MODE_A::PER_ELEMENT)
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRDMA_RAM_TYPE_A {
    #[doc = "1: `1`"]
    MC = 1,
    #[doc = "0: `0`"]
    CV = 0,
}
impl From<NRDMA_RAM_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: NRDMA_RAM_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRDMA_RAM_TYPE` reader - "]
pub struct NRDMA_RAM_TYPE_R(crate::FieldReader<bool>);
impl NRDMA_RAM_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRDMA_RAM_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRDMA_RAM_TYPE_A {
        match self.bits {
            true => NRDMA_RAM_TYPE_A::MC,
            false => NRDMA_RAM_TYPE_A::CV,
        }
    }
    #[doc = "Checks if the value of the field is `MC`"]
    #[inline(always)]
    pub fn is_mc(&self) -> bool {
        **self == NRDMA_RAM_TYPE_A::MC
    }
    #[doc = "Checks if the value of the field is `CV`"]
    #[inline(always)]
    pub fn is_cv(&self) -> bool {
        **self == NRDMA_RAM_TYPE_A::CV
    }
}
impl core::ops::Deref for NRDMA_RAM_TYPE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRDMA_RAM_TYPE` writer - "]
pub struct NRDMA_RAM_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> NRDMA_RAM_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NRDMA_RAM_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mc(self) -> &'a mut W {
        self.variant(NRDMA_RAM_TYPE_A::MC)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn cv(self) -> &'a mut W {
        self.variant(NRDMA_RAM_TYPE_A::CV)
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nrdma_disable(&self) -> NRDMA_DISABLE_R {
        NRDMA_DISABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn nrdma_data_use(&self) -> NRDMA_DATA_USE_R {
        NRDMA_DATA_USE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn nrdma_data_size(&self) -> NRDMA_DATA_SIZE_R {
        NRDMA_DATA_SIZE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn nrdma_data_mode(&self) -> NRDMA_DATA_MODE_R {
        NRDMA_DATA_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn nrdma_ram_type(&self) -> NRDMA_RAM_TYPE_R {
        NRDMA_RAM_TYPE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nrdma_disable(&mut self) -> NRDMA_DISABLE_W {
        NRDMA_DISABLE_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn nrdma_data_use(&mut self) -> NRDMA_DATA_USE_W {
        NRDMA_DATA_USE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn nrdma_data_size(&mut self) -> NRDMA_DATA_SIZE_W {
        NRDMA_DATA_SIZE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn nrdma_data_mode(&mut self) -> NRDMA_DATA_MODE_W {
        NRDMA_DATA_MODE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn nrdma_ram_type(&mut self) -> NRDMA_RAM_TYPE_W {
        NRDMA_RAM_TYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_nrdma_cfg](index.html) module"]
pub struct D_NRDMA_CFG_SPEC;
impl crate::RegisterSpec for D_NRDMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_nrdma_cfg::R](R) reader structure"]
impl crate::Readable for D_NRDMA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_nrdma_cfg::W](W) writer structure"]
impl crate::Writable for D_NRDMA_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_NRDMA_CFG to value 0x01"]
impl crate::Resettable for D_NRDMA_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
