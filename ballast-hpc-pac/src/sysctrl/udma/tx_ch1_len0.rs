#[doc = "Register `TX_CH1_LEN0` reader"]
pub struct R(crate::R<TX_CH1_LEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CH1_LEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CH1_LEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CH1_LEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CH1_LEN0` writer"]
pub struct W(crate::W<TX_CH1_LEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CH1_LEN0_SPEC>;
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
impl From<crate::W<TX_CH1_LEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CH1_LEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_CH1_ADD` reader - "]
pub type TX_CH1_ADD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TX_CH1_ADD` writer - "]
pub type TX_CH1_ADD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_CH1_LEN0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tx_ch1_add(&self) -> TX_CH1_ADD_R {
        TX_CH1_ADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch1_add(&mut self) -> TX_CH1_ADD_W<0> {
        TX_CH1_ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER tx channel length1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ch1_len0](index.html) module"]
pub struct TX_CH1_LEN0_SPEC;
impl crate::RegisterSpec for TX_CH1_LEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ch1_len0::R](R) reader structure"]
impl crate::Readable for TX_CH1_LEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ch1_len0::W](W) writer structure"]
impl crate::Writable for TX_CH1_LEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_CH1_LEN0 to value 0"]
impl crate::Resettable for TX_CH1_LEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
