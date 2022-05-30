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
pub struct RX_R(crate::FieldReader<bool>);
impl RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX` writer - Rx interrupt in enable flag: - 1'b0: RX IRQ disable - 1'b1: RX IRQ enable"]
pub struct RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_W<'a> {
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
#[doc = "Field `ERROR` reader - Error interrupt in enable flag: - 1'b0: Error IRQ disable - 1'b1: Error IRQ enable"]
pub struct ERROR_R(crate::FieldReader<bool>);
impl ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR` writer - Error interrupt in enable flag: - 1'b0: Error IRQ disable - 1'b1: Error IRQ enable"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
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
    pub fn rx(&mut self) -> RX_W {
        RX_W { w: self }
    }
    #[doc = "Bit 1 - Error interrupt in enable flag: - 1'b0: Error IRQ disable - 1'b1: Error IRQ enable"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
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
}
#[doc = "`reset()` method sets UART_IRQ_EN to value 0"]
impl crate::Resettable for UART_IRQ_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
