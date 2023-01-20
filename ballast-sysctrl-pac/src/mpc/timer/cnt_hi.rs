#[doc = "Register `CNT_HI` reader"]
pub struct R(crate::R<CNT_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT_HI` writer"]
pub struct W(crate::W<CNT_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_HI_SPEC>;
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
impl From<crate::W<CNT_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_HI` reader - "]
pub type CNT_HI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNT_HI` writer - "]
pub type CNT_HI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNT_HI_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cnt_hi(&self) -> CNT_HI_R {
        CNT_HI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_hi(&mut self) -> CNT_HI_W<0> {
        CNT_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer High counter value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt_hi](index.html) module"]
pub struct CNT_HI_SPEC;
impl crate::RegisterSpec for CNT_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt_hi::R](R) reader structure"]
impl crate::Readable for CNT_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt_hi::W](W) writer structure"]
impl crate::Writable for CNT_HI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNT_HI to value 0"]
impl crate::Resettable for CNT_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
