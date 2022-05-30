#[doc = "Register `RESET_HI` reader"]
pub struct R(crate::R<RESET_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_HI` writer"]
pub struct W(crate::W<RESET_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_HI_SPEC>;
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
impl From<crate::W<RESET_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_HI` reader - "]
pub struct RESET_HI_R(crate::FieldReader<bool>);
impl RESET_HI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_HI_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_HI` writer - "]
pub struct RESET_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_HI_W<'a> {
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
    pub fn reset_hi(&self) -> RESET_HI_R {
        RESET_HI_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reset_hi(&mut self) -> RESET_HI_W {
        RESET_HI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Timer High counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_hi](index.html) module"]
pub struct RESET_HI_SPEC;
impl crate::RegisterSpec for RESET_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_hi::R](R) reader structure"]
impl crate::Readable for RESET_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_hi::W](W) writer structure"]
impl crate::Writable for RESET_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESET_HI to value 0"]
impl crate::Resettable for RESET_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
