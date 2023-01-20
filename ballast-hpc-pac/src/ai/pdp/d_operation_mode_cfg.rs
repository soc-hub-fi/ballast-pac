#[doc = "Register `D_OPERATION_MODE_CFG` reader"]
pub struct R(crate::R<D_OPERATION_MODE_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_OPERATION_MODE_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_OPERATION_MODE_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_OPERATION_MODE_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_OPERATION_MODE_CFG` writer"]
pub struct W(crate::W<D_OPERATION_MODE_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_OPERATION_MODE_CFG_SPEC>;
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
impl From<crate::W<D_OPERATION_MODE_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_OPERATION_MODE_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POOLING_METHOD` reader - "]
pub type POOLING_METHOD_R = crate::FieldReader<u8, POOLING_METHOD_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POOLING_METHOD_A {
    #[doc = "0: `0`"]
    AVERAGE = 0,
    #[doc = "1: `1`"]
    MAX = 1,
    #[doc = "2: `10`"]
    MIN = 2,
}
impl From<POOLING_METHOD_A> for u8 {
    #[inline(always)]
    fn from(variant: POOLING_METHOD_A) -> Self {
        variant as _
    }
}
impl POOLING_METHOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<POOLING_METHOD_A> {
        match self.bits {
            0 => Some(POOLING_METHOD_A::AVERAGE),
            1 => Some(POOLING_METHOD_A::MAX),
            2 => Some(POOLING_METHOD_A::MIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AVERAGE`"]
    #[inline(always)]
    pub fn is_average(&self) -> bool {
        *self == POOLING_METHOD_A::AVERAGE
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == POOLING_METHOD_A::MAX
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == POOLING_METHOD_A::MIN
    }
}
#[doc = "Field `POOLING_METHOD` writer - "]
pub type POOLING_METHOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_OPERATION_MODE_CFG_SPEC, u8, POOLING_METHOD_A, 2, O>;
impl<'a, const O: u8> POOLING_METHOD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn average(self) -> &'a mut W {
        self.variant(POOLING_METHOD_A::AVERAGE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(POOLING_METHOD_A::MAX)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(POOLING_METHOD_A::MIN)
    }
}
#[doc = "Field `FLYING_MODE` reader - "]
pub type FLYING_MODE_R = crate::BitReader<bool>;
#[doc = "Field `FLYING_MODE` writer - "]
pub type FLYING_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_OPERATION_MODE_CFG_SPEC, bool, O>;
#[doc = "Field `SPLIT_NUM` reader - "]
pub type SPLIT_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPLIT_NUM` writer - "]
pub type SPLIT_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_OPERATION_MODE_CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pooling_method(&self) -> POOLING_METHOD_R {
        POOLING_METHOD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn flying_mode(&self) -> FLYING_MODE_R {
        FLYING_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn split_num(&self) -> SPLIT_NUM_R {
        SPLIT_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pooling_method(&mut self) -> POOLING_METHOD_W<0> {
        POOLING_METHOD_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn flying_mode(&mut self) -> FLYING_MODE_W<4> {
        FLYING_MODE_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn split_num(&mut self) -> SPLIT_NUM_W<8> {
        SPLIT_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Split number\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_operation_mode_cfg](index.html) module"]
pub struct D_OPERATION_MODE_CFG_SPEC;
impl crate::RegisterSpec for D_OPERATION_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_operation_mode_cfg::R](R) reader structure"]
impl crate::Readable for D_OPERATION_MODE_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_operation_mode_cfg::W](W) writer structure"]
impl crate::Writable for D_OPERATION_MODE_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_OPERATION_MODE_CFG to value 0"]
impl crate::Resettable for D_OPERATION_MODE_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
