#[doc = "Register `TX_CH1_LEN1` reader"]
pub struct R(crate::R<TX_CH1_LEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CH1_LEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CH1_LEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CH1_LEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CH1_LEN1` writer"]
pub struct W(crate::W<TX_CH1_LEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CH1_LEN1_SPEC>;
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
impl From<crate::W<TX_CH1_LEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CH1_LEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_TX_CH1_LEN1` reader - "]
pub struct REG_TX_CH1_LEN1_R(crate::FieldReader<u32>);
impl REG_TX_CH1_LEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        REG_TX_CH1_LEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_TX_CH1_LEN1_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_TX_CH1_LEN1` writer - "]
pub struct REG_TX_CH1_LEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_TX_CH1_LEN1_W<'a> {
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
    pub fn reg_tx_ch1_len1(&self) -> REG_TX_CH1_LEN1_R {
        REG_TX_CH1_LEN1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_tx_ch1_len1(&mut self) -> REG_TX_CH1_LEN1_W {
        REG_TX_CH1_LEN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER tx channel length2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ch1_len1](index.html) module"]
pub struct TX_CH1_LEN1_SPEC;
impl crate::RegisterSpec for TX_CH1_LEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ch1_len1::R](R) reader structure"]
impl crate::Readable for TX_CH1_LEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ch1_len1::W](W) writer structure"]
impl crate::Writable for TX_CH1_LEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CH1_LEN1 to value 0"]
impl crate::Resettable for TX_CH1_LEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
