#[doc = "Register `T3_CONFIG` reader"]
pub struct R(crate::R<T3_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T3_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T3_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T3_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T3_CONFIG` writer"]
pub struct W(crate::W<T3_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T3_CONFIG_SPEC>;
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
impl From<crate::W<T3_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T3_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSEL` reader - ADV_TIMER3 input source configuration bitfield: - 0-31: GPIO\\[0\\]
to GPIO\\[31\\]
- 32-35: Channel 0 to 3 of ADV_TIMER0 - 36-39: Channel 0 to 3 of ADV_TIMER1 - 40-43: Channel 0 to 3 of ADV_TIMER2 - 44-47: Channel 0 to 3 of ADV_TIMER3"]
pub struct INSEL_R(crate::FieldReader<u8, u8>);
impl INSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSEL` writer - ADV_TIMER3 input source configuration bitfield: - 0-31: GPIO\\[0\\]
to GPIO\\[31\\]
- 32-35: Channel 0 to 3 of ADV_TIMER0 - 36-39: Channel 0 to 3 of ADV_TIMER1 - 40-43: Channel 0 to 3 of ADV_TIMER2 - 44-47: Channel 0 to 3 of ADV_TIMER3"]
pub struct INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `MODE` reader - ADV_TIMER3 trigger mode configuration bitfield: - 3'h0: trigger event at each clock cycle. - 3'h1: trigger event if input source is 0 - 3'h2: trigger event if input source is 1 - 3'h3: trigger event on input source rising edge - 3'h4: trigger event on input source falling edge - 3'h5: trigger event on input source falling or rising edge - 3'h6: trigger event on input source rising edge when armed - 3'h7: trigger event on input source falling edge when armed"]
pub struct MODE_R(crate::FieldReader<u8, u8>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - ADV_TIMER3 trigger mode configuration bitfield: - 3'h0: trigger event at each clock cycle. - 3'h1: trigger event if input source is 0 - 3'h2: trigger event if input source is 1 - 3'h3: trigger event on input source rising edge - 3'h4: trigger event on input source falling edge - 3'h5: trigger event on input source falling or rising edge - 3'h6: trigger event on input source rising edge when armed - 3'h7: trigger event on input source falling edge when armed"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 8)) | ((value as u32 & 7) << 8);
        self.w
    }
}
#[doc = "Field `CLKSEL` reader - ADV_TIMER3 clock source configuration bitfield: - 1'b0: FLL - 1'b1: reference clock at 32kHz"]
pub struct CLKSEL_R(crate::FieldReader<bool, bool>);
impl CLKSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSEL` writer - ADV_TIMER3 clock source configuration bitfield: - 1'b0: FLL - 1'b1: reference clock at 32kHz"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `UPDOWNSEL` reader - ADV_TIMER3 center-aligned mode configuration bitfield: - 1'b0: The counter counts up and down alternatively. - 1'b1: The counter counts up and resets to 0 when reach threshold."]
pub struct UPDOWNSEL_R(crate::FieldReader<bool, bool>);
impl UPDOWNSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPDOWNSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPDOWNSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDOWNSEL` writer - ADV_TIMER3 center-aligned mode configuration bitfield: - 1'b0: The counter counts up and down alternatively. - 1'b1: The counter counts up and resets to 0 when reach threshold."]
pub struct UPDOWNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDOWNSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `PRESC` reader - ADV_TIMER3 prescaler value configuration bitfield."]
pub struct PRESC_R(crate::FieldReader<u8, u8>);
impl PRESC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRESC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESC` writer - ADV_TIMER3 prescaler value configuration bitfield."]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADV_TIMER3 input source configuration bitfield: - 0-31: GPIO\\[0\\]
to GPIO\\[31\\]
- 32-35: Channel 0 to 3 of ADV_TIMER0 - 36-39: Channel 0 to 3 of ADV_TIMER1 - 40-43: Channel 0 to 3 of ADV_TIMER2 - 44-47: Channel 0 to 3 of ADV_TIMER3"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - ADV_TIMER3 trigger mode configuration bitfield: - 3'h0: trigger event at each clock cycle. - 3'h1: trigger event if input source is 0 - 3'h2: trigger event if input source is 1 - 3'h3: trigger event on input source rising edge - 3'h4: trigger event on input source falling edge - 3'h5: trigger event on input source falling or rising edge - 3'h6: trigger event on input source rising edge when armed - 3'h7: trigger event on input source falling edge when armed"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ADV_TIMER3 clock source configuration bitfield: - 1'b0: FLL - 1'b1: reference clock at 32kHz"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADV_TIMER3 center-aligned mode configuration bitfield: - 1'b0: The counter counts up and down alternatively. - 1'b1: The counter counts up and resets to 0 when reach threshold."]
    #[inline(always)]
    pub fn updownsel(&self) -> UPDOWNSEL_R {
        UPDOWNSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - ADV_TIMER3 prescaler value configuration bitfield."]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADV_TIMER3 input source configuration bitfield: - 0-31: GPIO\\[0\\]
to GPIO\\[31\\]
- 32-35: Channel 0 to 3 of ADV_TIMER0 - 36-39: Channel 0 to 3 of ADV_TIMER1 - 40-43: Channel 0 to 3 of ADV_TIMER2 - 44-47: Channel 0 to 3 of ADV_TIMER3"]
    #[inline(always)]
    pub fn insel(&mut self) -> INSEL_W {
        INSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - ADV_TIMER3 trigger mode configuration bitfield: - 3'h0: trigger event at each clock cycle. - 3'h1: trigger event if input source is 0 - 3'h2: trigger event if input source is 1 - 3'h3: trigger event on input source rising edge - 3'h4: trigger event on input source falling edge - 3'h5: trigger event on input source falling or rising edge - 3'h6: trigger event on input source rising edge when armed - 3'h7: trigger event on input source falling edge when armed"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 11 - ADV_TIMER3 clock source configuration bitfield: - 1'b0: FLL - 1'b1: reference clock at 32kHz"]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bit 12 - ADV_TIMER3 center-aligned mode configuration bitfield: - 1'b0: The counter counts up and down alternatively. - 1'b1: The counter counts up and resets to 0 when reach threshold."]
    #[inline(always)]
    pub fn updownsel(&mut self) -> UPDOWNSEL_W {
        UPDOWNSEL_W { w: self }
    }
    #[doc = "Bits 16:23 - ADV_TIMER3 prescaler value configuration bitfield."]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADV_TIMER3 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t3_config](index.html) module"]
pub struct T3_CONFIG_SPEC;
impl crate::RegisterSpec for T3_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t3_config::R](R) reader structure"]
impl crate::Readable for T3_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t3_config::W](W) writer structure"]
impl crate::Writable for T3_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T3_CONFIG to value 0"]
impl crate::Resettable for T3_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
