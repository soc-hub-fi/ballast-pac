#[doc = "Register `RESET_LO` reader"]
pub struct R(crate::R<RESET_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_LO` writer"]
pub struct W(crate::W<RESET_LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_LO_SPEC>;
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
impl From<crate::W<RESET_LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_LO` reader - "]
pub struct RESET_LO_R(crate::FieldReader<bool>);
impl RESET_LO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_LO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_LO` writer - "]
pub struct RESET_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_LO_W<'a> {
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
    pub fn reset_lo(&self) -> RESET_LO_R {
        RESET_LO_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reset_lo(&mut self) -> RESET_LO_W {
        RESET_LO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Timer Low counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_lo](index.html) module"]
pub struct RESET_LO_SPEC;
impl crate::RegisterSpec for RESET_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_lo::R](R) reader structure"]
impl crate::Readable for RESET_LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_lo::W](W) writer structure"]
impl crate::Writable for RESET_LO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESET_LO to value 0"]
impl crate::Resettable for RESET_LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
