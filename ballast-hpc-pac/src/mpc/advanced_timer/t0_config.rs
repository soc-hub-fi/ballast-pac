#[doc = "Register `T0_CONFIG` reader"]
pub struct R(crate::R<T0_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0_CONFIG` writer"]
pub struct W(crate::W<T0_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0_CONFIG_SPEC>;
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
impl From<crate::W<T0_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSEL` reader - ADV_TIMER0 input source configuration bitfield: - 0-31: GPIO\\[0\\]
to GPIO\\[31\\]
- 32-35: Channel 0 to 3 of ADV_TIMER0 - 36-39: Channel 0 to 3 of ADV_TIMER1 - 40-43: Channel 0 to 3 of ADV_TIMER2 - 44-47: Channel 0 to 3 of ADV_TIMER3"]
pub type INSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSEL` writer - ADV_TIMER0 input source configuration bitfield: - 0-31: GPIO\\[0\\]
to GPIO\\[31\\]
- 32-35: Channel 0 to 3 of ADV_TIMER0 - 36-39: Channel 0 to 3 of ADV_TIMER1 - 40-43: Channel 0 to 3 of ADV_TIMER2 - 44-47: Channel 0 to 3 of ADV_TIMER3"]
pub type INSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `MODE` reader - ADV_TIMER0 trigger mode configuration bitfield: - 3'h0: trigger event at each clock cycle. - 3'h1: trigger event if input source is 0 - 3'h2: trigger event if input source is 1 - 3'h3: trigger event on input source rising edge - 3'h4: trigger event on input source falling edge - 3'h5: trigger event on input source falling or rising edge - 3'h6: trigger event on input source rising edge when armed - 3'h7: trigger event on input source falling edge when armed"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - ADV_TIMER0 trigger mode configuration bitfield: - 3'h0: trigger event at each clock cycle. - 3'h1: trigger event if input source is 0 - 3'h2: trigger event if input source is 1 - 3'h3: trigger event on input source rising edge - 3'h4: trigger event on input source falling edge - 3'h5: trigger event on input source falling or rising edge - 3'h6: trigger event on input source rising edge when armed - 3'h7: trigger event on input source falling edge when armed"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `CLKSEL` reader - ADV_TIMER0 clock source configuration bitfield: - 1'b0: FLL - 1'b1: reference clock at 32kHz"]
pub type CLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `CLKSEL` writer - ADV_TIMER0 clock source configuration bitfield: - 1'b0: FLL - 1'b1: reference clock at 32kHz"]
pub type CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0_CONFIG_SPEC, bool, O>;
#[doc = "Field `UPDOWNSEL` reader - ADV_TIMER0 center-aligned mode configuration bitfield: - 1'b0: The counter counts up and down alternatively. - 1'b1: The counter counts up and resets to 0 when reach threshold."]
pub type UPDOWNSEL_R = crate::BitReader<bool>;
#[doc = "Field `UPDOWNSEL` writer - ADV_TIMER0 center-aligned mode configuration bitfield: - 1'b0: The counter counts up and down alternatively. - 1'b1: The counter counts up and resets to 0 when reach threshold."]
pub type UPDOWNSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0_CONFIG_SPEC, bool, O>;
#[doc = "Field `PRESC` reader - ADV_TIMER0 prescaler value configuration bitfield."]
pub type PRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESC` writer - ADV_TIMER0 prescaler value configuration bitfield."]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0_CONFIG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ADV_TIMER0 input source configuration bitfield: - 0-31: GPIO\\[0\\]
to GPIO\\[31\\]
- 32-35: Channel 0 to 3 of ADV_TIMER0 - 36-39: Channel 0 to 3 of ADV_TIMER1 - 40-43: Channel 0 to 3 of ADV_TIMER2 - 44-47: Channel 0 to 3 of ADV_TIMER3"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - ADV_TIMER0 trigger mode configuration bitfield: - 3'h0: trigger event at each clock cycle. - 3'h1: trigger event if input source is 0 - 3'h2: trigger event if input source is 1 - 3'h3: trigger event on input source rising edge - 3'h4: trigger event on input source falling edge - 3'h5: trigger event on input source falling or rising edge - 3'h6: trigger event on input source rising edge when armed - 3'h7: trigger event on input source falling edge when armed"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ADV_TIMER0 clock source configuration bitfield: - 1'b0: FLL - 1'b1: reference clock at 32kHz"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADV_TIMER0 center-aligned mode configuration bitfield: - 1'b0: The counter counts up and down alternatively. - 1'b1: The counter counts up and resets to 0 when reach threshold."]
    #[inline(always)]
    pub fn updownsel(&self) -> UPDOWNSEL_R {
        UPDOWNSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - ADV_TIMER0 prescaler value configuration bitfield."]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADV_TIMER0 input source configuration bitfield: - 0-31: GPIO\\[0\\]
to GPIO\\[31\\]
- 32-35: Channel 0 to 3 of ADV_TIMER0 - 36-39: Channel 0 to 3 of ADV_TIMER1 - 40-43: Channel 0 to 3 of ADV_TIMER2 - 44-47: Channel 0 to 3 of ADV_TIMER3"]
    #[inline(always)]
    #[must_use]
    pub fn insel(&mut self) -> INSEL_W<0> {
        INSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - ADV_TIMER0 trigger mode configuration bitfield: - 3'h0: trigger event at each clock cycle. - 3'h1: trigger event if input source is 0 - 3'h2: trigger event if input source is 1 - 3'h3: trigger event on input source rising edge - 3'h4: trigger event on input source falling edge - 3'h5: trigger event on input source falling or rising edge - 3'h6: trigger event on input source rising edge when armed - 3'h7: trigger event on input source falling edge when armed"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<8> {
        MODE_W::new(self)
    }
    #[doc = "Bit 11 - ADV_TIMER0 clock source configuration bitfield: - 1'b0: FLL - 1'b1: reference clock at 32kHz"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<11> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 12 - ADV_TIMER0 center-aligned mode configuration bitfield: - 1'b0: The counter counts up and down alternatively. - 1'b1: The counter counts up and resets to 0 when reach threshold."]
    #[inline(always)]
    #[must_use]
    pub fn updownsel(&mut self) -> UPDOWNSEL_W<12> {
        UPDOWNSEL_W::new(self)
    }
    #[doc = "Bits 16:23 - ADV_TIMER0 prescaler value configuration bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<16> {
        PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADV_TIMER0 configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0_config](index.html) module"]
pub struct T0_CONFIG_SPEC;
impl crate::RegisterSpec for T0_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0_config::R](R) reader structure"]
impl crate::Readable for T0_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0_config::W](W) writer structure"]
impl crate::Writable for T0_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0_CONFIG to value 0"]
impl crate::Resettable for T0_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
