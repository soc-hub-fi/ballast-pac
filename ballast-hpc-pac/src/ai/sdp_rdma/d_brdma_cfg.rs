#[doc = "Register `D_BRDMA_CFG` reader"]
pub struct R(crate::R<D_BRDMA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_BRDMA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_BRDMA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_BRDMA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_BRDMA_CFG` writer"]
pub struct W(crate::W<D_BRDMA_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_BRDMA_CFG_SPEC>;
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
impl From<crate::W<D_BRDMA_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_BRDMA_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRDMA_DISABLE` reader - "]
pub type BRDMA_DISABLE_R = crate::BitReader<BRDMA_DISABLE_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRDMA_DISABLE_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<BRDMA_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BRDMA_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl BRDMA_DISABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRDMA_DISABLE_A {
        match self.bits {
            false => BRDMA_DISABLE_A::NO,
            true => BRDMA_DISABLE_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == BRDMA_DISABLE_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == BRDMA_DISABLE_A::YES
    }
}
#[doc = "Field `BRDMA_DISABLE` writer - "]
pub type BRDMA_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_BRDMA_CFG_SPEC, BRDMA_DISABLE_A, O>;
impl<'a, const O: u8> BRDMA_DISABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(BRDMA_DISABLE_A::NO)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(BRDMA_DISABLE_A::YES)
    }
}
#[doc = "Field `BRDMA_DATA_USE` reader - "]
pub type BRDMA_DATA_USE_R = crate::FieldReader<u8, BRDMA_DATA_USE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BRDMA_DATA_USE_A {
    #[doc = "0: `0`"]
    MUL = 0,
    #[doc = "1: `1`"]
    ALU = 1,
    #[doc = "2: `10`"]
    BOTH = 2,
}
impl From<BRDMA_DATA_USE_A> for u8 {
    #[inline(always)]
    fn from(variant: BRDMA_DATA_USE_A) -> Self {
        variant as _
    }
}
impl BRDMA_DATA_USE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BRDMA_DATA_USE_A> {
        match self.bits {
            0 => Some(BRDMA_DATA_USE_A::MUL),
            1 => Some(BRDMA_DATA_USE_A::ALU),
            2 => Some(BRDMA_DATA_USE_A::BOTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MUL`"]
    #[inline(always)]
    pub fn is_mul(&self) -> bool {
        *self == BRDMA_DATA_USE_A::MUL
    }
    #[doc = "Checks if the value of the field is `ALU`"]
    #[inline(always)]
    pub fn is_alu(&self) -> bool {
        *self == BRDMA_DATA_USE_A::ALU
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == BRDMA_DATA_USE_A::BOTH
    }
}
#[doc = "Field `BRDMA_DATA_USE` writer - "]
pub type BRDMA_DATA_USE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_BRDMA_CFG_SPEC, u8, BRDMA_DATA_USE_A, 2, O>;
impl<'a, const O: u8> BRDMA_DATA_USE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mul(self) -> &'a mut W {
        self.variant(BRDMA_DATA_USE_A::MUL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn alu(self) -> &'a mut W {
        self.variant(BRDMA_DATA_USE_A::ALU)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(BRDMA_DATA_USE_A::BOTH)
    }
}
#[doc = "Field `BRDMA_DATA_SIZE` reader - "]
pub type BRDMA_DATA_SIZE_R = crate::BitReader<BRDMA_DATA_SIZE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRDMA_DATA_SIZE_A {
    #[doc = "0: `0`"]
    ONE_BYTE = 0,
    #[doc = "1: `1`"]
    TWO_BYTE = 1,
}
impl From<BRDMA_DATA_SIZE_A> for bool {
    #[inline(always)]
    fn from(variant: BRDMA_DATA_SIZE_A) -> Self {
        variant as u8 != 0
    }
}
impl BRDMA_DATA_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRDMA_DATA_SIZE_A {
        match self.bits {
            false => BRDMA_DATA_SIZE_A::ONE_BYTE,
            true => BRDMA_DATA_SIZE_A::TWO_BYTE,
        }
    }
    #[doc = "Checks if the value of the field is `ONE_BYTE`"]
    #[inline(always)]
    pub fn is_one_byte(&self) -> bool {
        *self == BRDMA_DATA_SIZE_A::ONE_BYTE
    }
    #[doc = "Checks if the value of the field is `TWO_BYTE`"]
    #[inline(always)]
    pub fn is_two_byte(&self) -> bool {
        *self == BRDMA_DATA_SIZE_A::TWO_BYTE
    }
}
#[doc = "Field `BRDMA_DATA_SIZE` writer - "]
pub type BRDMA_DATA_SIZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_BRDMA_CFG_SPEC, BRDMA_DATA_SIZE_A, O>;
impl<'a, const O: u8> BRDMA_DATA_SIZE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn one_byte(self) -> &'a mut W {
        self.variant(BRDMA_DATA_SIZE_A::ONE_BYTE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn two_byte(self) -> &'a mut W {
        self.variant(BRDMA_DATA_SIZE_A::TWO_BYTE)
    }
}
#[doc = "Field `BRDMA_DATA_MODE` reader - "]
pub type BRDMA_DATA_MODE_R = crate::BitReader<BRDMA_DATA_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRDMA_DATA_MODE_A {
    #[doc = "0: `0`"]
    PER_KERNEL = 0,
    #[doc = "1: `1`"]
    PER_ELEMENT = 1,
}
impl From<BRDMA_DATA_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: BRDMA_DATA_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl BRDMA_DATA_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRDMA_DATA_MODE_A {
        match self.bits {
            false => BRDMA_DATA_MODE_A::PER_KERNEL,
            true => BRDMA_DATA_MODE_A::PER_ELEMENT,
        }
    }
    #[doc = "Checks if the value of the field is `PER_KERNEL`"]
    #[inline(always)]
    pub fn is_per_kernel(&self) -> bool {
        *self == BRDMA_DATA_MODE_A::PER_KERNEL
    }
    #[doc = "Checks if the value of the field is `PER_ELEMENT`"]
    #[inline(always)]
    pub fn is_per_element(&self) -> bool {
        *self == BRDMA_DATA_MODE_A::PER_ELEMENT
    }
}
#[doc = "Field `BRDMA_DATA_MODE` writer - "]
pub type BRDMA_DATA_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_BRDMA_CFG_SPEC, BRDMA_DATA_MODE_A, O>;
impl<'a, const O: u8> BRDMA_DATA_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn per_kernel(self) -> &'a mut W {
        self.variant(BRDMA_DATA_MODE_A::PER_KERNEL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn per_element(self) -> &'a mut W {
        self.variant(BRDMA_DATA_MODE_A::PER_ELEMENT)
    }
}
#[doc = "Field `BRDMA_RAM_TYPE` reader - "]
pub type BRDMA_RAM_TYPE_R = crate::BitReader<BRDMA_RAM_TYPE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRDMA_RAM_TYPE_A {
    #[doc = "0: `0`"]
    CV = 0,
    #[doc = "1: `1`"]
    MC = 1,
}
impl From<BRDMA_RAM_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: BRDMA_RAM_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl BRDMA_RAM_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRDMA_RAM_TYPE_A {
        match self.bits {
            false => BRDMA_RAM_TYPE_A::CV,
            true => BRDMA_RAM_TYPE_A::MC,
        }
    }
    #[doc = "Checks if the value of the field is `CV`"]
    #[inline(always)]
    pub fn is_cv(&self) -> bool {
        *self == BRDMA_RAM_TYPE_A::CV
    }
    #[doc = "Checks if the value of the field is `MC`"]
    #[inline(always)]
    pub fn is_mc(&self) -> bool {
        *self == BRDMA_RAM_TYPE_A::MC
    }
}
#[doc = "Field `BRDMA_RAM_TYPE` writer - "]
pub type BRDMA_RAM_TYPE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_BRDMA_CFG_SPEC, BRDMA_RAM_TYPE_A, O>;
impl<'a, const O: u8> BRDMA_RAM_TYPE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn cv(self) -> &'a mut W {
        self.variant(BRDMA_RAM_TYPE_A::CV)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mc(self) -> &'a mut W {
        self.variant(BRDMA_RAM_TYPE_A::MC)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn brdma_disable(&self) -> BRDMA_DISABLE_R {
        BRDMA_DISABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn brdma_data_use(&self) -> BRDMA_DATA_USE_R {
        BRDMA_DATA_USE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn brdma_data_size(&self) -> BRDMA_DATA_SIZE_R {
        BRDMA_DATA_SIZE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn brdma_data_mode(&self) -> BRDMA_DATA_MODE_R {
        BRDMA_DATA_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn brdma_ram_type(&self) -> BRDMA_RAM_TYPE_R {
        BRDMA_RAM_TYPE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn brdma_disable(&mut self) -> BRDMA_DISABLE_W<0> {
        BRDMA_DISABLE_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn brdma_data_use(&mut self) -> BRDMA_DATA_USE_W<1> {
        BRDMA_DATA_USE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn brdma_data_size(&mut self) -> BRDMA_DATA_SIZE_W<3> {
        BRDMA_DATA_SIZE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn brdma_data_mode(&mut self) -> BRDMA_DATA_MODE_W<4> {
        BRDMA_DATA_MODE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn brdma_ram_type(&mut self) -> BRDMA_RAM_TYPE_W<5> {
        BRDMA_RAM_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_brdma_cfg](index.html) module"]
pub struct D_BRDMA_CFG_SPEC;
impl crate::RegisterSpec for D_BRDMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_brdma_cfg::R](R) reader structure"]
impl crate::Readable for D_BRDMA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_brdma_cfg::W](W) writer structure"]
impl crate::Writable for D_BRDMA_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_BRDMA_CFG to value 0x01"]
impl crate::Resettable for D_BRDMA_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
