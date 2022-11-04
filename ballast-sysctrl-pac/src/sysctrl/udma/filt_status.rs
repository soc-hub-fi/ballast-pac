#[doc = "Register `FILT_STATUS` reader"]
pub struct R(crate::R<FILT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILT_STATUS` writer"]
pub struct W(crate::W<FILT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILT_STATUS_SPEC>;
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
impl From<crate::W<FILT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE` reader - Filter done flag, write 1 to clear the flag : -1'b0: Filter process is not finished -1'b1: Filter process is finished"]
pub struct DONE_R(crate::FieldReader<bool>);
impl DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE` writer - Filter done flag, write 1 to clear the flag : -1'b0: Filter process is not finished -1'b1: Filter process is finished"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
    #[doc = "Bit 0 - Filter done flag, write 1 to clear the flag : -1'b0: Filter process is not finished -1'b1: Filter process is finished"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter done flag, write 1 to clear the flag : -1'b0: Filter process is not finished -1'b1: Filter process is finished"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filt_status](index.html) module"]
pub struct FILT_STATUS_SPEC;
impl crate::RegisterSpec for FILT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filt_status::R](R) reader structure"]
impl crate::Readable for FILT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filt_status::W](W) writer structure"]
impl crate::Writable for FILT_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILT_STATUS to value 0"]
impl crate::Resettable for FILT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
