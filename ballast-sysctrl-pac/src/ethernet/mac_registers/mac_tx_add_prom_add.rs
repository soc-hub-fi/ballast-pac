#[doc = "Register `MAC_tx_add_prom_add` reader"]
pub struct R(crate::R<MAC_TX_ADD_PROM_ADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_TX_ADD_PROM_ADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_TX_ADD_PROM_ADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_TX_ADD_PROM_ADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_tx_add_prom_add` writer"]
pub struct W(crate::W<MAC_TX_ADD_PROM_ADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_TX_ADD_PROM_ADD_SPEC>;
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
impl From<crate::W<MAC_TX_ADD_PROM_ADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_TX_ADD_PROM_ADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAC_tx_add_prom_add` reader - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
pub type MAC_TX_ADD_PROM_ADD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAC_tx_add_prom_add` writer - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
pub type MAC_TX_ADD_PROM_ADD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAC_TX_ADD_PROM_ADD_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
    #[inline(always)]
    pub fn mac_tx_add_prom_add(&self) -> MAC_TX_ADD_PROM_ADD_R {
        MAC_TX_ADD_PROM_ADD_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
    #[inline(always)]
    #[must_use]
    pub fn mac_tx_add_prom_add(&mut self) -> MAC_TX_ADD_PROM_ADD_W<0> {
        MAC_TX_ADD_PROM_ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_tx_add_prom_add](index.html) module"]
pub struct MAC_TX_ADD_PROM_ADD_SPEC;
impl crate::RegisterSpec for MAC_TX_ADD_PROM_ADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_tx_add_prom_add::R](R) reader structure"]
impl crate::Readable for MAC_TX_ADD_PROM_ADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_tx_add_prom_add::W](W) writer structure"]
impl crate::Writable for MAC_TX_ADD_PROM_ADD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_tx_add_prom_add to value 0"]
impl crate::Resettable for MAC_TX_ADD_PROM_ADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
