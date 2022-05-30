#[doc = "Register `CLUSTER_IRQ` reader"]
pub struct R(crate::R<CLUSTER_IRQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLUSTER_IRQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLUSTER_IRQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLUSTER_IRQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLUSTER_IRQ` writer"]
pub struct W(crate::W<CLUSTER_IRQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLUSTER_IRQ_SPEC>;
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
impl From<crate::W<CLUSTER_IRQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLUSTER_IRQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Cluster_IRQ` reader - "]
pub struct CLUSTER_IRQ_R(crate::FieldReader<bool>);
impl CLUSTER_IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLUSTER_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLUSTER_IRQ_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cluster_IRQ` writer - "]
pub struct CLUSTER_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CLUSTER_IRQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cluster_irq(&self) -> CLUSTER_IRQ_R {
        CLUSTER_IRQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cluster_irq(&mut self) -> CLUSTER_IRQ_W {
        CLUSTER_IRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cluster_irq](index.html) module"]
pub struct CLUSTER_IRQ_SPEC;
impl crate::RegisterSpec for CLUSTER_IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cluster_irq::R](R) reader structure"]
impl crate::Readable for CLUSTER_IRQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cluster_irq::W](W) writer structure"]
impl crate::Writable for CLUSTER_IRQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLUSTER_IRQ to value 0"]
impl crate::Resettable for CLUSTER_IRQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
