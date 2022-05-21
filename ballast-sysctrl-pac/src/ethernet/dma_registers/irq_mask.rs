#[doc = "Register `irq_mask` reader"]
pub struct R(crate::R<IRQ_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irq_mask` writer"]
pub struct W(crate::W<IRQ_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_MASK_SPEC>;
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
impl From<crate::W<IRQ_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `irq_mask` reader - "]
pub struct IRQ_MASK_R(crate::FieldReader<u8, u8>);
impl IRQ_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IRQ_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `irq_mask` writer - "]
pub struct IRQ_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_MASK_W<'a> {
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
    pub fn irq_mask(&self) -> IRQ_MASK_R {
        IRQ_MASK_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn irq_mask(&mut self) -> IRQ_MASK_W {
        IRQ_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_mask](index.html) module"]
pub struct IRQ_MASK_SPEC;
impl crate::RegisterSpec for IRQ_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_mask::R](R) reader structure"]
impl crate::Readable for IRQ_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_mask::W](W) writer structure"]
impl crate::Writable for IRQ_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irq_mask to value 0"]
impl crate::Resettable for IRQ_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
