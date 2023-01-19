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
pub type RESET_LO_R = crate::BitReader<bool>;
#[doc = "Field `RESET_LO` writer - "]
pub type RESET_LO_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_LO_SPEC, bool, O>;
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
    #[must_use]
    pub fn reset_lo(&mut self) -> RESET_LO_W<0> {
        RESET_LO_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESET_LO to value 0"]
impl crate::Resettable for RESET_LO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
