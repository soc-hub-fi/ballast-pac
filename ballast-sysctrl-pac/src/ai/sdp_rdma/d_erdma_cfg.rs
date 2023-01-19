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
#[doc = "Field `ERDMA_DISABLE` reader - "]
pub type ERDMA_DISABLE_R = crate::BitReader<ERDMA_DISABLE_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ERDMA_DISABLE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == ERDMA_DISABLE_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ERDMA_DISABLE_A::NO
    }
}
#[doc = "Field `ERDMA_DISABLE` writer - "]
pub type ERDMA_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_ERDMA_CFG_SPEC, ERDMA_DISABLE_A, O>;
impl<'a, const O: u8> ERDMA_DISABLE_W<'a, O> {
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
}
#[doc = "Field `ERDMA_DATA_USE` reader - "]
pub type ERDMA_DATA_USE_R = crate::FieldReader<u8, ERDMA_DATA_USE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ERDMA_DATA_USE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == ERDMA_DATA_USE_A::MUL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == ERDMA_DATA_USE_A::BOTH
    }
    #[doc = "Checks if the value of the field is `ALU`"]
    #[inline(always)]
    pub fn is_alu(&self) -> bool {
        *self == ERDMA_DATA_USE_A::ALU
    }
}
#[doc = "Field `ERDMA_DATA_USE` writer - "]
pub type ERDMA_DATA_USE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_ERDMA_CFG_SPEC, u8, ERDMA_DATA_USE_A, 2, O>;
impl<'a, const O: u8> ERDMA_DATA_USE_W<'a, O> {
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
}
#[doc = "Field `ERDMA_DATA_SIZE` reader - "]
pub type ERDMA_DATA_SIZE_R = crate::BitReader<ERDMA_DATA_SIZE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ERDMA_DATA_SIZE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == ERDMA_DATA_SIZE_A::TWO_BYTE
    }
    #[doc = "Checks if the value of the field is `ONE_BYTE`"]
    #[inline(always)]
    pub fn is_one_byte(&self) -> bool {
        *self == ERDMA_DATA_SIZE_A::ONE_BYTE
    }
}
#[doc = "Field `ERDMA_DATA_SIZE` writer - "]
pub type ERDMA_DATA_SIZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_ERDMA_CFG_SPEC, ERDMA_DATA_SIZE_A, O>;
impl<'a, const O: u8> ERDMA_DATA_SIZE_W<'a, O> {
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
}
#[doc = "Field `ERDMA_DATA_MODE` reader - "]
pub type ERDMA_DATA_MODE_R = crate::BitReader<ERDMA_DATA_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ERDMA_DATA_MODE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == ERDMA_DATA_MODE_A::PER_KERNEL
    }
    #[doc = "Checks if the value of the field is `PER_ELEMENT`"]
    #[inline(always)]
    pub fn is_per_element(&self) -> bool {
        *self == ERDMA_DATA_MODE_A::PER_ELEMENT
    }
}
#[doc = "Field `ERDMA_DATA_MODE` writer - "]
pub type ERDMA_DATA_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_ERDMA_CFG_SPEC, ERDMA_DATA_MODE_A, O>;
impl<'a, const O: u8> ERDMA_DATA_MODE_W<'a, O> {
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
}
#[doc = "Field `ERDMA_RAM_TYPE` reader - "]
pub type ERDMA_RAM_TYPE_R = crate::BitReader<ERDMA_RAM_TYPE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ERDMA_RAM_TYPE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == ERDMA_RAM_TYPE_A::MC
    }
    #[doc = "Checks if the value of the field is `CV`"]
    #[inline(always)]
    pub fn is_cv(&self) -> bool {
        *self == ERDMA_RAM_TYPE_A::CV
    }
}
#[doc = "Field `ERDMA_RAM_TYPE` writer - "]
pub type ERDMA_RAM_TYPE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_ERDMA_CFG_SPEC, ERDMA_RAM_TYPE_A, O>;
impl<'a, const O: u8> ERDMA_RAM_TYPE_W<'a, O> {
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
    #[must_use]
    pub fn erdma_disable(&mut self) -> ERDMA_DISABLE_W<0> {
        ERDMA_DISABLE_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn erdma_data_use(&mut self) -> ERDMA_DATA_USE_W<1> {
        ERDMA_DATA_USE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn erdma_data_size(&mut self) -> ERDMA_DATA_SIZE_W<3> {
        ERDMA_DATA_SIZE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn erdma_data_mode(&mut self) -> ERDMA_DATA_MODE_W<4> {
        ERDMA_DATA_MODE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn erdma_ram_type(&mut self) -> ERDMA_RAM_TYPE_W<5> {
        ERDMA_RAM_TYPE_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_ERDMA_CFG to value 0x01"]
impl crate::Resettable for D_ERDMA_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
