#[doc = "Register `force_irq` reader"]
pub struct R(crate::R<FORCE_IRQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FORCE_IRQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FORCE_IRQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FORCE_IRQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `force_irq` writer"]
pub struct W(crate::W<FORCE_IRQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FORCE_IRQ_SPEC>;
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
impl From<crate::W<FORCE_IRQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FORCE_IRQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `force_irq` reader - "]
pub struct FORCE_IRQ_R(crate::FieldReader<u64>);
impl FORCE_IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        FORCE_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_IRQ_R {
    type Target = crate::FieldReader<u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `force_irq` writer - "]
pub struct FORCE_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_IRQ_W<'a> {
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
    pub fn force_irq(&self) -> FORCE_IRQ_R {
        FORCE_IRQ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn force_irq(&mut self) -> FORCE_IRQ_W {
        FORCE_IRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "force interrupt line active; NI\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [force_irq](index.html) module"]
pub struct FORCE_IRQ_SPEC;
impl crate::RegisterSpec for FORCE_IRQ_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [force_irq::R](R) reader structure"]
impl crate::Readable for FORCE_IRQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [force_irq::W](W) writer structure"]
impl crate::Writable for FORCE_IRQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets force_irq to value 0"]
impl crate::Resettable for FORCE_IRQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
