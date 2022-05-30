#[doc = "Register `core0_bootaddr` reader"]
pub struct R(crate::R<CORE0_BOOTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE0_BOOTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE0_BOOTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE0_BOOTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `core0_bootaddr` writer"]
pub struct W(crate::W<CORE0_BOOTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE0_BOOTADDR_SPEC>;
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
impl From<crate::W<CORE0_BOOTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE0_BOOTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `core0_bootaddr` reader - "]
pub struct CORE0_BOOTADDR_R(crate::FieldReader<u64>);
impl CORE0_BOOTADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        CORE0_BOOTADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE0_BOOTADDR_R {
    type Target = crate::FieldReader<u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `core0_bootaddr` writer - "]
pub struct CORE0_BOOTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE0_BOOTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u64) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn core0_bootaddr(&self) -> CORE0_BOOTADDR_R {
        CORE0_BOOTADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn core0_bootaddr(&mut self) -> CORE0_BOOTADDR_W {
        CORE0_BOOTADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core#0 boot address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core0_bootaddr](index.html) module"]
pub struct CORE0_BOOTADDR_SPEC;
impl crate::RegisterSpec for CORE0_BOOTADDR_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [core0_bootaddr::R](R) reader structure"]
impl crate::Readable for CORE0_BOOTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core0_bootaddr::W](W) writer structure"]
impl crate::Writable for CORE0_BOOTADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets core0_bootaddr to value 0"]
impl crate::Resettable for CORE0_BOOTADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
