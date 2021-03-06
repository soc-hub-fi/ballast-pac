#[doc = "Register `tx_pause_en` reader"]
pub struct R(crate::R<TX_PAUSE_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_PAUSE_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_PAUSE_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_PAUSE_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tx_pause_en` writer"]
pub struct W(crate::W<TX_PAUSE_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_PAUSE_EN_SPEC>;
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
impl From<crate::W<TX_PAUSE_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_PAUSE_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_pause_en` reader - When this register is “1”, this core will respond to received pause frame.The transmit state machine will enter PAUSE state according to quanta value in received packet . One quanta time is equal to the time of transmit 512bit data."]
pub struct TX_PAUSE_EN_R(crate::FieldReader<bool>);
impl TX_PAUSE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_PAUSE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PAUSE_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_pause_en` writer - When this register is “1”, this core will respond to received pause frame.The transmit state machine will enter PAUSE state according to quanta value in received packet . One quanta time is equal to the time of transmit 512bit data."]
pub struct TX_PAUSE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PAUSE_EN_W<'a> {
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
    #[doc = "Bit 0 - When this register is “1”, this core will respond to received pause frame.The transmit state machine will enter PAUSE state according to quanta value in received packet . One quanta time is equal to the time of transmit 512bit data."]
    #[inline(always)]
    pub fn tx_pause_en(&self) -> TX_PAUSE_EN_R {
        TX_PAUSE_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When this register is “1”, this core will respond to received pause frame.The transmit state machine will enter PAUSE state according to quanta value in received packet . One quanta time is equal to the time of transmit 512bit data."]
    #[inline(always)]
    pub fn tx_pause_en(&mut self) -> TX_PAUSE_EN_W {
        TX_PAUSE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "When this register is “1”, this core will respond to received pause frame.The transmit state machine will enter PAUSE state according to quanta value in received packet . One quanta time is equal to the time of transmit 512bit data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_pause_en](index.html) module"]
pub struct TX_PAUSE_EN_SPEC;
impl crate::RegisterSpec for TX_PAUSE_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_pause_en::R](R) reader structure"]
impl crate::Readable for TX_PAUSE_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_pause_en::W](W) writer structure"]
impl crate::Writable for TX_PAUSE_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tx_pause_en to value 0"]
impl crate::Resettable for TX_PAUSE_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
