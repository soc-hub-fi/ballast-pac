#[doc = "Register `irq_0_route_reg_1` reader"]
pub struct R(crate::R<IRQ_0_ROUTE_REG_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_0_ROUTE_REG_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_0_ROUTE_REG_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_0_ROUTE_REG_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irq_0_route_reg_1` writer"]
pub struct W(crate::W<IRQ_0_ROUTE_REG_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_0_ROUTE_REG_1_SPEC>;
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
impl From<crate::W<IRQ_0_ROUTE_REG_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_0_ROUTE_REG_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `irq_0_route_reg_1` reader - "]
pub type IRQ_0_ROUTE_REG_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `irq_0_route_reg_1` writer - "]
pub type IRQ_0_ROUTE_REG_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, IRQ_0_ROUTE_REG_1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn irq_0_route_reg_1(&self) -> IRQ_0_ROUTE_REG_1_R {
        IRQ_0_ROUTE_REG_1_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn irq_0_route_reg_1(&mut self) -> IRQ_0_ROUTE_REG_1_W<0> {
        IRQ_0_ROUTE_REG_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SysCtrl route register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_0_route_reg_1](index.html) module"]
pub struct IRQ_0_ROUTE_REG_1_SPEC;
impl crate::RegisterSpec for IRQ_0_ROUTE_REG_1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [irq_0_route_reg_1::R](R) reader structure"]
impl crate::Readable for IRQ_0_ROUTE_REG_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_0_route_reg_1::W](W) writer structure"]
impl crate::Writable for IRQ_0_ROUTE_REG_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets irq_0_route_reg_1 to value 0"]
impl crate::Resettable for IRQ_0_ROUTE_REG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
