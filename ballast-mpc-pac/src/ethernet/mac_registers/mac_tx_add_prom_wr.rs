#[doc = "Register `MAC_tx_add_prom_wr` reader"]
pub struct R(crate::R<MAC_TX_ADD_PROM_WR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_TX_ADD_PROM_WR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_TX_ADD_PROM_WR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_TX_ADD_PROM_WR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_tx_add_prom_wr` writer"]
pub struct W(crate::W<MAC_TX_ADD_PROM_WR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_TX_ADD_PROM_WR_SPEC>;
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
impl From<crate::W<MAC_TX_ADD_PROM_WR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_TX_ADD_PROM_WR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAC_tx_add_prom_wr` reader - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
pub type MAC_TX_ADD_PROM_WR_R = crate::BitReader<bool>;
#[doc = "Field `MAC_tx_add_prom_wr` writer - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
pub type MAC_TX_ADD_PROM_WR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MAC_TX_ADD_PROM_WR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
    #[inline(always)]
    pub fn mac_tx_add_prom_wr(&self) -> MAC_TX_ADD_PROM_WR_R {
        MAC_TX_ADD_PROM_WR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
    #[inline(always)]
    #[must_use]
    pub fn mac_tx_add_prom_wr(&mut self) -> MAC_TX_ADD_PROM_WR_W<0> {
        MAC_TX_ADD_PROM_WR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_tx_add_prom_wr](index.html) module"]
pub struct MAC_TX_ADD_PROM_WR_SPEC;
impl crate::RegisterSpec for MAC_TX_ADD_PROM_WR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_tx_add_prom_wr::R](R) reader structure"]
impl crate::Readable for MAC_TX_ADD_PROM_WR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_tx_add_prom_wr::W](W) writer structure"]
impl crate::Writable for MAC_TX_ADD_PROM_WR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_tx_add_prom_wr to value 0"]
impl crate::Resettable for MAC_TX_ADD_PROM_WR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
