#[doc = "Register `irq_status` reader"]
pub struct R(crate::R<IRQ_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irq_status` writer"]
pub struct W(crate::W<IRQ_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_STATUS_SPEC>;
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
impl From<crate::W<IRQ_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `irq_status` reader - "]
pub struct IRQ_STATUS_R(crate::FieldReader<u64, u64>);
impl IRQ_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        IRQ_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_STATUS_R {
    type Target = crate::FieldReader<u64, u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `irq_status` writer - "]
pub struct IRQ_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_STATUS_W<'a> {
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
    pub fn irq_status(&self) -> IRQ_STATUS_R {
        IRQ_STATUS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn irq_status(&mut self) -> IRQ_STATUS_W {
        IRQ_STATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt flag statuses; NI\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_status](index.html) module"]
pub struct IRQ_STATUS_SPEC;
impl crate::RegisterSpec for IRQ_STATUS_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [irq_status::R](R) reader structure"]
impl crate::Readable for IRQ_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_status::W](W) writer structure"]
impl crate::Writable for IRQ_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irq_status to value 0"]
impl crate::Resettable for IRQ_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
