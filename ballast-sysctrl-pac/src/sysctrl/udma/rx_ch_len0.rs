#[doc = "Register `RX_CH_LEN0` reader"]
pub struct R(crate::R<RX_CH_LEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CH_LEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CH_LEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CH_LEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CH_LEN0` writer"]
pub struct W(crate::W<RX_CH_LEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CH_LEN0_SPEC>;
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
impl From<crate::W<RX_CH_LEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CH_LEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_RX_CH_LEN0` reader - "]
pub struct REG_RX_CH_LEN0_R(crate::FieldReader<u32>);
impl REG_RX_CH_LEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        REG_RX_CH_LEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_RX_CH_LEN0_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_RX_CH_LEN0` writer - "]
pub struct REG_RX_CH_LEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_RX_CH_LEN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_rx_ch_len0(&self) -> REG_RX_CH_LEN0_R {
        REG_RX_CH_LEN0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_rx_ch_len0(&mut self) -> REG_RX_CH_LEN0_W {
        REG_RX_CH_LEN0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER RX channel configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ch_len0](index.html) module"]
pub struct RX_CH_LEN0_SPEC;
impl crate::RegisterSpec for RX_CH_LEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_ch_len0::R](R) reader structure"]
impl crate::Readable for RX_CH_LEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_ch_len0::W](W) writer structure"]
impl crate::Writable for RX_CH_LEN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_CH_LEN0 to value 0"]
impl crate::Resettable for RX_CH_LEN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
