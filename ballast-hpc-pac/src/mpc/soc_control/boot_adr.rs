#[doc = "Register `BOOT_ADR` reader"]
pub struct R(crate::R<BOOT_ADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOT_ADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOT_ADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOT_ADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOT_ADR` writer"]
pub struct W(crate::W<BOOT_ADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOT_ADR_SPEC>;
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
impl From<crate::W<BOOT_ADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOT_ADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT_ADR` reader - Note! Reset value"]
pub struct BOOT_ADR_R(crate::FieldReader<u32>);
impl BOOT_ADR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BOOT_ADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_ADR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_ADR` writer - Note! Reset value"]
pub struct BOOT_ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Note! Reset value"]
    #[inline(always)]
    pub fn boot_adr(&self) -> BOOT_ADR_R {
        BOOT_ADR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Note! Reset value"]
    #[inline(always)]
    pub fn boot_adr(&mut self) -> BOOT_ADR_W {
        BOOT_ADR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register holds the boot address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot_adr](index.html) module"]
pub struct BOOT_ADR_SPEC;
impl crate::RegisterSpec for BOOT_ADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [boot_adr::R](R) reader structure"]
impl crate::Readable for BOOT_ADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [boot_adr::W](W) writer structure"]
impl crate::Writable for BOOT_ADR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOOT_ADR to value 0x1a10_0000"]
impl crate::Resettable for BOOT_ADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1a10_0000
    }
}
