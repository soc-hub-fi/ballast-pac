#[doc = "Register `INTSTATUS` reader"]
pub struct R(crate::R<INTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTATUS` writer"]
pub struct W(crate::W<INTSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTATUS_SPEC>;
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
impl From<crate::W<INTSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTSTATUS` reader - "]
pub struct INTSTATUS_R(crate::FieldReader<u32>);
impl INTSTATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INTSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTSTATUS_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSTATUS` writer - "]
pub struct INTSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSTATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn intstatus(&mut self) -> INTSTATUS_W {
        INTSTATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains interrupt status per GPIO line. The status register is cleared when read. Similarly the interrupt line is high while a bit is set in interrupt status and will be deasserted when the status register is read.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstatus](index.html) module"]
pub struct INTSTATUS_SPEC;
impl crate::RegisterSpec for INTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstatus::R](R) reader structure"]
impl crate::Readable for INTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intstatus::W](W) writer structure"]
impl crate::Writable for INTSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTSTATUS to value 0"]
impl crate::Resettable for INTSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
