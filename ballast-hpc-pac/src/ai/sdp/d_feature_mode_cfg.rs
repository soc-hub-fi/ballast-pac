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
    #[doc = "1: `1`"]
    ON = 1,
    #[doc = "0: `0`"]
    OFF = 0,
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
            true => FLYING_MODE_A::ON,
            false => FLYING_MODE_A::OFF,
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FLYING_MODE_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FLYING_MODE_A::OFF
    }
}
#[doc = "Field `FLYING_MODE` writer - "]
pub type FLYING_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_FEATURE_MODE_CFG_SPEC, FLYING_MODE_A, O>;
impl<'a, const O: u8> FLYING_MODE_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(FLYING_MODE_A::ON)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(FLYING_MODE_A::OFF)
    }
}
#[doc = "Field `OUTPUT_DST` reader - "]
pub type OUTPUT_DST_R = crate::BitReader<OUTPUT_DST_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTPUT_DST_A {
    #[doc = "0: `0`"]
    MEM = 0,
    #[doc = "1: `1`"]
    PDP = 1,
}
impl From<OUTPUT_DST_A> for bool {
    #[inline(always)]
    fn from(variant: OUTPUT_DST_A) -> Self {
        variant as u8 != 0
    }
}
impl OUTPUT_DST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTPUT_DST_A {
        match self.bits {
            false => OUTPUT_DST_A::MEM,
            true => OUTPUT_DST_A::PDP,
        }
    }
    #[doc = "Checks if the value of the field is `MEM`"]
    #[inline(always)]
    pub fn is_mem(&self) -> bool {
        *self == OUTPUT_DST_A::MEM
    }
    #[doc = "Checks if the value of the field is `PDP`"]
    #[inline(always)]
    pub fn is_pdp(&self) -> bool {
        *self == OUTPUT_DST_A::PDP
    }
}
#[doc = "Field `OUTPUT_DST` writer - "]
pub type OUTPUT_DST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_FEATURE_MODE_CFG_SPEC, OUTPUT_DST_A, O>;
impl<'a, const O: u8> OUTPUT_DST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mem(self) -> &'a mut W {
        self.variant(OUTPUT_DST_A::MEM)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pdp(self) -> &'a mut W {
        self.variant(OUTPUT_DST_A::PDP)
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
#[doc = "Field `NAN_TO_ZERO` reader - "]
pub type NAN_TO_ZERO_R = crate::BitReader<NAN_TO_ZERO_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NAN_TO_ZERO_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<NAN_TO_ZERO_A> for bool {
    #[inline(always)]
    fn from(variant: NAN_TO_ZERO_A) -> Self {
        variant as u8 != 0
    }
}
impl NAN_TO_ZERO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NAN_TO_ZERO_A {
        match self.bits {
            true => NAN_TO_ZERO_A::ENABLE,
            false => NAN_TO_ZERO_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NAN_TO_ZERO_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NAN_TO_ZERO_A::DISABLE
    }
}
#[doc = "Field `NAN_TO_ZERO` writer - "]
pub type NAN_TO_ZERO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_FEATURE_MODE_CFG_SPEC, NAN_TO_ZERO_A, O>;
impl<'a, const O: u8> NAN_TO_ZERO_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(NAN_TO_ZERO_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(NAN_TO_ZERO_A::DISABLE)
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
    pub fn output_dst(&self) -> OUTPUT_DST_R {
        OUTPUT_DST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn winograd(&self) -> WINOGRAD_R {
        WINOGRAD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn nan_to_zero(&self) -> NAN_TO_ZERO_R {
        NAN_TO_ZERO_R::new(((self.bits >> 3) & 1) != 0)
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
    pub fn output_dst(&mut self) -> OUTPUT_DST_W<1> {
        OUTPUT_DST_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn winograd(&mut self) -> WINOGRAD_W<2> {
        WINOGRAD_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn nan_to_zero(&mut self) -> NAN_TO_ZERO_W<3> {
        NAN_TO_ZERO_W::new(self)
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
#[doc = "Operation configuration: flying mode, output destination, Direct or Winograd mode, flush NaN to zero, batch number.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_feature_mode_cfg](index.html) module"]
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
#[doc = "`reset()` method sets D_FEATURE_MODE_CFG to value 0"]
impl crate::Resettable for D_FEATURE_MODE_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
