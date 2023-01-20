#[doc = "Register `UART_IRQ_EN` reader"]
pub struct R(crate::R<UART_IRQ_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_IRQ_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_IRQ_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_IRQ_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_IRQ_EN` writer"]
pub struct W(crate::W<UART_IRQ_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_IRQ_EN_SPEC>;
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
impl From<crate::W<UART_IRQ_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_IRQ_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX` reader - Rx interrupt in enable flag: - 1'b0: RX IRQ disable - 1'b1: RX IRQ enable"]
pub type RX_R = crate::BitReader<bool>;
#[doc = "Field `RX` writer - Rx interrupt in enable flag: - 1'b0: RX IRQ disable - 1'b1: RX IRQ enable"]
pub type RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_IRQ_EN_SPEC, bool, O>;
#[doc = "Field `ERROR` reader - Error interrupt in enable flag: - 1'b0: Error IRQ disable - 1'b1: Error IRQ enable"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - Error interrupt in enable flag: - 1'b0: Error IRQ disable - 1'b1: Error IRQ enable"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_IRQ_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Rx interrupt in enable flag: - 1'b0: RX IRQ disable - 1'b1: RX IRQ enable"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error interrupt in enable flag: - 1'b0: Error IRQ disable - 1'b1: Error IRQ enable"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx interrupt in enable flag: - 1'b0: RX IRQ disable - 1'b1: RX IRQ enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<0> {
        RX_W::new(self)
    }
    #[doc = "Bit 1 - Error interrupt in enable flag: - 1'b0: Error IRQ disable - 1'b1: Error IRQ enable"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<1> {
        ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA UART Read or Error interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_irq_en](index.html) module"]
pub struct UART_IRQ_EN_SPEC;
impl crate::RegisterSpec for UART_IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_irq_en::R](R) reader structure"]
impl crate::Readable for UART_IRQ_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_irq_en::W](W) writer structure"]
impl crate::Writable for UART_IRQ_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_IRQ_EN to value 0"]
impl crate::Resettable for UART_IRQ_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
