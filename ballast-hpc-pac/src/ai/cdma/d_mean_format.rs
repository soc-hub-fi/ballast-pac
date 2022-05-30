#[doc = "Register `D_MEAN_FORMAT` reader"]
pub struct R(crate::R<D_MEAN_FORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_MEAN_FORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_MEAN_FORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_MEAN_FORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_MEAN_FORMAT` writer"]
pub struct W(crate::W<D_MEAN_FORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_MEAN_FORMAT_SPEC>;
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
impl From<crate::W<D_MEAN_FORMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_MEAN_FORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEAN_FORMAT_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<MEAN_FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: MEAN_FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEAN_FORMAT` reader - "]
pub struct MEAN_FORMAT_R(crate::FieldReader<bool>);
impl MEAN_FORMAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEAN_FORMAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEAN_FORMAT_A {
        match self.bits {
            true => MEAN_FORMAT_A::ENABLE,
            false => MEAN_FORMAT_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == MEAN_FORMAT_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == MEAN_FORMAT_A::DISABLE
    }
}
impl core::ops::Deref for MEAN_FORMAT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEAN_FORMAT` writer - "]
pub struct MEAN_FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> MEAN_FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEAN_FORMAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MEAN_FORMAT_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MEAN_FORMAT_A::DISABLE)
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mean_format(&self) -> MEAN_FORMAT_R {
        MEAN_FORMAT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mean_format(&mut self) -> MEAN_FORMAT_W {
        MEAN_FORMAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Whether mean registers are used or not\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_mean_format](index.html) module"]
pub struct D_MEAN_FORMAT_SPEC;
impl crate::RegisterSpec for D_MEAN_FORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_mean_format::R](R) reader structure"]
impl crate::Readable for D_MEAN_FORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_mean_format::W](W) writer structure"]
impl crate::Writable for D_MEAN_FORMAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_MEAN_FORMAT to value 0"]
impl crate::Resettable for D_MEAN_FORMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
