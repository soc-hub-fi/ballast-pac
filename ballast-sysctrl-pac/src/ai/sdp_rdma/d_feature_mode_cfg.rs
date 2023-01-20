#[doc = "Register `D_FEATURE_MODE_CFG` reader"]
pub struct R(crate::R<D_FEATURE_MODE_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_FEATURE_MODE_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_FEATURE_MODE_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_FEATURE_MODE_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_FEATURE_MODE_CFG` writer"]
pub struct W(crate::W<D_FEATURE_MODE_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_FEATURE_MODE_CFG_SPEC>;
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
impl From<crate::W<D_FEATURE_MODE_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_FEATURE_MODE_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLYING_MODE` reader - "]
pub type FLYING_MODE_R = crate::BitReader<FLYING_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLYING_MODE_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<FLYING_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FLYING_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl FLYING_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLYING_MODE_A {
        match self.bits {
            false => FLYING_MODE_A::OFF,
            true => FLYING_MODE_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FLYING_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FLYING_MODE_A::ON
    }
}
#[doc = "Field `FLYING_MODE` writer - "]
pub type FLYING_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_FEATURE_MODE_CFG_SPEC, FLYING_MODE_A, O>;
impl<'a, const O: u8> FLYING_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(FLYING_MODE_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(FLYING_MODE_A::ON)
    }
}
#[doc = "Field `WINOGRAD` reader - "]
pub type WINOGRAD_R = crate::BitReader<WINOGRAD_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WINOGRAD_A {
    #[doc = "1: `1`"]
    ON = 1,
    #[doc = "0: `0`"]
    OFF = 0,
}
impl From<WINOGRAD_A> for bool {
    #[inline(always)]
    fn from(variant: WINOGRAD_A) -> Self {
        variant as u8 != 0
    }
}
impl WINOGRAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WINOGRAD_A {
        match self.bits {
            true => WINOGRAD_A::ON,
            false => WINOGRAD_A::OFF,
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == WINOGRAD_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == WINOGRAD_A::OFF
    }
}
#[doc = "Field `WINOGRAD` writer - "]
pub type WINOGRAD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_FEATURE_MODE_CFG_SPEC, WINOGRAD_A, O>;
impl<'a, const O: u8> WINOGRAD_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(WINOGRAD_A::ON)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(WINOGRAD_A::OFF)
    }
}
#[doc = "Field `IN_PRECISION` reader - "]
pub type IN_PRECISION_R = crate::FieldReader<u8, IN_PRECISION_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IN_PRECISION_A {
    #[doc = "2: `10`"]
    FP16 = 2,
    #[doc = "1: `1`"]
    INT16 = 1,
    #[doc = "0: `0`"]
    INT8 = 0,
}
impl From<IN_PRECISION_A> for u8 {
    #[inline(always)]
    fn from(variant: IN_PRECISION_A) -> Self {
        variant as _
    }
}
impl IN_PRECISION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IN_PRECISION_A> {
        match self.bits {
            2 => Some(IN_PRECISION_A::FP16),
            1 => Some(IN_PRECISION_A::INT16),
            0 => Some(IN_PRECISION_A::INT8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FP16`"]
    #[inline(always)]
    pub fn is_fp16(&self) -> bool {
        *self == IN_PRECISION_A::FP16
    }
    #[doc = "Checks if the value of the field is `INT16`"]
    #[inline(always)]
    pub fn is_int16(&self) -> bool {
        *self == IN_PRECISION_A::INT16
    }
    #[doc = "Checks if the value of the field is `INT8`"]
    #[inline(always)]
    pub fn is_int8(&self) -> bool {
        *self == IN_PRECISION_A::INT8
    }
}
#[doc = "Field `IN_PRECISION` writer - "]
pub type IN_PRECISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_FEATURE_MODE_CFG_SPEC, u8, IN_PRECISION_A, 2, O>;
impl<'a, const O: u8> IN_PRECISION_W<'a, O> {
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fp16(self) -> &'a mut W {
        self.variant(IN_PRECISION_A::FP16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn int16(self) -> &'a mut W {
        self.variant(IN_PRECISION_A::INT16)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn int8(self) -> &'a mut W {
        self.variant(IN_PRECISION_A::INT8)
    }
}
#[doc = "Field `PROC_PRECISION` reader - "]
pub type PROC_PRECISION_R = crate::FieldReader<u8, PROC_PRECISION_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PROC_PRECISION_A {
    #[doc = "2: `10`"]
    FP16 = 2,
    #[doc = "1: `1`"]
    INT16 = 1,
    #[doc = "0: `0`"]
    INT8 = 0,
}
impl From<PROC_PRECISION_A> for u8 {
    #[inline(always)]
    fn from(variant: PROC_PRECISION_A) -> Self {
        variant as _
    }
}
impl PROC_PRECISION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PROC_PRECISION_A> {
        match self.bits {
            2 => Some(PROC_PRECISION_A::FP16),
            1 => Some(PROC_PRECISION_A::INT16),
            0 => Some(PROC_PRECISION_A::INT8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FP16`"]
    #[inline(always)]
    pub fn is_fp16(&self) -> bool {
        *self == PROC_PRECISION_A::FP16
    }
    #[doc = "Checks if the value of the field is `INT16`"]
    #[inline(always)]
    pub fn is_int16(&self) -> bool {
        *self == PROC_PRECISION_A::INT16
    }
    #[doc = "Checks if the value of the field is `INT8`"]
    #[inline(always)]
    pub fn is_int8(&self) -> bool {
        *self == PROC_PRECISION_A::INT8
    }
}
#[doc = "Field `PROC_PRECISION` writer - "]
pub type PROC_PRECISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_FEATURE_MODE_CFG_SPEC, u8, PROC_PRECISION_A, 2, O>;
impl<'a, const O: u8> PROC_PRECISION_W<'a, O> {
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fp16(self) -> &'a mut W {
        self.variant(PROC_PRECISION_A::FP16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn int16(self) -> &'a mut W {
        self.variant(PROC_PRECISION_A::INT16)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn int8(self) -> &'a mut W {
        self.variant(PROC_PRECISION_A::INT8)
    }
}
#[doc = "Field `OUT_PRECISION` reader - "]
pub type OUT_PRECISION_R = crate::FieldReader<u8, OUT_PRECISION_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUT_PRECISION_A {
    #[doc = "2: `10`"]
    FP16 = 2,
    #[doc = "1: `1`"]
    INT16 = 1,
    #[doc = "0: `0`"]
    INT8 = 0,
}
impl From<OUT_PRECISION_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT_PRECISION_A) -> Self {
        variant as _
    }
}
impl OUT_PRECISION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUT_PRECISION_A> {
        match self.bits {
            2 => Some(OUT_PRECISION_A::FP16),
            1 => Some(OUT_PRECISION_A::INT16),
            0 => Some(OUT_PRECISION_A::INT8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FP16`"]
    #[inline(always)]
    pub fn is_fp16(&self) -> bool {
        *self == OUT_PRECISION_A::FP16
    }
    #[doc = "Checks if the value of the field is `INT16`"]
    #[inline(always)]
    pub fn is_int16(&self) -> bool {
        *self == OUT_PRECISION_A::INT16
    }
    #[doc = "Checks if the value of the field is `INT8`"]
    #[inline(always)]
    pub fn is_int8(&self) -> bool {
        *self == OUT_PRECISION_A::INT8
    }
}
#[doc = "Field `OUT_PRECISION` writer - "]
pub type OUT_PRECISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_FEATURE_MODE_CFG_SPEC, u8, OUT_PRECISION_A, 2, O>;
impl<'a, const O: u8> OUT_PRECISION_W<'a, O> {
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fp16(self) -> &'a mut W {
        self.variant(OUT_PRECISION_A::FP16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn int16(self) -> &'a mut W {
        self.variant(OUT_PRECISION_A::INT16)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn int8(self) -> &'a mut W {
        self.variant(OUT_PRECISION_A::INT8)
    }
}
#[doc = "Field `BATCH_NUMBER` reader - "]
pub type BATCH_NUMBER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BATCH_NUMBER` writer - "]
pub type BATCH_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_FEATURE_MODE_CFG_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flying_mode(&self) -> FLYING_MODE_R {
        FLYING_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn winograd(&self) -> WINOGRAD_R {
        WINOGRAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn in_precision(&self) -> IN_PRECISION_R {
        IN_PRECISION_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn proc_precision(&self) -> PROC_PRECISION_R {
        PROC_PRECISION_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn out_precision(&self) -> OUT_PRECISION_R {
        OUT_PRECISION_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn batch_number(&self) -> BATCH_NUMBER_R {
        BATCH_NUMBER_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn flying_mode(&mut self) -> FLYING_MODE_W<0> {
        FLYING_MODE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn winograd(&mut self) -> WINOGRAD_W<1> {
        WINOGRAD_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn in_precision(&mut self) -> IN_PRECISION_W<2> {
        IN_PRECISION_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn proc_precision(&mut self) -> PROC_PRECISION_W<4> {
        PROC_PRECISION_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn out_precision(&mut self) -> OUT_PRECISION_W<6> {
        OUT_PRECISION_W::new(self)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn batch_number(&mut self) -> BATCH_NUMBER_W<8> {
        BATCH_NUMBER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_feature_mode_cfg](index.html) module"]
pub struct D_FEATURE_MODE_CFG_SPEC;
impl crate::RegisterSpec for D_FEATURE_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_feature_mode_cfg::R](R) reader structure"]
impl crate::Readable for D_FEATURE_MODE_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_feature_mode_cfg::W](W) writer structure"]
impl crate::Writable for D_FEATURE_MODE_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_FEATURE_MODE_CFG to value 0x14"]
impl crate::Resettable for D_FEATURE_MODE_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
