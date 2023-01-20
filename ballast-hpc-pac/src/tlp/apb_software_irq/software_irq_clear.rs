#[doc = "Register `software_irq_clear` writer"]
pub struct W(crate::W<SOFTWARE_IRQ_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFTWARE_IRQ_CLEAR_SPEC>;
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
impl From<crate::W<SOFTWARE_IRQ_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFTWARE_IRQ_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clear` writer - "]
pub type CLEAR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SOFTWARE_IRQ_CLEAR_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<0> {
        CLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [software_irq_clear](index.html) module"]
pub struct SOFTWARE_IRQ_CLEAR_SPEC;
impl crate::RegisterSpec for SOFTWARE_IRQ_CLEAR_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [software_irq_clear::W](W) writer structure"]
impl crate::Writable for SOFTWARE_IRQ_CLEAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets software_irq_clear to value 0"]
impl crate::Resettable for SOFTWARE_IRQ_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
