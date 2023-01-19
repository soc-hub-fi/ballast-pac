#[doc = "Register `PADDIR_32_63` reader"]
pub struct R(crate::R<PADDIR_32_63_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADDIR_32_63_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADDIR_32_63_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADDIR_32_63_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADDIR_32_63` writer"]
pub struct W(crate::W<PADDIR_32_63_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADDIR_32_63_SPEC>;
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
impl From<crate::W<PADDIR_32_63_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADDIR_32_63_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - "]
pub type DIR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DIR` writer - "]
pub type DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PADDIR_32_63_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<0> {
        DIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit 31 - 0 DIR (R/W) GPIO\\[63:32\\]
direction configuration bitfield: - bit\\[i\\]=1b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1b1: Output mode for GPIO\\[i\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paddir_32_63](index.html) module"]
pub struct PADDIR_32_63_SPEC;
impl crate::RegisterSpec for PADDIR_32_63_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [paddir_32_63::R](R) reader structure"]
impl crate::Readable for PADDIR_32_63_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paddir_32_63::W](W) writer structure"]
impl crate::Writable for PADDIR_32_63_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADDIR_32_63 to value 0"]
impl crate::Resettable for PADDIR_32_63_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
