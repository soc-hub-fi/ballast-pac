#[doc = "Register `irq` reader"]
pub struct R(crate::R<IRQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irq` writer"]
pub struct W(crate::W<IRQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_SPEC>;
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
impl From<crate::W<IRQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `irq` reader - "]
pub struct IRQ_R(crate::FieldReader<u8>);
impl IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `irq` writer - "]
pub struct IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn irq(&mut self) -> IRQ_W {
        IRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "One bit corresponds to an interrupt line from the MAC/DMA. When an interrupt is raised and its corresponding bit in irq_mask is set, its corresponding bit in irq is set and the external IRQ line is asserted. The IRQ line is deasserted when all bits in the irq register are unset by a write by the processor. In other words, the external IRQ line is an OR-reduction of the irq register. Writes to the irq register cannot assert new interrupt requests; in other words, the value stored in the irq register is the old contents combined with the write value by a bitwise AND.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq](index.html) module"]
pub struct IRQ_SPEC;
impl crate::RegisterSpec for IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq::R](R) reader structure"]
impl crate::Readable for IRQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq::W](W) writer structure"]
impl crate::Writable for IRQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irq to value 0"]
impl crate::Resettable for IRQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
