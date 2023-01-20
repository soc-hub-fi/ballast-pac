#[doc = "Register `INTEN_32_63` reader"]
pub struct R(crate::R<INTEN_32_63_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_32_63_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_32_63_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_32_63_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN_32_63` writer"]
pub struct W(crate::W<INTEN_32_63_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_32_63_SPEC>;
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
impl From<crate::W<INTEN_32_63_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_32_63_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEN` reader - "]
pub type INTEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTEN` writer - "]
pub type INTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTEN_32_63_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<0> {
        INTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit 31 - 0 INTEN (R/W) GPIO\\[63:32\\]
interrupt enable configuration bitfield: - bit\\[i\\]=1b0: disable interrupt for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable interrupt for GPIO\\[i\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten_32_63](index.html) module"]
pub struct INTEN_32_63_SPEC;
impl crate::RegisterSpec for INTEN_32_63_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten_32_63::R](R) reader structure"]
impl crate::Readable for INTEN_32_63_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten_32_63::W](W) writer structure"]
impl crate::Writable for INTEN_32_63_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN_32_63 to value 0"]
impl crate::Resettable for INTEN_32_63_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
