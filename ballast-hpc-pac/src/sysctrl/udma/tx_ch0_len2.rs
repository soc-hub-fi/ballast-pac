#[doc = "Register `TX_CH0_LEN2` reader"]
pub struct R(crate::R<TX_CH0_LEN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CH0_LEN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CH0_LEN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CH0_LEN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CH0_LEN2` writer"]
pub struct W(crate::W<TX_CH0_LEN2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CH0_LEN2_SPEC>;
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
impl From<crate::W<TX_CH0_LEN2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CH0_LEN2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_TX_CH0_LEN2` reader - "]
pub struct REG_TX_CH0_LEN2_R(crate::FieldReader<u32>);
impl REG_TX_CH0_LEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        REG_TX_CH0_LEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_TX_CH0_LEN2_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_TX_CH0_LEN2` writer - "]
pub struct REG_TX_CH0_LEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_TX_CH0_LEN2_W<'a> {
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
    pub fn reg_tx_ch0_len2(&self) -> REG_TX_CH0_LEN2_R {
        REG_TX_CH0_LEN2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_tx_ch0_len2(&mut self) -> REG_TX_CH0_LEN2_W {
        REG_TX_CH0_LEN2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER tx channel 0 length2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ch0_len2](index.html) module"]
pub struct TX_CH0_LEN2_SPEC;
impl crate::RegisterSpec for TX_CH0_LEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ch0_len2::R](R) reader structure"]
impl crate::Readable for TX_CH0_LEN2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ch0_len2::W](W) writer structure"]
impl crate::Writable for TX_CH0_LEN2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CH0_LEN2 to value 0"]
impl crate::Resettable for TX_CH0_LEN2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
