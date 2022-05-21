#[doc = "Register `FCBOOT` reader"]
pub struct R(crate::R<FCBOOT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCBOOT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCBOOT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCBOOT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCBOOT` writer"]
pub struct W(crate::W<FCBOOT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCBOOT_SPEC>;
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
impl From<crate::W<FCBOOT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCBOOT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCBOOT` reader - Boot Start address"]
pub struct FCBOOT_R(crate::FieldReader<u32, u32>);
impl FCBOOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FCBOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCBOOT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCBOOT` writer - Boot Start address"]
pub struct FCBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> FCBOOT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Boot Start address"]
    #[inline(always)]
    pub fn fcboot(&self) -> FCBOOT_R {
        FCBOOT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Boot Start address"]
    #[inline(always)]
    pub fn fcboot(&mut self) -> FCBOOT_W {
        FCBOOT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register holds the boot address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcboot](index.html) module"]
pub struct FCBOOT_SPEC;
impl crate::RegisterSpec for FCBOOT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcboot::R](R) reader structure"]
impl crate::Readable for FCBOOT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcboot::W](W) writer structure"]
impl crate::Writable for FCBOOT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCBOOT to value 0x1a00_0080"]
impl crate::Resettable for FCBOOT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1a00_0080
    }
}
