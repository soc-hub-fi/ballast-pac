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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `POOLING_METHOD` reader - "]
pub struct POOLING_METHOD_R(crate::FieldReader<u8>);
impl POOLING_METHOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        POOLING_METHOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == POOLING_METHOD_A::AVERAGE
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        **self == POOLING_METHOD_A::MAX
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        **self == POOLING_METHOD_A::MIN
    }
}
impl core::ops::Deref for POOLING_METHOD_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POOLING_METHOD` writer - "]
pub struct POOLING_METHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> POOLING_METHOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POOLING_METHOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `FLYING_MODE` reader - "]
pub struct FLYING_MODE_R(crate::FieldReader<bool>);
impl FLYING_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLYING_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLYING_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLYING_MODE` writer - "]
pub struct FLYING_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLYING_MODE_W<'a> {
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
#[doc = "Field `SPLIT_NUM` reader - "]
pub struct SPLIT_NUM_R(crate::FieldReader<u8>);
impl SPLIT_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPLIT_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPLIT_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIT_NUM` writer - "]
pub struct SPLIT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
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
    pub fn pooling_method(&mut self) -> POOLING_METHOD_W {
        POOLING_METHOD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn flying_mode(&mut self) -> FLYING_MODE_W {
        FLYING_MODE_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn split_num(&mut self) -> SPLIT_NUM_W {
        SPLIT_NUM_W { w: self }
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
}
#[doc = "`reset()` method sets D_OPERATION_MODE_CFG to value 0"]
impl crate::Resettable for D_OPERATION_MODE_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
