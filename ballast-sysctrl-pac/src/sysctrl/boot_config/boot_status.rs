#[doc = "Register `BOOT_STATUS` reader"]
pub struct R(crate::R<BOOT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOT_STATUS` writer"]
pub struct W(crate::W<BOOT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOT_STATUS_SPEC>;
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
impl From<crate::W<BOOT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT_STATUS` reader - "]
pub type BOOT_STATUS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BOOT_STATUS` writer - "]
pub type BOOT_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOT_STATUS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn boot_status(&self) -> BOOT_STATUS_R {
        BOOT_STATUS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn boot_status(&mut self) -> BOOT_STATUS_W<0> {
        BOOT_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot_status](index.html) module"]
pub struct BOOT_STATUS_SPEC;
impl crate::RegisterSpec for BOOT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [boot_status::R](R) reader structure"]
impl crate::Readable for BOOT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [boot_status::W](W) writer structure"]
impl crate::Writable for BOOT_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOT_STATUS to value 0"]
impl crate::Resettable for BOOT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
