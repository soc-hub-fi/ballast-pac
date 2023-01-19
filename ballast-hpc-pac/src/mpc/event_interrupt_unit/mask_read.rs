#[doc = "Register `MASK_read` reader"]
pub struct R(crate::R<MASK_READ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_READ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_READ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_READ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK_read` writer"]
pub struct W(crate::W<MASK_READ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_READ_SPEC>;
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
impl From<crate::W<MASK_READ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_READ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - "]
pub type MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MASK` writer - "]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MASK_READ_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<0> {
        MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected Writing to 0x1A10_9008 clears the bits of the MASK register selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask_read](index.html) module"]
pub struct MASK_READ_SPEC;
impl crate::RegisterSpec for MASK_READ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask_read::R](R) reader structure"]
impl crate::Readable for MASK_READ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask_read::W](W) writer structure"]
impl crate::Writable for MASK_READ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASK_read to value 0"]
impl crate::Resettable for MASK_READ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
