#[doc = "Register `MAC_rx_add_chk_en` reader"]
pub struct R(crate::R<MAC_RX_ADD_CHK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_RX_ADD_CHK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_RX_ADD_CHK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_RX_ADD_CHK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_rx_add_chk_en` writer"]
pub struct W(crate::W<MAC_RX_ADD_CHK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_RX_ADD_CHK_EN_SPEC>;
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
impl From<crate::W<MAC_RX_ADD_CHK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_RX_ADD_CHK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAC_rx_add_chk_en` reader - "]
pub type MAC_RX_ADD_CHK_EN_R = crate::BitReader<bool>;
#[doc = "Field `MAC_rx_add_chk_en` writer - "]
pub type MAC_RX_ADD_CHK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MAC_RX_ADD_CHK_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mac_rx_add_chk_en(&self) -> MAC_RX_ADD_CHK_EN_R {
        MAC_RX_ADD_CHK_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mac_rx_add_chk_en(&mut self) -> MAC_RX_ADD_CHK_EN_W<0> {
        MAC_RX_ADD_CHK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_rx_add_chk_en](index.html) module"]
pub struct MAC_RX_ADD_CHK_EN_SPEC;
impl crate::RegisterSpec for MAC_RX_ADD_CHK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_rx_add_chk_en::R](R) reader structure"]
impl crate::Readable for MAC_RX_ADD_CHK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_rx_add_chk_en::W](W) writer structure"]
impl crate::Writable for MAC_RX_ADD_CHK_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_rx_add_chk_en to value 0"]
impl crate::Resettable for MAC_RX_ADD_CHK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
