#[doc = "Register `rtc_cfg2` reader"]
pub struct R(crate::R<RTC_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rtc_cfg2` writer"]
pub struct W(crate::W<RTC_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CFG2_SPEC>;
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
impl From<crate::W<RTC_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rtc_cfg2` reader - "]
pub type RTC_CFG2_R = crate::FieldReader<u64, u64>;
#[doc = "Field `rtc_cfg2` writer - "]
pub type RTC_CFG2_W<'a, const O: u8> = crate::FieldWriter<'a, u64, RTC_CFG2_SPEC, u64, u64, 64, O>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn rtc_cfg2(&self) -> RTC_CFG2_R {
        RTC_CFG2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_cfg2(&mut self) -> RTC_CFG2_W<0> {
        RTC_CFG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "real time clock generator clock fine tune\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cfg2](index.html) module"]
pub struct RTC_CFG2_SPEC;
impl crate::RegisterSpec for RTC_CFG2_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [rtc_cfg2::R](R) reader structure"]
impl crate::Readable for RTC_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cfg2::W](W) writer structure"]
impl crate::Writable for RTC_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtc_cfg2 to value 0"]
impl crate::Resettable for RTC_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
