#[doc = "Register `CNT_LO` reader"]
pub struct R(crate::R<CNT_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT_LO` writer"]
pub struct W(crate::W<CNT_LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_LO_SPEC>;
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
impl From<crate::W<CNT_LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_LO` reader - "]
pub type CNT_LO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNT_LO` writer - "]
pub type CNT_LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNT_LO_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cnt_lo(&self) -> CNT_LO_R {
        CNT_LO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_lo(&mut self) -> CNT_LO_W<0> {
        CNT_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Low counter value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt_lo](index.html) module"]
pub struct CNT_LO_SPEC;
impl crate::RegisterSpec for CNT_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt_lo::R](R) reader structure"]
impl crate::Readable for CNT_LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt_lo::W](W) writer structure"]
impl crate::Writable for CNT_LO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNT_LO to value 0"]
impl crate::Resettable for CNT_LO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
