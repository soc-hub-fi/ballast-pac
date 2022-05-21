#[doc = "Register `MAC_tx_add_en` reader"]
pub struct R(crate::R<MAC_TX_ADD_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_TX_ADD_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_TX_ADD_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_TX_ADD_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_tx_add_en` writer"]
pub struct W(crate::W<MAC_TX_ADD_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_TX_ADD_EN_SPEC>;
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
impl From<crate::W<MAC_TX_ADD_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_TX_ADD_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAC_tx_add_en` reader - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
pub struct MAC_TX_ADD_EN_R(crate::FieldReader<bool, bool>);
impl MAC_TX_ADD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAC_TX_ADD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAC_TX_ADD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAC_tx_add_en` writer - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
pub struct MAC_TX_ADD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_TX_ADD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
    #[inline(always)]
    pub fn mac_tx_add_en(&self) -> MAC_TX_ADD_EN_R {
        MAC_TX_ADD_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
    #[inline(always)]
    pub fn mac_tx_add_en(&mut self) -> MAC_TX_ADD_EN_W {
        MAC_TX_ADD_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_tx_add_en](index.html) module"]
pub struct MAC_TX_ADD_EN_SPEC;
impl crate::RegisterSpec for MAC_TX_ADD_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_tx_add_en::R](R) reader structure"]
impl crate::Readable for MAC_TX_ADD_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_tx_add_en::W](W) writer structure"]
impl crate::Writable for MAC_TX_ADD_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_tx_add_en to value 0"]
impl crate::Resettable for MAC_TX_ADD_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
