#[doc = "Register `D_ERDMA_CFG` reader"]
pub struct R(crate::R<D_ERDMA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_ERDMA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_ERDMA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_ERDMA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_ERDMA_CFG` writer"]
pub struct W(crate::W<D_ERDMA_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_ERDMA_CFG_SPEC>;
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
impl From<crate::W<D_ERDMA_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_ERDMA_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERDMA_DISABLE_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<ERDMA_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ERDMA_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERDMA_DISABLE` reader - "]
pub struct ERDMA_DISABLE_R(crate::FieldReader<bool, ERDMA_DISABLE_A>);
impl ERDMA_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERDMA_DISABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERDMA_DISABLE_A {
        match self.bits {
            true => ERDMA_DISABLE_A::YES,
            false => ERDMA_DISABLE_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == ERDMA_DISABLE_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == ERDMA_DISABLE_A::NO
    }
}
impl core::ops::Deref for ERDMA_DISABLE_R {
    type Target = crate::FieldReader<bool, ERDMA_DISABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERDMA_DISABLE` writer - "]
pub struct ERDMA_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERDMA_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERDMA_DISABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(ERDMA_DISABLE_A::YES)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ERDMA_DISABLE_A::NO)
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
pub enum ERDMA_DATA_USE_A {
    #[doc = "0: `0`"]
    MUL = 0,
    #[doc = "2: `10`"]
    BOTH = 2,
    #[doc = "1: `1`"]
    ALU = 1,
}
impl From<ERDMA_DATA_USE_A> for u8 {
    #[inline(always)]
    fn from(variant: ERDMA_DATA_USE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ERDMA_DATA_USE` reader - "]
pub struct ERDMA_DATA_USE_R(crate::FieldReader<u8, ERDMA_DATA_USE_A>);
impl ERDMA_DATA_USE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERDMA_DATA_USE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERDMA_DATA_USE_A> {
        match self.bits {
            0 => Some(ERDMA_DATA_USE_A::MUL),
            2 => Some(ERDMA_DATA_USE_A::BOTH),
            1 => Some(ERDMA_DATA_USE_A::ALU),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MUL`"]
    #[inline(always)]
    pub fn is_mul(&self) -> bool {
        **self == ERDMA_DATA_USE_A::MUL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == ERDMA_DATA_USE_A::BOTH
    }
    #[doc = "Checks if the value of the field is `ALU`"]
    #[inline(always)]
    pub fn is_alu(&self) -> bool {
        **self == ERDMA_DATA_USE_A::ALU
    }
}
impl core::ops::Deref for ERDMA_DATA_USE_R {
    type Target = crate::FieldReader<u8, ERDMA_DATA_USE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERDMA_DATA_USE` writer - "]
pub struct ERDMA_DATA_USE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERDMA_DATA_USE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERDMA_DATA_USE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mul(self) -> &'a mut W {
        self.variant(ERDMA_DATA_USE_A::MUL)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(ERDMA_DATA_USE_A::BOTH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn alu(self) -> &'a mut W {
        self.variant(ERDMA_DATA_USE_A::ALU)
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
pub enum ERDMA_DATA_SIZE_A {
    #[doc = "1: `1`"]
    TWO_BYTE = 1,
    #[doc = "0: `0`"]
    ONE_BYTE = 0,
}
impl From<ERDMA_DATA_SIZE_A> for bool {
    #[inline(always)]
    fn from(variant: ERDMA_DATA_SIZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERDMA_DATA_SIZE` reader - "]
pub struct ERDMA_DATA_SIZE_R(crate::FieldReader<bool, ERDMA_DATA_SIZE_A>);
impl ERDMA_DATA_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERDMA_DATA_SIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERDMA_DATA_SIZE_A {
        match self.bits {
            true => ERDMA_DATA_SIZE_A::TWO_BYTE,
            false => ERDMA_DATA_SIZE_A::ONE_BYTE,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_BYTE`"]
    #[inline(always)]
    pub fn is_two_byte(&self) -> bool {
        **self == ERDMA_DATA_SIZE_A::TWO_BYTE
    }
    #[doc = "Checks if the value of the field is `ONE_BYTE`"]
    #[inline(always)]
    pub fn is_one_byte(&self) -> bool {
        **self == ERDMA_DATA_SIZE_A::ONE_BYTE
    }
}
impl core::ops::Deref for ERDMA_DATA_SIZE_R {
    type Target = crate::FieldReader<bool, ERDMA_DATA_SIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERDMA_DATA_SIZE` writer - "]
pub struct ERDMA_DATA_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERDMA_DATA_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERDMA_DATA_SIZE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn two_byte(self) -> &'a mut W {
        self.variant(ERDMA_DATA_SIZE_A::TWO_BYTE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn one_byte(self) -> &'a mut W {
        self.variant(ERDMA_DATA_SIZE_A::ONE_BYTE)
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
pub enum ERDMA_DATA_MODE_A {
    #[doc = "0: `0`"]
    PER_KERNEL = 0,
    #[doc = "1: `1`"]
    PER_ELEMENT = 1,
}
impl From<ERDMA_DATA_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: ERDMA_DATA_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERDMA_DATA_MODE` reader - "]
pub struct ERDMA_DATA_MODE_R(crate::FieldReader<bool, ERDMA_DATA_MODE_A>);
impl ERDMA_DATA_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERDMA_DATA_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERDMA_DATA_MODE_A {
        match self.bits {
            false => ERDMA_DATA_MODE_A::PER_KERNEL,
            true => ERDMA_DATA_MODE_A::PER_ELEMENT,
        }
    }
    #[doc = "Checks if the value of the field is `PER_KERNEL`"]
    #[inline(always)]
    pub fn is_per_kernel(&self) -> bool {
        **self == ERDMA_DATA_MODE_A::PER_KERNEL
    }
    #[doc = "Checks if the value of the field is `PER_ELEMENT`"]
    #[inline(always)]
    pub fn is_per_element(&self) -> bool {
        **self == ERDMA_DATA_MODE_A::PER_ELEMENT
    }
}
impl core::ops::Deref for ERDMA_DATA_MODE_R {
    type Target = crate::FieldReader<bool, ERDMA_DATA_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERDMA_DATA_MODE` writer - "]
pub struct ERDMA_DATA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERDMA_DATA_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERDMA_DATA_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn per_kernel(self) -> &'a mut W {
        self.variant(ERDMA_DATA_MODE_A::PER_KERNEL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn per_element(self) -> &'a mut W {
        self.variant(ERDMA_DATA_MODE_A::PER_ELEMENT)
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
pub enum ERDMA_RAM_TYPE_A {
    #[doc = "1: `1`"]
    MC = 1,
    #[doc = "0: `0`"]
    CV = 0,
}
impl From<ERDMA_RAM_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: ERDMA_RAM_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERDMA_RAM_TYPE` reader - "]
pub struct ERDMA_RAM_TYPE_R(crate::FieldReader<bool, ERDMA_RAM_TYPE_A>);
impl ERDMA_RAM_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERDMA_RAM_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERDMA_RAM_TYPE_A {
        match self.bits {
            true => ERDMA_RAM_TYPE_A::MC,
            false => ERDMA_RAM_TYPE_A::CV,
        }
    }
    #[doc = "Checks if the value of the field is `MC`"]
    #[inline(always)]
    pub fn is_mc(&self) -> bool {
        **self == ERDMA_RAM_TYPE_A::MC
    }
    #[doc = "Checks if the value of the field is `CV`"]
    #[inline(always)]
    pub fn is_cv(&self) -> bool {
        **self == ERDMA_RAM_TYPE_A::CV
    }
}
impl core::ops::Deref for ERDMA_RAM_TYPE_R {
    type Target = crate::FieldReader<bool, ERDMA_RAM_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERDMA_RAM_TYPE` writer - "]
pub struct ERDMA_RAM_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERDMA_RAM_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERDMA_RAM_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mc(self) -> &'a mut W {
        self.variant(ERDMA_RAM_TYPE_A::MC)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn cv(self) -> &'a mut W {
        self.variant(ERDMA_RAM_TYPE_A::CV)
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
    pub fn erdma_disable(&self) -> ERDMA_DISABLE_R {
        ERDMA_DISABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn erdma_data_use(&self) -> ERDMA_DATA_USE_R {
        ERDMA_DATA_USE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn erdma_data_size(&self) -> ERDMA_DATA_SIZE_R {
        ERDMA_DATA_SIZE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn erdma_data_mode(&self) -> ERDMA_DATA_MODE_R {
        ERDMA_DATA_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn erdma_ram_type(&self) -> ERDMA_RAM_TYPE_R {
        ERDMA_RAM_TYPE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn erdma_disable(&mut self) -> ERDMA_DISABLE_W {
        ERDMA_DISABLE_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn erdma_data_use(&mut self) -> ERDMA_DATA_USE_W {
        ERDMA_DATA_USE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn erdma_data_size(&mut self) -> ERDMA_DATA_SIZE_W {
        ERDMA_DATA_SIZE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn erdma_data_mode(&mut self) -> ERDMA_DATA_MODE_W {
        ERDMA_DATA_MODE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn erdma_ram_type(&mut self) -> ERDMA_RAM_TYPE_W {
        ERDMA_RAM_TYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_erdma_cfg](index.html) module"]
pub struct D_ERDMA_CFG_SPEC;
impl crate::RegisterSpec for D_ERDMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_erdma_cfg::R](R) reader structure"]
impl crate::Readable for D_ERDMA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_erdma_cfg::W](W) writer structure"]
impl crate::Writable for D_ERDMA_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_ERDMA_CFG to value 0x01"]
impl crate::Resettable for D_ERDMA_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
